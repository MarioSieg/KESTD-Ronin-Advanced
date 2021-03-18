extern crate ronin;

use ronin::engine::Engine;

fn main() {
    let mut engine = Engine::initialize();
    engine.run();
    engine.shutdown();
}
