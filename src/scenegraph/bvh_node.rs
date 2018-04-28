use math::{Bounded, BoundingVolume, Float, Ray};
use scenegraph::Bvh;
use tracing::{Hit, Traceable};

pub trait BvhLeaf<F: Float, B: BoundingVolume<F>>: Bounded<F, B> + Traceable<F> {}

impl<T, F: Float, B: BoundingVolume<F>> BvhLeaf<F, B> for T
where
    T: Bounded<F, B> + Traceable<F>,
{
}

pub enum BvhNode<F: Float, B: BoundingVolume<F>, L: BvhLeaf<F, B>> {
    Node(Bvh<F, B, L>),
    Leaf(L),
}

impl<F: Float, B: BoundingVolume<F>, L: BvhLeaf<F, B>> Bounded<F, B> for BvhNode<F, B, L> {
    fn bounding_volume(&self) -> B {
        match self {
            BvhNode::Node(n) => n.bounding_volume(),
            BvhNode::Leaf(n) => n.bounding_volume(),
        }
    }
}

impl<F: Float, B: BoundingVolume<F>, L: BvhLeaf<F, B>> Traceable<F> for BvhNode<F, B, L> {
    fn trace(&self, ray: &Ray<F>) -> Option<Hit<F>> {
        match self {
            BvhNode::Node(n) => n.trace(ray),
            BvhNode::Leaf(n) => n.trace(ray),
        }
    }
}
