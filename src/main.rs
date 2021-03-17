extern crate libdynasim;

use libdynasim::engine::Engine;

fn main() {
    let mut engine = Engine::initialize();
    engine.run();
    engine.shutdown();
}
