use crate::engine::*;
use raylib::prelude::*;

#[derive(Debug, Default)]
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    #[inline(always)]
    fn add_plugin(engine: &mut Engine, _: &mut RaylibHandle, _: &RaylibThread) {
        engine
            .add_resource(Camera2D {
                zoom: 0.25,
                ..Default::default()
            })
            .add_update(CameraPointer::update);
    }
}
#[derive(Debug, Default)]
pub struct CameraPointer {
    active: bool,
    pos: Vector2,
}
impl CameraPointer {
    pub fn update(engine: &mut Engine, (rl, _): (&mut RaylibHandle, &mut RaylibThread), _: f32) {
        let Some(mut new_camera) = engine.resource::<Camera2D>().cloned() else {
            return;
        };
        for CameraPointer { pos, active } in engine.world.query::<&CameraPointer>().iter() {
            if !*active {
                continue;
            }
            new_camera.target = *pos - rl.get_window_scale_dpi() / 2.0 / new_camera.zoom;
        }
        let Some(old_camera) = engine.resource_mut::<Camera2D>() else {
            return;
        };
        *old_camera = new_camera;
    }
}
