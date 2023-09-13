use std::cell::RefCell;
use std::collections::HashMap;

use js_sys::{ArrayBuffer, Uint8Array};
use macroquad::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};
use web_sys::console::log_1;

use crate::events::EventSub;
use crate::scene::ObjectsIndex;

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

pub static mut ID_COUNT: usize = 1;
pub static mut NEXT_OBJECTS: Vec<(usize, JsGameObject)> = vec![];

thread_local! {
pub static OBJECTS: RefCell<HashMap<usize, JsGameObject>> = RefCell::new(HashMap::new());
pub static OBJECTS_INDEX: RefCell<ObjectsIndex> = RefCell::new(ObjectsIndex::default());
pub static EVENTS: RefCell<HashMap<String, Vec<EventSub>>> = RefCell::new(HashMap::new());
}

pub static mut TEXTURES: Vec<Texture2D> = vec![];
pub static mut DEL_OBJECTS: Vec<usize> = vec![];


#[wasm_bindgen]
pub struct Gloam {}

pub fn log(str: &String) {
    log_1(&str.into());
}

#[wasm_bindgen]
impl Gloam {
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
}
