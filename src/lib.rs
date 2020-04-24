#![recursion_limit = "1024"]

pub mod bulma;
pub mod managers;
pub mod menu;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
