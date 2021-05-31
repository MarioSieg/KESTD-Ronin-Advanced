#![allow(dead_code)]

mod components;
mod config;
mod core;
mod engine;
mod logger;
mod panic_hook;
mod resources;
mod scenery;
mod scenery_resources;
mod scheduler;
mod service;
mod systems;

use crate::engine::Engine;

fn main() {
    println!("{}", std::mem::size_of::<scenery::Scenery>());
    let mut engine = Engine::initialize();
    engine.run();
    engine.shutdown();
}
