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

solana_program::declare_id!("spacR3JQmQ2gSQM4XPHs6XkRNAh2gYPKRB35xJ78rVr");
