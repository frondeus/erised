// This is an input for codegen pass.

pub mod codegen_input;

// And this is an output of the codegen pass

mod imp;
pub mod types;
pub mod visitor;

// Manually implemented
pub mod heap_types {
    pub use crate::codegen_input::*;
}

pub mod build;
pub mod builder;
pub mod destruct;
mod static_gen;
pub(crate) mod utils;
