// bvh.rs
// BVH node structure
// Stephen Marz
// 15 Dec 2020

use crate::hitable::{Hitable, HitList, HitRecord};
use crate::bounding_box::AxisAlignedBoundingBox;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::random::random_int;
use std::sync::Arc;

#[derive(Clone)]
pub struct BvhNode {
    left: Arc<dyn Hitable + Send + Sync>,
    right: Arc<dyn Hitable + Send + Sync>,
    bbox: AxisAlignedBoundingBox
}

unsafe impl Send for BvhNode {}
unsafe impl Sync for BvhNode {}

impl BvhNode {
    pub fn new(list: &mut HitList, time0: f64, time1: f64) -> Self {
        Self::new_slice(list, 0, list.len(), time0, time1)
    }
    #[allow(unused_assignments)]
    pub fn new_slice(list: &mut HitList, start: usize, end: usize, time0: f64, time1: f64) -> Self {
        let objects = list.objects_mut();

        let axis = random_int(0,2);
        let comparator = match axis {
                                0 => box_x_compare,
                                1 => box_y_compare,
                                _ => box_z_compare
                            };

        let object_span = end - start;
        let mut left = objects[start].clone();
        let mut right = objects[start].clone();

        if object_span == 2 {
            if comparator(objects[start].clone(), objects[start+1].clone()) {
                left = objects[start].clone();
                right = objects[start+1].clone();
            } else {
                left = objects[start+1].clone();
                right = objects[start].clone();
            }
        } 
        else {
            // std::sort(objects.begin() + start, objects.begin() + end, comparator);
            let mid = start + object_span/2;
            left = Arc::new(BvhNode::new_slice(list, start, mid, time0, time1));
            right = Arc::new(BvhNode::new_slice(list, mid, end, time0, time1));
        }

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);

        if box_left.is_none() || box_right.is_none() {
            panic!("No bounding box in bvh_node constructor.\n");
        }

        let box_left = box_left.unwrap();
        let box_right = box_right.unwrap();
        let bbox = box_left.surrounding_box(&box_right);

        Self {
            left,
            right,
            bbox
        }
    }
}

impl Hitable for BvhNode {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AxisAlignedBoundingBox> {
        Some(self.bbox.clone())
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let is_hit = self.bbox.hit(ray, t_min, t_max);
        if !is_hit {
            None
        }
        else {
            let hit_left = self.left.hit(ray, t_min, t_max);

            if let Some(hl) = hit_left {
                // In here hit_left hit, so now we check the right.
                if let Some(hr) = self.right.hit(ray, t_min, hl.t()) {
                    Some(hr)
                }
                else {
                    None
                }
            }
            else {
                // If we get here, hit_left evaluated to None
                if let Some(hr) = self.right.hit(ray, t_min, t_max) {
                    Some(hr)
                }
                else {
                    None
                }
            }
        }
    }
}

fn box_compare(a: Arc<dyn Hitable>, b: Arc<dyn Hitable>, axis: usize) -> bool {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    if box_a.is_none() || box_b.is_none() {
        panic!("Invalid bounding box in BVH node.");
    }

    let box_a = box_a.unwrap();
    let box_b = box_b.unwrap();

    box_a.min()[axis] < box_b.min()[axis]
}

fn box_x_compare(a: Arc<dyn Hitable>, b: Arc<dyn Hitable>) -> bool {
    box_compare(a, b, 0)
}

fn box_y_compare(a: Arc<dyn Hitable>, b: Arc<dyn Hitable>) -> bool {
    box_compare(a, b, 1)
}

fn box_z_compare(a: Arc<dyn Hitable>, b: Arc<dyn Hitable>) -> bool {
    box_compare(a, b, 2)
}




