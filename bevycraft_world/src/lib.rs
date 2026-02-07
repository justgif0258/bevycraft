mod block;
mod morton;
mod chunk;
mod spatial;

pub mod prelude {
    pub use crate::block::{
        block::{
            Block,
            BlockBehaviour,
            BehaviourTrait
        },
    };
    pub use crate::morton::morton_3d::*;
    pub use crate::spatial::{
        child_descriptor::ChildDescriptor,
        svo::Svo,
    };
}

pub mod presets {
    pub use crate::block::{
        *
    };
}