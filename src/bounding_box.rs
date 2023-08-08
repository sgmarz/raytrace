// bounding_box.rs
// Axis-aligned bounding boxes (AABBs)
// Stephen Marz
// 15 Dec 2020

use crate::vector::Vec3;
use crate::ray::Ray;

#[derive(Default, Copy, Clone)]
pub struct AxisAlignedBoundingBox {
    minimum: Vec3,
    maximum: Vec3,
}

impl AxisAlignedBoundingBox {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Self {
        Self {
            minimum,
            maximum
        }
    }

    pub fn min(&self) -> &Vec3 {
        &self.minimum
    }

    pub fn max(&self) -> &Vec3 {
        &self.maximum
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min_so_far = t_min;
        let mut t_max_so_far = t_max;
        for a in 0..3 {
            let mino = (self.minimum[a] - ray.origin()[a]) / ray.direction()[a];
            let maxo = (self.maximum[a] - ray.origin()[a]) / ray.direction()[a];

            let t0 = mino.min(maxo);
            let t1 = mino.max(maxo);

            t_min_so_far = t0.max(t_min_so_far);
            t_max_so_far = t1.min(t_max_so_far);

        }
        t_max > t_min
    }
    pub fn surrounding_box(&self, other: &AxisAlignedBoundingBox) -> AxisAlignedBoundingBox {
        let small = Vec3::new(self.min().x().min(other.min().x()),
                 self.min().y().min(other.min().y()),
                 self.min().z().min(other.min().z()));

        let big = Vec3::new(self.max().x().max(other.max().x()),
               self.max().y().max(other.max().y()),
               self.max().z().max(other.max().z()));

        AxisAlignedBoundingBox::new(small,big)
    }
}

