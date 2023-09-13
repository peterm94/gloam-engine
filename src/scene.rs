use std::collections::HashMap;

use js_sys::{Array, Function, JsString};
use wasm_bindgen::prelude::*;

use crate::game::{DEL_OBJECTS, Gloam, ID_COUNT, JsGameObject, log, NEXT_OBJECTS, OBJECTS, OBJECTS_INDEX};

#[allow(dead_code)]
#[derive(Default)]
pub struct ObjectsIndex {
    names: HashMap<String, usize>,
    types: HashMap<String, Box<Vec<usize>>>,
    tags: HashMap<String, Box<Vec<usize>>>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "(object: JsGameObject) => void")]
    pub type WithObjFn;

    #[wasm_bindgen(typescript_type = "(objects: [JsGameObject]) => void")]
    pub type WithObjsFn;

    #[wasm_bindgen(typescript_type = "number[]")]
    pub type JsNumArray;
}


#[wasm_bindgen]
impl Gloam {
    pub fn update_once(delta: f64) {
        Gloam::update(delta);
    }

    pub fn update(delta: f64) {
        unsafe {
            DEL_OBJECTS.drain(..).for_each(|x| {
                OBJECTS.with(|objects| {
                    objects.borrow_mut().remove(&x);
                })
            });


            OBJECTS.with(|objects| {
                for (_, object) in objects.borrow().iter() {
                    object.update(delta);
                }
            });

            if !NEXT_OBJECTS.is_empty() {
                // move the pending additions out of the static so it doesn't cause problems with init()
                let mut temp = vec![];
                temp.append(&mut NEXT_OBJECTS);

                // init each object
                temp.iter().for_each(|(_, x)| x.init());

                // Put them in the global object map
                OBJECTS.with(|objects| {
                    let mut objects = objects.borrow_mut();
                    temp.into_iter().for_each(|(k, v)| {
                        objects.insert(k, v);
                    });
                });
            }
        }
    }


    pub fn add_object(js_object: JsGameObject) -> usize {
        let name = Gloam::get_js_obj_name(&js_object);
        log(&format!("name {name}"));
        unsafe {
            let id = ID_COUNT;
            ID_COUNT += 1;
            NEXT_OBJECTS.push((id, js_object));

            Gloam::add_type(name, id);
            id
        }
    }

    pub fn destroy_object(id: usize) {
        unsafe { DEL_OBJECTS.push(id) };
    }

    fn get_js_obj_name(x: &JsValue) -> String {
        let proto = js_sys::Object::get_prototype_of(x);
        let constructor = proto.constructor();
        constructor.name().as_string().unwrap()
    }

    fn add_type(name: String, id: usize) {
        OBJECTS_INDEX.with(|index| {
            let mut index = index.borrow_mut();
            if let Some(inner) = index.types.get_mut(&name) {
                inner.push(id);
            } else {
                index.types.insert(name, Box::new(vec![id]));
            }
        });
    }

    pub fn with_type(type_name: &JsString, f: &WithObjFn) {
        let f = JsValue::from(f).unchecked_into::<Function>();
        let name: String = type_name.into();
        OBJECTS_INDEX.with(|index| {
            if let Some(ids) = index.borrow().types.get(&name) {
                OBJECTS.with(|objects| {
                    let objects = objects.borrow();
                    for id in ids.iter() {
                        let this = JsValue::null();
                        if let Some(obj) = objects.get(&id) {
                            f.call1(&this, obj).unwrap();
                        }
                    }
                });
            }
        })
    }


    pub fn with_object(id: usize, f: &WithObjFn) {
        let f = JsValue::from(f).unchecked_into::<Function>();

        OBJECTS.with(|objects| {
            let this = JsValue::null();
            let objects = objects.borrow();
            if let Some(obj) = objects.get(&id) {
                f.call1(&this, obj).unwrap();
            }
        });
    }

    pub fn with_objects(ids: JsNumArray, f: &WithObjsFn) {
        let ids = JsValue::from(ids).unchecked_into::<Array>();
        let f = JsValue::from(f).unchecked_into::<Function>();

        let this_ref = JsValue::null();

        OBJECTS.with(|objects| {
            let objects = objects.borrow();

            let args = Array::new();

            for object_id in ids.iter() {
                let id = JsValue::from(object_id).as_f64().unwrap() as usize;

                if let Some(found) = objects.get(&id) {
                    args.push(found);
                } else {
                    return;
                }
            }
            f.call1(&this_ref, &args).unwrap();
            // f.apply(&this_ref, &args).unwrap();
        });
    }

    pub fn find_objs_with_type(_type_name: &JsString) -> Vec<usize> {
        unimplemented!()
    }

    pub fn find_obj_with_type(_type_name: &JsString) -> usize {
        unimplemented!()
    }
}