
#![recursion_limit = "1024"]

extern crate serde_derive;

use yew::prelude::*;
// use yew::services::{ TimeoutService, Task };
use yew_router::prelude::*;
use yew_router::Switch;

use std::rc::Rc;
use std::time::Duration;

pub mod bulma;
pub mod managers;
pub mod menu;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
