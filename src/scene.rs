use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;
use wasm_bindgen::prelude::*;

use crate::game::log;
use crate::GameState;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "GameObject")]
    pub type GameObject;

    #[wasm_bindgen(structural, method)]
    pub fn update(this: &GameObject, delta: f32);

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
    transform: Rc<RefCell<Transform>>,
    object: Option<GameObject>,
}

#[derive(Default)]
pub struct Transform {
    pub pos_x: f32,
    pub pos_y: f32,
    pub g_pos_x: f32,
    pub g_pos_y: f32,
    pub dirty: bool,
}

impl Transform {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { dirty: true, ..Self::default() }))
    }
}

pub struct GloamGraph {
    graph: Graph<GNode, ()>,
    root: NodeIndex,
}

impl GloamGraph {
    pub fn new() -> Self {
        let mut graph = Graph::<GNode, ()>::new();

        let root = GNode { id: 0, transform: Transform::new(), object: None };
        let root_idx = graph.add_node(root);

        Self { graph, root: root_idx }
    }

    // pub fn get_transform(&self, object_id: usize) -> Option<&Transform> {
    //     if let Some((_, node)) = self.graph.node_references().find(|(_, node)| node.id == object_id) {
    //         return Some(&node.transform);
    //     }
    //     None
    // }

    pub fn add(&mut self, parent_id: usize, child: GameObject, transform: Rc<RefCell<Transform>>) {
        let child_id = child.id();

        let new_node = GNode { id: child_id, transform, object: Some(child) };
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
pub struct Scene {
    graph: GloamGraph,
    state: Rc<RefCell<GameState>>,

    add_objects: Vec<(usize, GameObject, Rc<RefCell<Transform>>)>,
    del_objects: Vec<usize>,
}

impl Scene {
    pub fn new(state: Rc<RefCell<GameState>>) -> Self {
        Self {
            graph: GloamGraph::new(),
            state,
            add_objects: vec![],
            del_objects: vec![],
        }
    }
}

#[wasm_bindgen]
impl Scene {
    pub fn update(&mut self, delta: f32) {
        if !self.state.borrow().del_objects.is_empty() {
            mem::swap(&mut self.del_objects, &mut self.state.borrow_mut().del_objects);
            self.del_objects.drain(..).for_each(|x| self.graph.remove(x));
        }

        self.graph.graph.node_references().for_each(|(_, node)| {
            if let Some(object) = &node.object {
                object.update(delta);
            }
        });

        // TODO do I need to init in a separate loop?
        if !self.state.borrow().add_objects.is_empty() {
            mem::swap(&mut self.add_objects, &mut self.state.borrow_mut().add_objects);
            self.add_objects.drain(..).for_each(|(parent, child, transform)| {
                child.init();
                self.graph.add(parent, child, transform);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use petgraph::algo::toposort;
    use super::*;

    #[test]
    fn test_order() {
        let mut graph: Graph<&str, usize> = Graph::new();
        let n_root = graph.add_node("root");
        let n_1 = graph.add_node("1");
        let n_3 = graph.add_node("3");
        let n_2 = graph.add_node("2");
        let n_4 = graph.add_node("4");
        let n_7 = graph.add_node("7");
        let n_5 = graph.add_node("5");
        let n_10 = graph.add_node("10");
        let n_9 = graph.add_node("9");

        graph.add_edge(n_root, n_1, 1);
        graph.add_edge(n_root, n_3, 3);
        graph.add_edge(n_root, n_2, 2);
        graph.add_edge(n_root, n_4, 4);
        graph.add_edge(n_root, n_7, 7);
        graph.add_edge(n_root, n_5, 5);
        graph.add_edge(n_root, n_10, 10);
        graph.add_edge(n_root, n_9, 9);

        let mut bfs = Bfs::new(&graph, n_root);
        while let Some(nx) = bfs.next(&graph) {
            println!("{} - {}", nx.index(), graph[nx])
        }

        println!("for");
        graph.node_references().for_each(|(_, x)|println!("{}", x));
        println!("topo");

        for x in toposort(&graph, None).unwrap() {
            println!("{} - {}", x.index(), graph[x])
        }
    }
}