use std::cell::RefCell;
use std::collections::HashMap;

use js_sys::{Array, ArrayBuffer, Function, JsString, Uint8Array};
use macroquad::prelude::{draw_texture, ImageFormat, Texture2D, WHITE};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};
use web_sys::console::log_1;

#[wasm_bindgen(typescript_custom_section)]
const SCRIPT: &'static str = r#"
export interface JsGameObject {
    init(): void;
    update(delta: number): void;
}

export function load_game(onReady : () => void): void;
"#;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "JsGameObject")]
    pub type JsGameObject;

    #[wasm_bindgen(structural, method)]
    pub fn update(this: &JsGameObject, delta: f64);

    #[wasm_bindgen(structural, method)]
    pub fn init(this: &JsGameObject);
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

pub static mut ID_COUNT: usize = 1;
pub static mut NEXT_OBJECTS: Vec<(usize, JsGameObject)> = vec![];

thread_local! {
pub static OBJECTS: RefCell<HashMap<usize, JsGameObject>> = RefCell::new(HashMap::new());
pub static OBJECTS_INDEX: RefCell<ObjectsIndex> = RefCell::new(ObjectsIndex::default());
}

pub static mut TEXTURES: Vec<Texture2D> = vec![];
pub static mut DEL_OBJECTS: Vec<usize> = vec![];

#[allow(dead_code)]
#[derive(Default)]
pub struct ObjectsIndex {
    names: HashMap<String, usize>,
    types: HashMap<String, Box<Vec<usize>>>,
    tags: HashMap<String, Box<Vec<usize>>>,
}


#[wasm_bindgen]
pub struct Gloam;

fn log(str: &String) {
    log_1(&str.into());
}

#[wasm_bindgen]
impl Gloam {
    pub fn update_once(delta: f64) {
        Gloam::update(delta);
    }

    pub fn draw(tex_id: usize, x: f32, y: f32) {
        let tex = unsafe { TEXTURES.get(tex_id) };
        if let Some(tex) = tex {
            draw_texture(*tex, x, y, WHITE);
        }
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

    pub async fn load_texture(img_url: &str) -> usize {
        let request = Request::new_with_str(img_url).unwrap();

        let window = web_sys::window().unwrap();
        let resp: Response = JsFuture::from(window.fetch_with_request(&request)).await.unwrap().dyn_into().unwrap();

        let image_data: ArrayBuffer = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap().dyn_into().unwrap();
        let data_array: Vec<u8> = Uint8Array::new(&image_data).to_vec();

        let tex = Texture2D::from_file_with_format(data_array.as_slice(), Some(ImageFormat::Png));

        unsafe { TEXTURES.push(tex); }
        unsafe { return TEXTURES.len() - 1; }
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

    pub fn destroy_object(id: usize) {
        unsafe { DEL_OBJECTS.push(id) };
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

    pub fn find_objs_with_type(_type_name: &JsString) -> Vec<usize> {
        unimplemented!()
    }

    pub fn find_obj_with_type(_type_name: &JsString) -> usize {
        unimplemented!()
    }

    fn get_js_obj_name(x: &JsValue) -> String {
        let proto = js_sys::Object::get_prototype_of(x);
        let constructor = proto.constructor();
        constructor.name().as_string().unwrap()
    }
}
