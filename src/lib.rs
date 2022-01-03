// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(dead_code)]

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub const PREFIX: &str = "space_runners";
