extern crate ronin;

use ronin::engine::Engine;

fn main() {
    let _ = simple_logger::SimpleLogger::new().init();
    let mut engine = Engine::initialize();
    engine.run();
    engine.shutdown();
}
