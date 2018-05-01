use math::{BoundingVolume, Float, Point, Ray};

#[derive(Clone, Copy)]
pub struct Aabb<F: Float> {
    pub x_min: F,
    pub x_max: F,
    pub y_min: F,
    pub y_max: F,
    pub z_min: F,
    pub z_max: F,
}

impl<F: Float> Aabb<F> {
    pub fn unbound() -> Self {
        Self {
            x_min: F::min_value(),
            x_max: F::max_value(),
            y_min: F::min_value(),
            y_max: F::max_value(),
            z_min: F::min_value(),
            z_max: F::max_value(),
        }
    }

    pub fn from_points(points: &Vec<Point<F>>) -> Self {
        let mut aabb = Aabb::unbound();
        for p in points {
            aabb.x_max = F::max(p.x, aabb.x_max);
            aabb.x_min = F::min(p.x, aabb.x_min);
            aabb.y_max = F::max(p.y, aabb.y_max);
            aabb.y_min = F::min(p.y, aabb.y_min);
            aabb.z_max = F::max(p.z, aabb.z_max);
            aabb.z_min = F::min(p.z, aabb.z_min);
        }
        aabb
    }
}

impl<F: Float> BoundingVolume<F> for Aabb<F> {
    fn combine(&self, other: &Self) -> Self {
        Self {
            x_max: F::max(self.x_max, other.x_max),
            x_min: F::min(self.x_min, other.x_min),
            y_max: F::max(self.y_max, other.y_max),
            y_min: F::min(self.y_min, other.y_min),
            z_max: F::max(self.z_max, other.z_max),
            z_min: F::min(self.z_min, other.z_min),
        }
    }

    fn estimated_volume(&self) -> F {
        let dx = self.x_max - self.x_min;
        let dy = self.y_max - self.y_min;
        let dz = self.z_max - self.z_min;
        dx * dy * dz
    }

    fn test(&self, ray: &Ray<F>) -> bool {
        // TODO: branchless guards for NaN, Infinity and subnormals
        let tx1: F = (self.x_min - ray.origin.x) * ray.inv_direction.x;
        let tx2: F = (self.x_max - ray.origin.x) * ray.inv_direction.x;

        let ty1 = (self.y_min - ray.origin.y) * ray.inv_direction.y;
        let ty2 = (self.y_max - ray.origin.y) * ray.inv_direction.y;

        let tz1 = (self.z_min - ray.origin.z) * ray.inv_direction.z;
        let tz2 = (self.z_max - ray.origin.z) * ray.inv_direction.z;

        let tmin = F::max(F::max(F::min(tx1, tx2), F::min(ty1, ty2)), F::min(tz1, tz2));
        let tmax = F::min(F::min(F::max(tx1, tx2), F::max(ty1, ty2)), F::max(tz1, tz2));

        return tmax >= tmin;
    }
}
