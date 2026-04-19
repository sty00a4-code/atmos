use crate::engine::*;
use noise::{NoiseFn, OpenSimplex};
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
            .add_resource(Stars::new(42))
            .add_startup(CameraPointer::startup)
            .add_update(CameraPointer::update)
            .add_draw(CameraPointer::draw);
    }
}

pub const STAR_TEXTS: [&str; 4] = [
    "space2/Lasers/laserBlue08.png",
    "space2/Lasers/laserBlue09.png",
    "space2/Lasers/laserBlue10.png",
    "space2/Lasers/laserBlue11.png",
];
#[derive(Debug)]
pub struct Stars {
    pub seed: u32,
    pub noise: OpenSimplex,
    pub cell_size: i32,
    pub star_chance: f64,
}

impl Stars {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            noise: OpenSimplex::new(seed),
            cell_size: 80,
            star_chance: 0.06,
        }
    }

    #[inline(always)]
    fn hash_u64(&self, x: i64, y: i64) -> u64 {
        let mut v = (self.seed as u64)
            ^ (x as u64).wrapping_mul(0x9e3779b97f4a7c15)
            ^ (y as u64).wrapping_mul(0xc2b2ae3d27d4eb4f);
        v ^= v >> 33;
        v = v.wrapping_mul(0xff51afd7ed558ccd);
        v ^= v >> 33;
        v = v.wrapping_mul(0xc4ceb9fe1a85ec53);
        v ^ (v >> 33)
    }

    #[inline(always)]
    pub fn star(&self, x: i32, y: i32) -> Option<(&'static str, Vector2)> {
        let cx = x.div_euclid(self.cell_size);
        let cy = y.div_euclid(self.cell_size);

        let region = self.noise.get([cx as f64 * 0.08, cy as f64 * 0.08]);
        if region < 0.2 {
            return None;
        }

        let h = self.hash_u64(cx as i64, cy as i64);
        let chance = (h as f64) / (u64::MAX as f64);
        if chance >= self.star_chance {
            return None;
        }

        let hx = self.hash_u64(cx as i64 + 1337, cy as i64 + 7331);
        let hy = self.hash_u64(cx as i64 + 9001, cy as i64 + 42);

        let fx = (hx as f32 / u64::MAX as f32) * self.cell_size as f32;
        let fy = (hy as f32 / u64::MAX as f32) * self.cell_size as f32;

        let px = cx * self.cell_size + fx as i32;
        let py = cy * self.cell_size + fy as i32;

        let tex = STAR_TEXTS[(h as usize) % STAR_TEXTS.len()];
        Some((tex, Vector2::new(px as f32, py as f32)))
    }
}

#[derive(Debug, Default)]
pub struct CameraPointer {
    pub active: bool,
    pub pos: Vector2,
}
impl CameraPointer {
    pub fn startup(engine: &mut Engine, (rl, thread): (&mut RaylibHandle, &mut RaylibThread)) {
        let Some(asset_server) = engine.resource_mut::<AssetServer>() else {
            return;
        };
        for key in STAR_TEXTS {
            dbg!(key);
            asset_server.load_texture(key, rl, thread);
        }
    }
    #[inline(always)]
    pub fn update(engine: &mut Engine, (rl, _): (&mut RaylibHandle, &mut RaylibThread), _: f32) {
        let Some(mut new_camera) = engine.resource::<Camera2D>().cloned() else {
            return;
        };
        for CameraPointer { pos, active } in engine.world.query::<&CameraPointer>().iter() {
            if !*active {
                continue;
            }
            let screen_size =
                Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32)
                    / new_camera.zoom;
            new_camera.target = *pos - screen_size / 2.0;
            break;
        }
        let Some(old_camera) = engine.resource_mut::<Camera2D>() else {
            return;
        };
        *old_camera = new_camera;
    }
    #[inline(always)]
    pub fn draw(engine: &mut Engine, (d, _): (&mut RaylibDrawHandle, &mut RaylibThread)) {
        let Some(asset_server) = engine.resource::<AssetServer>() else {
            return;
        };
        let Some(stars) = engine.resource::<Stars>() else {
            return;
        };
        let Some(camera) = engine.resource::<Camera2D>() else {
            return;
        };
        for CameraPointer { pos, active } in engine.world.query::<&CameraPointer>().iter() {
            if !*active {
                continue;
            }
            let screen_size =
                Vector2::new(d.get_screen_width() as f32, d.get_screen_height() as f32)
                    / camera.zoom;
            let min_x = ((pos.x - screen_size.x * 0.5) / 10.0).floor() as i32;
            let max_x = ((pos.x + screen_size.x * 0.5) / 10.0).ceil() as i32;
            let min_y = ((pos.y - screen_size.y * 0.5) / 10.0).floor() as i32;
            let max_y = ((pos.y + screen_size.y * 0.5) / 10.0).ceil() as i32;

            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if let Some((star, pos)) = stars.star(x * 10, y * 10)
                        && let Some(texture) = asset_server.get(star)
                    {
                        d.draw_texture_v(texture, pos, Color::WHITE);
                    }
                }
            }
            break;
        }
    }
}
