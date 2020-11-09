#![recursion_limit = "1024"]

// #[macro_use]
// extern crate serde_derive;

#[cfg(feature = "fetch")]
pub mod core;
pub mod managers;
pub mod menu;
pub mod ui;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
