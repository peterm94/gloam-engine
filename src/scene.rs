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

    #[wasm_bindgen(typescript_type = "SceneWrapper")]
    pub type SceneWrapper;

    #[wasm_bindgen(structural, method)]
    pub fn update(this: &SceneWrapper, delta: f32);
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
pub struct NextFrame {
    add_objects: Vec<(usize, GameObject)>,
    del_objects: Vec<usize>,
}

#[wasm_bindgen]
impl NextFrame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { add_objects: vec![], del_objects: vec![] }
    }

    pub fn add_child(&mut self, parent_id: usize, child: GameObject) {
        self.add_objects.push((parent_id, child));
    }

    pub fn remove_object(&mut self, object_id: usize) {
        self.del_objects.push(object_id);
    }
}

#[wasm_bindgen]
pub struct Scene {
    graph: GloamGraph,
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { graph: GloamGraph::new() }
    }
}

#[wasm_bindgen]
impl Scene {
    pub fn update(&mut self, mut next_frame: NextFrame, delta: f64) {
        if !next_frame.del_objects.is_empty() {
            next_frame.del_objects.drain(..).for_each(|x| self.graph.remove(x));
        }

        self.graph.graph.node_references().for_each(|(_, node)| {
            if let Some(object) = &node.object {
                object.update(delta);
            }
        });

        if !next_frame.add_objects.is_empty() {

            // TODO check the order of this.
            next_frame.add_objects.iter().for_each(|(_, x)| x.init());
            // Insert in the graph
            next_frame.add_objects.drain(..).for_each(|(parent_id, child)| {
                self.graph.add(parent_id, child);
            });
        }
    }
}
