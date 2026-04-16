pub mod camera;
pub mod object;

use crate::{base::object::Object, engine::*};
use raylib::prelude::*;

use camera::CameraPlugin;
use object::ObjectPlugin;

#[derive(Debug, Default)]
pub struct BasePlugin;
impl Plugin for BasePlugin {
    #[inline(always)]
    fn add_plugin(engine: &mut Engine, rl: &mut RaylibHandle, thread: &RaylibThread) {
        engine
            .add_draw(clear_background)
            .add_plugin::<CameraPlugin>(rl, thread)
            .add_plugin::<ObjectPlugin>(rl, thread);
        engine.spawn((
            Object {
                pos: Vector2::zero(),
                vel: Vector2::one().normalized() * 200.0,
                dir: 0.0,
                tor: 100.0,
                size: 110.0,
                density: 1.0,
            },
            Asset {
                path: "space1/Ships/spaceShips_002.png",
            },
        ));
        engine.spawn((
            Object {
                pos: Vector2::one() * 500.0,
                vel: Vector2::zero(),
                dir: 0.0,
                tor: 0.0,
                size: 200.0,
                density: 1.0,
            },
            Asset {
                path: "space1/Meteors/spaceMeteors_001.png",
            },
        ));
    }
}
pub fn clear_background(_: &mut Engine, (d, _): (&mut RaylibDrawHandle, &mut RaylibThread)) {
    d.clear_background(Color::BLACK);
}
