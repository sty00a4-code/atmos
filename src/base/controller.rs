use crate::{
    base::object::Object,
    engine::{Asset, Combination, Engine, Plugin},
};
use raylib::prelude::*;

#[derive(Debug, Default)]
pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    #[inline(always)]
    fn add_plugin(engine: &mut Engine, rl: &mut RaylibHandle, thread: &RaylibThread) {
        engine
            .add_update(Controller::update)
            .add_update(Player::update)
            .spawn(
                Player {
                    controller: Controller::default(),
                    object: Object {
                        pos: Vector2::zero(),
                        vel: Vector2::zero(),
                        dir: 0.0,
                        tor: 0.0,
                        size: 110.0,
                        density: 1.0,
                    },
                    asset: Asset {
                        path: "space1/Ships/spaceShips_002.png",
                    },
                }
                .comp(),
            );
    }
}

#[derive(Debug, Default, Clone)]
pub struct Controller {
    pub forward: bool,
    pub left: bool,
    pub right: bool,
}
impl Controller {
    pub fn update(engine: &mut Engine, (rl, _): (&mut RaylibHandle, &mut RaylibThread), _: f32) {
        for controller in engine.world.query_mut::<&mut Controller>() {
            controller.forward = rl.is_key_down(KeyboardKey::KEY_W);
            controller.left = rl.is_key_down(KeyboardKey::KEY_A);
            controller.right = rl.is_key_down(KeyboardKey::KEY_D);
        }
    }
}

pub struct Player {
    pub controller: Controller,
    pub object: Object,
    pub asset: Asset,
}
impl Combination for Player {
    type Query<'a> = (&'a Controller, &'a Object, &'a Asset);
    type QueryMut<'a> = (&'a mut Controller, &'a mut Object, &'a mut Asset);
    fn comp(self) -> impl hecs::DynamicBundle {
        (self.controller, self.object, self.asset)
    }
}
impl Player {
    pub fn update(engine: &mut Engine, (rl, _): (&mut RaylibHandle, &mut RaylibThread), dt: f32) {
        let mut target = Vector2::zero();
        for (controller, object, _) in engine
            .world
            .query_mut::<(&mut Controller, &mut Object, &Asset)>()
        {
            if controller.forward {
                object.vel += Vector2::new(
                    -object.dir.to_radians().sin(),
                    object.dir.to_radians().cos(),
                ) * 300.0
                    * dt;
            }
            if controller.left {
                object.tor -= 100.0 * dt;
            }
            if controller.right {
                object.tor += 100.0 * dt;
            }
            target = object.pos;
        }
        let Some(old_cam) = engine.resource_mut::<Camera2D>() else {
            return;
        };
        let (w, h) = (rl.get_screen_width(), rl.get_screen_height());
        old_cam.target = target - (Vector2::new(w as f32, h as f32) / 2.0 / old_cam.zoom);
    }
}
