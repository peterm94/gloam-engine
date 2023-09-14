use std::cell::RefCell;
use std::collections::HashMap;

use trees::Tree;
use wasm_bindgen::prelude::*;

use crate::game::log;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "GameObject")]
    pub type GameObject;

    #[wasm_bindgen(structural, method)]
    pub fn update(this: &GameObject, delta: f64);

    #[wasm_bindgen(structural, method)]
    pub fn init(this: &GameObject);

    #[wasm_bindgen(structural, method)]
    pub fn id(this: &GameObject) -> usize;
}

struct Node {
    id: usize,
    transform: Transform,
}

#[derive(Default)]
struct Transform {
    pos_x: f32,
    pos_y: f32,
    g_pos_x: f32,
    g_pos_y: f32,
}

pub struct Graph {
    objects: HashMap<usize, GameObject>,
    tree: Tree<Node>,
}

thread_local! {
pub static SCENE_GRAPH: RefCell<Graph> = RefCell::new(Graph::new());
}
static mut DEL_OBJECTS: Vec<GameObject> = vec![];
static mut ADD_OBJECTS: Vec<(usize, GameObject)> = vec![];

impl Graph {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            tree: Tree::new(Node { id: 0, transform: Transform::default() }),
        }
    }

    pub fn add(&mut self, parent_id: usize, child: GameObject) {
        let child_id = child.id();
        self.tree.iter_mut().find(|node| node.data().id == parent_id)
            .and_then(|node| {
                node.get_mut().push_back(Tree::new(Node { id: child_id, transform: Transform::default() }));
                None::<Node>
            });

        self.objects.insert(child_id, child);
        log(&format!("adding {parent_id} -> {child_id}"));
    }
}

#[wasm_bindgen]
pub struct Scene;

#[wasm_bindgen]
impl Scene {
    pub fn add_child(parent_id: usize, child: GameObject) {
        unsafe { ADD_OBJECTS.push((parent_id, child)); }
    }

    pub fn remove_object(object: GameObject) {
        unsafe { DEL_OBJECTS.push(object); }
    }
}

impl Scene {
    pub fn update(delta: f64) {
        unsafe {
            DEL_OBJECTS.drain(..).for_each(|x| {
                SCENE_GRAPH.with(|graph| {
                    let mut graph = graph.borrow_mut();
                    // graph.graph.remove(&x.id());
                    // TODO get parent and fix that up
                    // if let Some(inner) = graph.graph.get()
                    graph.objects.remove(&x.id());
                });
            })
        }

        SCENE_GRAPH.with(|graph| {
            let graph = graph.borrow();
            for obj in graph.objects.values() {
                obj.update(delta);
            }
        });

        unsafe {
            if !ADD_OBJECTS.is_empty() {
                let mut temp = vec![];
                temp.append(&mut ADD_OBJECTS);

                // Initialize
                temp.iter().for_each(|(_, x)| { x.init() });

                // Insert in the graph
                SCENE_GRAPH.with(|graph| {
                    let mut graph = graph.borrow_mut();
                    temp.into_iter().for_each(|(parent_id, child)| {
                        graph.add(parent_id, child);
                    });
                });
            }
        }
    }
}