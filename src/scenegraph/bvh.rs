use math::{Bounded, BoundingVolume, BoundingVolumeSum, Float, Ray};
use scenegraph::{BvhLeaf, BvhNode};
use std::cmp::Ordering::Equal;
use std::marker::PhantomData;
use tracing::{Hit, Traceable};

pub struct Bvh<F: Float, B: BoundingVolume<F>, L: BvhLeaf<F, B>> {
    bound: B,
    children: Vec<BvhNode<F, B, L>>,
    _float: PhantomData<F>,
}

impl<F: Float, B: BoundingVolume<F>, L: BvhLeaf<F, B>> Bvh<F, B, L> {
    pub fn from_nodes(children: Vec<BvhNode<F, B, L>>) -> Option<Self> {
        children
            .iter()
            .map(|node| node.bounding_volume())
            .bounding_sum()
            .map(|bound| Self {
                bound,
                children,
                _float: PhantomData,
            })
    }
}

impl<F: Float, B: BoundingVolume<F>, L: BvhLeaf<F, B>> Bounded<F, B> for Bvh<F, B, L> {
    fn bounding_volume(&self) -> B {
        self.bound.clone()
    }
}

impl<F: Float, B: BoundingVolume<F>, L: BvhLeaf<F, B>> Traceable<F> for Bvh<F, B, L> {
    fn trace(&self, ray: &Ray<F>) -> Option<Hit<F>> {
        if !&self.bounding_volume().test(ray) {
            None
        } else {
            self.children
                .iter()
                .filter_map(|node| node.trace(&ray))
                .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(Equal))
        }
    }
}
