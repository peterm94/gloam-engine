use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use js_sys::{ArrayBuffer, Uint8Array};
use macroquad::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};
use web_sys::console::log_1;

use crate::{COLL_GRAPH, CURRENT_SCENE, GAME_OPTIONS, GameState, STARTED};
use crate::collisions::Shape;
use crate::events::EventSub;
use crate::scene::{GameObject, Scene, Transform};

#[wasm_bindgen(typescript_custom_section)]
const SCRIPT: &'static str = r#"
export function load_game(onReady: () => void): void;
"#;

thread_local! {
pub static EVENTS: RefCell<HashMap<String, Vec<EventSub>>> = RefCell::new(HashMap::new());
}

pub static mut TEXTURES: Vec<Texture2D> = vec![];

#[wasm_bindgen]
pub struct GameOptions {
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    pub background_colour: u32,
}

#[wasm_bindgen]
impl GameOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, background_colour: u32) -> Self {
        Self { width, height, scale: 1, background_colour }
    }
}

#[wasm_bindgen]
pub struct Gloam {}

pub fn log(str: &String) {
    log_1(&str.into());
}

#[wasm_bindgen]
impl GloamWasm {
    pub fn add_child(&mut self, parent_id: usize, child: GameObject) -> TransformWasm {
        let transform = Transform::new();

        self.state.borrow_mut().add_objects.push((parent_id, child, transform.clone()));
        return TransformWasm { transform };
    }

    pub fn add_object(&mut self, object: GameObject) -> TransformWasm {
        self.add_child(0, object)
    }

    pub fn remove_object(&mut self, object_id: usize) {
        self.state.borrow_mut().del_objects.push(object_id);
    }

    pub fn add_collider(&mut self, x: f32, y: f32, w: f32, h: f32) -> ColliderWasm {
        let shape = Shape::new(x, y, w, h);
        let collider = Rc::new(RefCell::new(0_usize));
        self.state.borrow_mut().add_colliders.push((shape, collider.clone()));

        return ColliderWasm { collider };
    }

    pub fn move_collider(&mut self, id: usize, x: f32, y: f32, w: f32, h: f32) {
        self.state.borrow_mut().move_colliders.push((id, Shape::new(x, y, w, h)));
    }

    pub fn collisions_for(&self, node_id: usize) -> Vec<usize> {
        let state = &self.state.borrow();
        let f1 = state.collisions_this_frame.iter().filter(|x| x.0 == node_id).map(|x| x.1);
        let f2 = state.collisions_this_frame.iter().filter(|x| x.1 == node_id).map(|x| x.0);
        return f1.chain(f2).collect();
    }
    pub fn check_collision(&self, node_1: usize, node_2: usize) -> bool {
        self.state.borrow().collisions_this_frame.contains(&(node_1, node_2)) || self.state.borrow().collisions_this_frame.contains(&(node_2, node_1))
    }
}

#[wasm_bindgen]
pub struct TransformWasm {
    transform: Rc<RefCell<Transform>>,
}

#[wasm_bindgen]
pub struct ColliderWasm {
    collider: Rc<RefCell<usize>>,
}

#[wasm_bindgen]
impl ColliderWasm {
    pub fn id(&self) -> usize {
        return self.collider.borrow().clone();
    }
}

#[wasm_bindgen]
pub struct GloamWasm {
    state: Rc<RefCell<GameState>>,
}

#[wasm_bindgen]
impl Gloam {
    pub fn start(game_options: GameOptions) -> GloamWasm {
        log(&"this is the when start() is called".to_string());

        let game_state = Rc::new(RefCell::new(GameState::default()));
        let scene = Scene::new(game_state.clone());
        let coll_graph = scene.coll_graph.clone();
        unsafe { CURRENT_SCENE = Some(scene); }
        unsafe { COLL_GRAPH = Some(coll_graph); }
        unsafe { GAME_OPTIONS = game_options; }
        unsafe { STARTED = true; }
        return GloamWasm { state: game_state };
    }

    pub async fn load_texture(img_url: &str) -> usize {
        let request = Request::new_with_str(img_url).unwrap();

        let window = web_sys::window().unwrap();
        let resp: Response = JsFuture::from(window.fetch_with_request(&request)).await.unwrap().dyn_into().unwrap();

        let image_data: ArrayBuffer = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap().dyn_into().unwrap();
        let data_array: Vec<u8> = Uint8Array::new(&image_data).to_vec();

        let tex = Texture2D::from_file_with_format(data_array.as_slice(), Some(ImageFormat::Png));

        // We want perfect pixel scaling
        tex.set_filter(FilterMode::Nearest);

        unsafe { TEXTURES.push(tex); }
        unsafe { return TEXTURES.len() - 1; }
    }

    pub fn texture_info(tex_id: usize) -> TextureInfo {
        let tex = unsafe { TEXTURES.get(tex_id) };
        if let Some(tex) = tex {
            return TextureInfo { width: tex.width(), height: tex.height() };
        }
        // TODO can I error or something?
        return TextureInfo { width: 0.0, height: 0.0 };
    }
}

#[wasm_bindgen]
pub struct TextureInfo {
    pub width: f32,
    pub height: f32,
}