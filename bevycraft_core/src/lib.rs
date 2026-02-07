extern crate core;

mod identity;
mod registry;
mod memory;
mod io;

pub mod prelude {
    pub use crate::identity::{
        resource_id::*,
    };
    pub use crate::registry::{
        compiled_registry::CompiledRegistry,
        registry_solver::RegistrySolver,
    };
    pub use crate::memory::{
        simple_pool::SimplePool,
        packed_array_u32::PackedArrayU32,
    };
    pub use crate::io::{
        serializable_registry::SerializableRegistry,
    };
}