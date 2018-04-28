use math::{Aabb, Bounded, BoundingVolume, Float, Point, Ray};
use tracing::{Hit, Traceable};

#[derive(Clone, PartialEq, Debug)]
pub struct Sphere<F: Float> {
    pub center: Point<F>,
    pub radius: F,
    pub radius_sq: F,
}

impl<F: Float> Sphere<F> {
    pub fn new(center: Point<F>, radius: F) -> Self {
        Sphere {
            center,
            radius,
            radius_sq: radius * radius,
        }
    }
}

impl<F: Float> Traceable<F> for Sphere<F>
where
    F: ::std::fmt::Debug,
{
    fn trace(&self, ray: &Ray<F>) -> Option<Hit<F>> {
        let oc = &self.center - &ray.origin;
        let closest_tangent_dist = oc.dot(&ray.direction);

        if closest_tangent_dist < F::zero() {
            // sphere behind ray
            return None;
        };

        let closest_on_tangent = ray.point_at_distance(closest_tangent_dist);
        let sq_distance_to_tangent = (&self.center - &closest_on_tangent).magnitude_sq();

        if sq_distance_to_tangent > self.radius_sq {
            return None;
        };

        let dist_to_radius_diff = (self.radius_sq - sq_distance_to_tangent).sqrt();

        let distance = closest_tangent_dist - dist_to_radius_diff;

        let intersection = ray.point_at_distance(distance);

        let normal = (intersection - &self.center).normalized();

        Some(Hit { distance, normal })
    }
}

impl<F: Float> BoundingVolume<F> for Sphere<F> {
    fn combine(&self, rhs: &Self) -> Self {
        let half = (F::one() + F::one()).recip();

        let disp = &rhs.center - &self.center;
        let distance = disp.magnitude();

        let new_diameter = distance + self.radius + rhs.radius;
        let new_radius = new_diameter * half;

        let new_center = &self.center + disp * distance.recip() * (new_radius - self.radius);

        Sphere::new(new_center, new_radius)
    }

    fn estimated_volume(&self) -> F {
        let four_over_three =
            (F::one() + F::one() + F::one() + F::one()) / (F::one() + F::one() + F::one());
        four_over_three * self.radius * self.radius * self.radius * F::PI()
    }

    fn test(&self, ray: &Ray<F>) -> bool {
        let oc = &self.center - &ray.origin;
        let closest_tangent_dist = oc.dot(&ray.direction);

        if closest_tangent_dist < F::zero() {
            return false;
        };

        let closest_on_tangent = ray.point_at_distance(closest_tangent_dist);
        let sq_distance_to_tangent = (&self.center - &closest_on_tangent).magnitude_sq();
        return sq_distance_to_tangent <= self.radius_sq;
    }
}

impl<F: Float> Bounded<F, Sphere<F>> for Sphere<F> {
    fn bounding_volume(&self) -> Self {
        self.clone()
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
