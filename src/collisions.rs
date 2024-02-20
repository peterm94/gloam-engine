use cgmath::{Basis2, Decomposed, Vector2, Rotation2};
use collision::{Aabb, Aabb2};
use collision::dbvt::TreeValue;
use collision::prelude::*;
use collision::primitive::{Circle, Primitive2, Rectangle};
use wasm_bindgen::prelude::wasm_bindgen;


// #[wasm_bindgen]
// impl Collider {
    // #[wasm_bindgen(constructor)]
    // pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
    //     let shape = Shape::new(x, y, x + w, y + h);
    //     let node_index = unsafe {
    //         if let Some(graph) = &mut COLLISION_GRAPH {
    //             graph.insert(shape)
    //         } else {
    //             panic!("bad");
    //         }
    //     };
    //     unsafe { UPDATED_COLLS.push(true) };
    //     Self { node_index, w, h }
    // }
    //
    // pub fn translate(&mut self, new_x: f32, new_y: f32) {
    //     let new_shape = Shape::new(new_x, new_y, self.w, self.h);
    //     // TODO borrow issues here
    //     unsafe {
    //         if let Some(graph) = &mut COLLISION_GRAPH {
    //             graph.update_node(self.node_index, new_shape);
    //         }
    //     }
    //
    //     // Mark node as dirty in our other tracker.
    //     unsafe { UPDATED_COLLS[self.node_index] = true; }
    // }
// }
struct Collisions;
impl Collisions {
    pub fn translate(&mut self, )
}

#[derive(Debug, Clone)]
pub struct Collider {
    pub aabb: Aabb2<f32>,
    fat_aabb: Aabb2<f32>,
    inner: Primitive2<f32>,
    transform: Decomposed<Vector2<f32>, Basis2<f32>>
}

impl Collider {
    pub fn circle(x: f32, y: f32, r: f32) -> Self {
        let circle = Circle::new(r);
        let aabb = circle.compute_bound();
        Self {
            aabb,
            fat_aabb: aabb.add_margin(Vector2::new(0., 0.)),
            inner: Primitive2::Circle(circle),
            transform: Self::pos(x, y)
        }
    }

    pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Self {
        let rect = Rectangle::new(w, h);
        let aabb = rect.compute_bound();

        Self {
            aabb,
            fat_aabb: aabb.add_margin(Vector2::new(0., 0.)),
            inner: Primitive2::Rectangle(rect),
            transform: Self::pos(x, y)
        }
    }
    fn pos(x: f32, y:f32) -> Decomposed<Vector2<f32>, Basis2<f32>> {
        Decomposed {
            disp: Vector2::new(x, y),
            rot: Rotation2::from_angle(0.0),
            scale: 1.0
        }
    }
}

impl TreeValue for Collider {
    type Bound = Aabb2<f32>;

    fn bound(&self) -> &Self::Bound {
        &self.aabb
    }

    fn get_bound_with_margin(&self) -> Self::Bound {
        self.fat_aabb
    }
}

// pub fn aabb2(minx: f32, miny: f32, maxx: f32, maxy: f32) -> Aabb2<f32> {
//     Aabb2::new(Point2::new(minx, miny), Point2::new(maxx, maxy))
// }

#[cfg(test)]
mod tests {
    use cgmath::{Basis2, Decomposed, Rad, Rotation2};
    use collision::algorithm::broad_phase::DbvtBroadPhase;
    use collision::algorithm::minkowski::GJK2;
    use collision::dbvt::DynamicBoundingVolumeTree;

    use super::*;

    #[test]
    fn it_works() {
        let mut tree = DynamicBoundingVolumeTree::new();

        let node_index = tree.insert(Collider::new(0., 0., 12., 10.));
        tree.insert(Collider::new(0., 0., 10., 10.));
        tree.insert(Collider::new(11., 11., 14., 10.));

        tree.tick();

        let mut phaser = DbvtBroadPhase::new();
        // only one of the dirty flags needs to be set for it to count for collisions
        let pairs = phaser.find_collider_pairs(&tree, &[true, true, true]);

        let values = tree.values();
        println!("{pairs:?}");

        let gjk = GJK2::new();

        // TODO use this as the internal transform type for physics objects?
        let transform: Decomposed<Vector2<f32>, Basis2<f32>> = Decomposed {
            disp: Vector2::new(0.0, 0.0),
            rot: Rotation2::from_angle(Rad(0.0)),
            scale: 1.,
        };

        for (sh1, sh2) in pairs {
            let (_, shape1) = &values[sh1];
            let (_, shape2) = &values[sh2];

            let result = gjk.intersect(&shape1.inner, &transform, &shape2.inner, &transform);
            println!("{result:?}");

            // todo get details on the collision event?
        }
    }
}