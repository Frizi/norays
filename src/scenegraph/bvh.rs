use math::{Bounded, BoundingVolume, BoundingVolumeSum, Float, Ray};
use scenegraph::{BvhLeaf, BvhNode};
use shading::Material;
use std::cmp::Ordering::Equal;
use std::marker::PhantomData;
use tracing::{Hitable, Traceable};

pub struct Bvh<F, B, H, M, L>
where
    F: Float,
    B: BoundingVolume<F>,
    H: Hitable<F, Material = M>,
    M: Material<F>,
    L: BvhLeaf<F, B, H, M>,
{
    bound: B,
    children: Vec<BvhNode<F, B, H, M, L>>,
    _f: PhantomData<F>,
    _h: PhantomData<H>,
    _m: PhantomData<M>,
}

impl<F, B, H, M, L> Bvh<F, B, H, M, L>
where
    F: Float,
    B: BoundingVolume<F>,
    H: Hitable<F, Material = M>,
    M: Material<F>,
    L: BvhLeaf<F, B, H, M>,
{
    pub fn from_nodes(children: Vec<BvhNode<F, B, H, M, L>>) -> Option<Self> {
        children
            .iter()
            .map(|node| node.bounding_volume())
            .bounding_sum()
            .map(|bound| Self {
                bound,
                children,
                _f: PhantomData,
                _h: PhantomData,
                _m: PhantomData,
            })
    }
}

impl<F, B, H, M, L> Bounded<F, B> for Bvh<F, B, H, M, L>
where
    F: Float,
    B: BoundingVolume<F>,
    H: Hitable<F, Material = M>,
    M: Material<F>,
    L: BvhLeaf<F, B, H, M>,
{
    fn bounding_volume(&self) -> B {
        self.bound
    }
}

impl<F, B, H, M, L> Traceable<F, H, M> for Bvh<F, B, H, M, L>
where
    F: Float,
    B: BoundingVolume<F>,
    H: Hitable<F, Material = M>,
    M: Material<F>,
    L: BvhLeaf<F, B, H, M>,
{
    fn trace(&self, ray: &Ray<F>) -> Option<(F, &H)> {
        if !&self.bounding_volume().test(ray) {
            None
        } else {
            self.children
                .iter()
                .filter_map(|node| node.trace(&ray))
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Equal))
        }
    }
}
