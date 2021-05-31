mod components;
mod config;
mod engine;
mod impls;
mod resources;
mod scenery;
mod scenery_resources;
mod service;
mod systems;

use crate::engine::Engine;

fn main() {
    let mut engine = Engine::initialize();
    engine.run();
    engine.shutdown();
}
