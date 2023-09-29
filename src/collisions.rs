use cgmath::{Point2, Vector2};
use collision::{Aabb, Aabb2};
use collision::dbvt::TreeValue;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{COLLISION_GRAPH, UPDATED_COLLS};

#[wasm_bindgen]
pub struct Collider {
    node_index: usize,
    w: f32,
    h: f32,
}

#[wasm_bindgen]
impl Collider {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        let shape = Shape::new(x, y, x + w, y + h);
        let node_index = unsafe {
            if let Some(graph) = &mut COLLISION_GRAPH {
                graph.insert(shape)
            } else {
                panic!("bad");
            }
        };
        unsafe { UPDATED_COLLS.push(true) };
        Self { node_index, w, h }
    }

    pub fn translate(&mut self, new_x: f32, new_y: f32) {
        let new_shape = Shape::new(new_x, new_y, self.w, self.h);
        // TODO borrow issues here
        unsafe {
            if let Some(graph) = &mut COLLISION_GRAPH {
                graph.update_node(self.node_index, new_shape);
            }
        }

        // Mark node as dirty in our other tracker.
        unsafe { UPDATED_COLLS[self.node_index] = true; }
    }
}

#[derive(Debug, Clone)]
pub struct Shape {
    aabb: Aabb2<f32>,
    fat_aabb: Aabb2<f32>,
}

impl Shape {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        let aabb = aabb2(x, y, x + w, x + h);
        Self { aabb, fat_aabb: aabb.add_margin(Vector2::new(0., 0.)) }
    }
}

impl TreeValue for Shape {
    type Bound = Aabb2<f32>;

    fn bound(&self) -> &Self::Bound {
        &self.aabb
    }

    fn get_bound_with_margin(&self) -> Self::Bound {
        self.fat_aabb
    }
}

fn aabb2(minx: f32, miny: f32, maxx: f32, maxy: f32) -> Aabb2<f32> {
    Aabb2::new(Point2::new(minx, miny), Point2::new(maxx, maxy))
}

#[cfg(test)]
mod tests {
    use collision::algorithm::broad_phase::DbvtBroadPhase;
    use collision::dbvt::DynamicBoundingVolumeTree;

    use super::*;

    #[test]
    fn it_works() {
        let mut tree = DynamicBoundingVolumeTree::new();

        let node_index = tree.insert(Shape::new(0., 0., 12., 10.));
        tree.insert(Shape::new(0., 0., 10., 10.));
        tree.insert(Shape::new(11., 0., 14., 10.));

        tree.tick();

        // use tree.flag_updated() if something moves

        let mut phaser = DbvtBroadPhase::new();
        // only one of the dirty flags needs to be set for it to count for collisions
        let pairs = phaser.find_collider_pairs(&tree, &[true, true, true]);

        for pair in &pairs {
            // todo get details on the collision event?
        }
        println!("{pairs:?}");
    }
}