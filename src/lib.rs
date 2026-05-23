//! This is a module-level comment for a Rust lib

#![deny(clippy::arithmetic_side_effects)]

mod command;

pub use command::*;
mod functions;
mod types;
pub use types::*;
mod constants;
pub use constants::*;
mod traits;
pub use traits::*;
mod prompts;
