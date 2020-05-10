//! # BVH
//!
//! A library for bounding volume hierarchy.

use super::{Axis, Float, HitRecord, Hittable, Ray, RcHittable, RcRandomizer, AABB};
use std::fmt;
use std::rc::Rc;

/// Models a node in a bounding volume hierarchy.
pub struct BVH {
    /// Left child. Leaf nodes would be any Hittable other than a BVH node.
    left: RcHittable,

    /// Right child. Leaf nodes would be any Hittable other than a BVH node.
    right: RcHittable,

    /// Indicates `left` == `right`. This helps avoid using `Option<RcHittable>`
    /// for `left` and `right` and simplify the `split()` function.
    leaf: bool,

    /// Bounding box for objects in the current node.
    bbox: Option<AABB>,
}

impl BVH {
    /// Create a new bounding volume hierarchy.
    ///
    /// Notes:
    /// * This function will panic if any object doesn't have a bounding box.
    /// * The objects list will be re-ordered.
    ///
    /// * `objects` - List of objects
    /// * `time0` - Start time of motion.
    /// * `time1` - End time of motion.
    /// * `rng` - Random number generator.
    pub fn new(
        objects: &mut Vec<RcHittable>,
        time0: Float,
        time1: Float,
        rng: RcRandomizer,
    ) -> RcHittable {
        split(objects, 0, objects.len(), time0, time1, Rc::clone(&rng))
    }
}

impl fmt::Display for BVH {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}]", self)
    }
}

impl fmt::Debug for BVH {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BVH")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("bbox", &self.bbox)
            .finish()
    }
}

impl Hittable for BVH {
    /// Calculate the intersection of a ray with the object.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        // If the ray doesn't hit the bounding volume at this level, terminate
        // search against subtree.
        if self.bbox.is_none() || !self.bbox.unwrap().hit(ray, t_min, t_max) {
            None
        } else if self.leaf {
            self.left.hit(ray, t_min, t_max)
        } else if let Some(left_hit) = self.left.hit(ray, t_min, t_max) {
            // Check if we get a hit on the right subtree that is closer
            // than left_hit.t.
            if let Some(right_hit) = self.right.hit(ray, t_min, left_hit.t) {
                Some(right_hit)
            } else {
                Some(left_hit)
            }
        } else {
            // Try right subtree.
            self.right.hit(ray, t_min, t_max)
        }
    }

    /// Create a bounding box across time interval `[t0, t1]`.
    /// If no bounding box exists return None. This is meant for objects
    /// like an infinite plane.
    ///
    /// * `_time0` - Start time of motion (Ignored).
    /// * `_time1` - End time of motion (Ignored).
    ///
    /// The time parameters are ignored since the BVH node's bounding box
    /// will already include it based on bounding boxes of the objects in it.
    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        self.bbox
    }
}

/// Split a list of objects into a bounding volume hierarchy.
///
/// __Notes:__
/// * The list of objects gets re-ordered in this algorithm.
/// * If bounding box is not is not returned for any object,
///   function will panic. This is fine since its built before the
///   raytracing algorithm is executed.
///
/// * `objects` - List of objects.
/// * `start` - Starting index in `objects`.
/// * `n` - Number of objects to split.
/// * `time0` - Start time of motion.
/// * `time1` - End time of motion.
/// * `rng` - Random number generator.
fn split(
    objects: &mut Vec<RcHittable>,
    start: usize,
    n: usize,
    time0: Float,
    time1: Float,
    rng: RcRandomizer,
) -> RcHittable {
    let axis = rng.clone().float_in_range(0.0, 2.0).round() as Axis;

    let (left, right, leaf) = if n == 1 {
        (Rc::clone(&objects[start]), Rc::clone(&objects[start]), true)
    } else {
        let end = start + n - 1;
        let slice = &mut objects[start..=end];

        slice.sort_unstable_by(|a, b| {
            let bbox_a = a.bounding_box(time0, time1);
            let bbox_b = b.bounding_box(time0, time1);

            match (bbox_a, bbox_b) {
                (Some(bbox_a), Some(bbox_b)) => {
                    let m1 = bbox_a.min[axis];
                    let m2 = bbox_b.min[axis];
                    if m1 < m2 {
                        std::cmp::Ordering::Less
                    } else if m1 > m2 {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                }

                _ => panic!("No objects in BVH::split"),
            }
        });

        if n == 2 {
            (
                Rc::clone(&objects[start]),
                Rc::clone(&objects[start + 1]),
                false,
            )
        } else {
            let half = n / 2;
            let even = n % 2 == 0;
            let half2 = if even { half } else { half + 1 };

            let l = split(objects, start, half, time0, time1, Rc::clone(&rng));
            let r = split(objects, start + half, half2, time0, time1, Rc::clone(&rng));
            (l, r, false)
        }
    };

    let bbox = if leaf {
        left.bounding_box(time0, time1)
    } else {
        match (
            left.bounding_box(time0, time1),
            right.bounding_box(time0, time1),
        ) {
            (Some(bbox_left), Some(bbox_right)) => {
                Some(AABB::surrounding_box(bbox_left, bbox_right))
            }
            _ => panic!("No objects in BVH::split"),
        }
    };

    Rc::new(BVH {
        left,
        right,
        bbox,
        leaf,
    })
}
