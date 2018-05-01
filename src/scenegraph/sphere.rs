use math::{Aabb, Bounded, BoundingVolume, Float, Point, Ray, Vector};
use shading::Material;
use tracing::{HitPoint, Hitable, Traceable};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere<F: Float> {
    center: Point<F>,
    radius: F,
    radius_sq: F,
}

impl<F: Float> Sphere<F> {
    pub fn new(center: Point<F>, radius: F) -> Self {
        Sphere {
            center,
            radius,
            radius_sq: radius * radius,
        }
    }

    pub fn normal_at(&self, point: Point<F>) -> Vector<F> {
        (point - self.center).normalized()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ShadedSphere<F: Float, M: Material<F>> {
    inner: Sphere<F>,
    material: M,
}

impl<F: Float, M: Material<F>> ShadedSphere<F, M> {
    pub fn new(center: Point<F>, radius: F, material: M) -> Self {
        ShadedSphere {
            inner: Sphere::new(center, radius),
            material,
        }
    }
}

impl<F: Float, M: Material<F>> Hitable<F> for ShadedSphere<F, M> {
    type Material = M;
    fn get_hit(&self, ray: &Ray<F>, distance: F) -> HitPoint<F, Self::Material> {
        let point = ray.point_at_distance(distance);

        HitPoint::new(
            point,
            self.inner.normal_at(point),
            ray.direction,
            Default::default(),
            self.material.clone(),
        )
    }
}

impl<F, M> Traceable<F, Self, M> for ShadedSphere<F, M>
where
    F: Float,
    M: Material<F>,
{
    fn trace(&self, ray: &Ray<F>) -> Option<(F, &Self)> {
        let oc = self.inner.center - ray.origin;
        let closest_tangent_dist = oc.dot(ray.direction);

        if closest_tangent_dist < F::zero() {
            // sphere behind ray
            return None;
        };

        let closest_on_tangent = ray.point_at_distance(closest_tangent_dist);
        let sq_distance_to_tangent = (self.inner.center - closest_on_tangent).magnitude_sq();

        if sq_distance_to_tangent > self.inner.radius_sq {
            return None;
        };

        let dist_to_radius_diff = (self.inner.radius_sq - sq_distance_to_tangent).sqrt();
        let distance = closest_tangent_dist - dist_to_radius_diff;

        Some((distance, &self))
    }
}

impl<F: Float> BoundingVolume<F> for Sphere<F> {
    fn combine(&self, rhs: &Self) -> Self {
        let half = (F::one() + F::one()).recip();

        let disp = rhs.center - self.center;
        let distance = disp.magnitude();

        let new_diameter = distance + self.radius + rhs.radius;
        let new_radius = new_diameter * half;

        let new_center = self.center + disp * distance.recip() * (new_radius - self.radius);

        Sphere::new(new_center, new_radius)
    }

    fn estimated_volume(&self) -> F {
        let four_over_three =
            (F::one() + F::one() + F::one() + F::one()) / (F::one() + F::one() + F::one());
        four_over_three * self.radius * self.radius * self.radius * F::PI()
    }

    fn test(&self, ray: &Ray<F>) -> bool {
        let oc = self.center - ray.origin;
        let closest_tangent_dist = oc.dot(ray.direction);

        if closest_tangent_dist < F::zero() {
            return false;
        };

        let closest_on_tangent = ray.point_at_distance(closest_tangent_dist);
        let sq_distance_to_tangent = (self.center - closest_on_tangent).magnitude_sq();
        return sq_distance_to_tangent <= self.radius_sq;
    }
}

impl<F: Float> Bounded<F, Sphere<F>> for Sphere<F> {
    fn bounding_volume(&self) -> Self {
        self.clone()
    }
}

impl<F, M, B> Bounded<F, B> for ShadedSphere<F, M>
where
    F: Float,
    M: Material<F>,
    B: BoundingVolume<F>,
    Sphere<F>: Bounded<F, B>,
{
    fn bounding_volume(&self) -> B {
        self.inner.bounding_volume()
    }
}

impl<F: Float> Bounded<F, Aabb<F>> for Sphere<F> {
    fn bounding_volume(&self) -> Aabb<F> {
        Aabb {
            x_min: self.center.x - self.radius,
            x_max: self.center.x + self.radius,
            y_min: self.center.y - self.radius,
            y_max: self.center.y + self.radius,
            z_min: self.center.z - self.radius,
            z_max: self.center.z + self.radius,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sphere_combine() {
        let s1: Sphere<f64> = Sphere::new(Point::new(-3.0, -3.0, 0.0), 1.0);
        let s2: Sphere<f64> = Sphere::new(Point::new(4.0, -3.0, 0.0), 2.0);

        let expected = Sphere::new(Point::new(1.0, -3.0, 0.0), 5.0);

        assert_eq!(s1.combine(&s2), expected)
    }
}
