pub mod base;
pub mod engine;

use base::BasePlugin;
use engine::{Engine, EngineConfig};

fn main() {
    let mut engine = Engine::default();
    let (mut rl, mut thread) = engine.init(EngineConfig {
        title: "ATMOS",
        w: (1920.0 / 1.5) as i32,
        h: (1080.0 / 1.5) as i32,
        ..Default::default()
    });
    engine.add_plugin::<BasePlugin>(&mut rl, &thread);
    engine.run(&mut rl, &mut thread);
}
