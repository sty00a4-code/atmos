use crate::{
    base::{camera::CameraPointer, object::Object},
    engine::{Asset, Combination, Engine, Plugin},
};
use raylib::prelude::*;

#[derive(Debug, Default)]
pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    #[inline(always)]
    fn add_plugin(engine: &mut Engine, _: &mut RaylibHandle, _: &RaylibThread) {
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
                    cam_ptr: CameraPointer {
                        active: true,
                        pos: Vector2::zero(),
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
    pub cam_ptr: CameraPointer,
}
impl Combination for Player {
    fn comp(self) -> impl hecs::DynamicBundle {
        (self.controller, self.object, self.asset, self.cam_ptr)
    }
}
impl Player {
    pub fn update(engine: &mut Engine, _: (&mut RaylibHandle, &mut RaylibThread), dt: f32) {
        for (controller, object, _, cam_ptr) in
            engine
                .world
                .query_mut::<(&mut Controller, &mut Object, &Asset, &mut CameraPointer)>()
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
            cam_ptr.pos = object.pos;
        }
    }
}
