use std::cell::RefCell;

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

struct GNode {
    id: usize,
    transform: Transform,
    object: Option<GameObject>,
}

#[wasm_bindgen]
#[derive(Default)]
struct Transform {
    pos_x: f32,
    pos_y: f32,
    g_pos_x: f32,
    g_pos_y: f32,
}

pub struct Graph {
    tree: Tree<GNode>,
}

thread_local! {
pub static SCENE_GRAPH: RefCell<Graph> = RefCell::new(Graph::new());
}
static mut DEL_OBJECTS: Vec<GameObject> = vec![];
static mut ADD_OBJECTS: Vec<(usize, GameObject)> = vec![];

impl Graph {
    pub fn new() -> Self {
        Self {
            tree: Tree::new(GNode { id: 0, transform: Transform::default(), object: None }),
        }
    }

    pub fn add(&mut self, parent_id: usize, child: GameObject) {
        let child_id = child.id();

        let new_node = Tree::new(GNode { id: child_id, transform: Transform::default(), object: Some(child) });
        if parent_id == 0 {
            self.tree.push_back(new_node);
            log(&format!("adding {parent_id} -> {child_id}"));
            return;
        }

        let mut node = &self.tree;

         if let Some(node) = self.tree.bfs_children_mut().iter.find(|node| node.data.id == parent_id) {

            // node.push_back(new_node);
            log(&format!("adding {parent_id} -> {child_id}"));
        }
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
                    // graph.objects.remove(&x.id());
                });
            })
        }

        SCENE_GRAPH.with(|graph| {
            let graph = graph.borrow();
            for obj in graph.tree.bfs_children().iter
            {
                log(&format!("{}", obj.data.id));
                if let Some(object) = &obj.data.object {
                    object.update(delta);
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = Tree::new(0);
        let mut tree2 = Tree::new(3);
        tree.push_back(Tree::new(1));
        tree2.push_back(Tree::new(2));
        tree.push_back(tree2);

        tree.bfs_children().iter.for_each(|x| println!("{}", x.data))
    }
}