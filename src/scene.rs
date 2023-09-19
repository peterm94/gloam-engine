use std::cell::RefCell;

use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;
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

struct GNode {
    id: usize,
    transform: Transform,
    object: Option<GameObject>,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Transform {
    pub pos_x: f32,
    pub pos_y: f32,
    pub g_pos_x: f32,
    pub g_pos_y: f32,
}

#[wasm_bindgen]
impl Transform {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct GloamGraph {
    graph: Graph<GNode, ()>,
    root: NodeIndex,
}

thread_local! {
pub static SCENE_GRAPH: RefCell<GloamGraph> = RefCell::new(GloamGraph::new());
}
static mut DEL_OBJECTS: Vec<usize> = vec![];
static mut ADD_OBJECTS: Vec<(usize, GameObject)> = vec![];

impl GloamGraph {
    pub fn new() -> Self {
        let mut graph = Graph::<GNode, ()>::new();

        let root = GNode { id: 0, transform: Transform::default(), object: None };
        let root_idx = graph.add_node(root);

        Self { graph, root: root_idx }
    }

    pub fn get_transform(&self, object_id: usize) -> Option<&Transform> {
        if let Some((_, node)) = self.graph.node_references().find(|(_, node)| node.id == object_id) {
            return Some(&node.transform);
        }
        None
    }

    pub fn add(&mut self, parent_id: usize, child: GameObject) {
        let child_id = child.id();

        let new_node = GNode { id: child_id, transform: Transform::default(), object: Some(child) };
        let new_node_idx = self.graph.add_node(new_node);
        let mut bfs = Bfs::new(&self.graph, self.root);
        while let Some(nx) = bfs.next(&self.graph) {
            if self.graph[nx].id == parent_id {
                self.graph.add_edge(nx, new_node_idx, ());
            }
            return;
        }
        log(&format!("could not find parent with id {parent_id}, object not inserted."));
    }

    pub fn remove(&mut self, object_id: usize) {
        let mut bfs = Bfs::new(&self.graph, self.root);
        while let Some(nx) = bfs.next(&self.graph) {
            if self.graph[nx].id == object_id {
                self.graph.remove_node(nx);
                return;
            }
        }
        log(&format!("could not find node with id {object_id}, object not deleted."));
    }
}

#[wasm_bindgen]
pub struct Scene;

#[wasm_bindgen]
impl Scene {
    pub fn add_child(parent_id: usize, child: GameObject) {
        unsafe { ADD_OBJECTS.push((parent_id, child)); }
    }

    pub fn remove_object(object_id: usize) {
        unsafe { DEL_OBJECTS.push(object_id); }
    }
}

impl Scene {
    pub fn update(delta: f64) {
        unsafe {
            if !DEL_OBJECTS.is_empty() {
                let mut temp = vec![];
                temp.append(&mut DEL_OBJECTS);

                SCENE_GRAPH.with(|graph| {
                    let mut graph = graph.borrow_mut();
                    temp.into_iter().for_each(|x| {
                        graph.remove(x);
                    });
                });
            }
        }

        SCENE_GRAPH.with(|graph| {
            let graph = graph.borrow();
            graph.graph.node_references().for_each(|(idx, node)| {
                if let Some(object) = &node.object {
                    object.update(delta);
                }
            });
        });

        unsafe {
            if !ADD_OBJECTS.is_empty() {
                // TODO we can do some mem swapping probably to avoid a copy and alloc each time
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