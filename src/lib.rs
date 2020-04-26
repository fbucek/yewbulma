#![recursion_limit = "1024"]

#[macro_use]
extern crate serde_derive;

pub mod ui;
pub mod managers;
pub mod menu;
pub mod core;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
