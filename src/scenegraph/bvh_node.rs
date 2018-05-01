use math::{Bounded, BoundingVolume, Float, Ray};
use scenegraph::Bvh;
use shading::Material;
use tracing::{Hitable, Traceable};

pub trait BvhLeaf<F, B, H, M>: Bounded<F, B> + Traceable<F, H, M>
where
    F: Float,
    B: BoundingVolume<F>,
    H: Hitable<F, Material = M>,
    M: Material<F>,
{
}

impl<T, F, B, H, M> BvhLeaf<F, B, H, M> for T
where
    T: Bounded<F, B> + Traceable<F, H, M>,
    F: Float,
    B: BoundingVolume<F>,
    H: Hitable<F, Material = M>,
    M: Material<F>,
{
}

pub enum BvhNode<F, B, H, M, L>
where
    F: Float,
    B: BoundingVolume<F>,
    H: Hitable<F, Material = M>,
    M: Material<F>,
    L: BvhLeaf<F, B, H, M>,
{
    Node(Bvh<F, B, H, M, L>),
    Leaf(L),
}

impl<F, B, H, M, L> Bounded<F, B> for BvhNode<F, B, H, M, L>
where
    F: Float,
    B: BoundingVolume<F>,
    H: Hitable<F, Material = M>,
    M: Material<F>,
    L: BvhLeaf<F, B, H, M>,
{
    fn bounding_volume(&self) -> B {
        match self {
            BvhNode::Node(n) => n.bounding_volume(),
            BvhNode::Leaf(n) => n.bounding_volume(),
        }
    }
}

impl<F, B, H, M, L> Traceable<F, H, M> for BvhNode<F, B, H, M, L>
where
    F: Float,
    B: BoundingVolume<F>,
    H: Hitable<F, Material = M>,
    M: Material<F>,
    L: BvhLeaf<F, B, H, M>,
{
    fn trace(&self, ray: &Ray<F>) -> Option<(F, &H)> {
        match self {
            BvhNode::Node(n) => n.trace(ray),
            BvhNode::Leaf(n) => n.trace(ray),
        }
    }
}
