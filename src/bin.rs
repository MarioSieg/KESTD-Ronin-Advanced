extern crate ronin;

use ronin::engine::Engine;

fn main() {
    let _ = ronin::setup_logger().init();
    let mut engine = Engine::initialize();
    engine.run();
    engine.shutdown();
}
