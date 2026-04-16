use crate::engine::*;
use hecs::Entity;
use raylib::prelude::*;

#[derive(Debug, Default)]
pub struct ObjectPlugin;
impl Plugin for ObjectPlugin {
    #[inline(always)]
    fn add_plugin(engine: &mut Engine, _: &mut RaylibHandle, _: &RaylibThread) {
        engine.add_update(Object::update).add_draw(Object::draw);
    }
}

#[derive(Debug, Default)]
pub struct Object {
    pub pos: Vector2,
    pub vel: Vector2,
    pub dir: f32,
    pub tor: f32,
    pub size: f32,
    pub density: f32,
}
impl Object {
    #[inline(always)]
    pub fn collision(&self, other: &Self) -> bool {
        self.pos.distance_to(other.pos) <= self.size + other.size
    }
    #[inline(always)]
    pub fn mass(&self) -> f32 {
        self.density * self.size
    }
    #[inline(always)]
    pub fn collision_response(a: &Object, b: &Object) -> (Vector2, Vector2, Vector2, Vector2) {
        let delta = b.pos - a.pos;
        let dist_sq = delta.length_sqr();
        let radii = a.size + b.size;

        if dist_sq >= radii * radii {
            return (
                Vector2::zero(),
                Vector2::zero(),
                Vector2::zero(),
                Vector2::zero(),
            );
        }

        let dist = dist_sq.sqrt().max(1e-6);
        let normal = delta / dist;
        let penetration = radii - dist;

        let inv_mass_a = if a.mass() > 0.0 { 1.0 / a.mass() } else { 0.0 };
        let inv_mass_b = if b.mass() > 0.0 { 1.0 / b.mass() } else { 0.0 };
        let inv_mass_sum = inv_mass_a + inv_mass_b;

        if inv_mass_sum == 0.0 {
            return (
                Vector2::zero(),
                Vector2::zero(),
                Vector2::zero(),
                Vector2::zero(),
            );
        }

        let percent = 0.8;
        let slop = 0.01;

        let correction_mag = ((penetration - slop).max(0.0) / inv_mass_sum) * percent;
        let pos_a = -normal * correction_mag * inv_mass_a;
        let pos_b = normal * correction_mag * inv_mass_b;

        let rv = b.vel - a.vel;
        let vel_along_normal = rv.dot(normal);

        if vel_along_normal > 0.0 {
            return (pos_a, pos_b, Vector2::zero(), Vector2::zero());
        }

        let restitution = 0.5;
        let j = -(1.0 + restitution) * vel_along_normal / inv_mass_sum;
        let impulse = normal * j;

        let vel_a = -impulse * inv_mass_a;
        let vel_b = impulse * inv_mass_b;

        (pos_a, pos_b, vel_a, vel_b)
    }
    #[inline(always)]
    pub fn update(engine: &mut Engine, _: (&mut RaylibHandle, &mut RaylibThread), dt: f32) {
        let entities: Vec<Entity> = engine
            .world
            .query::<(Entity, &Object)>()
            .iter()
            .map(|(e, _)| e)
            .collect();

        for &e in &entities {
            let mut body = engine.world.get::<&mut Object>(e).unwrap();
            let vel = body.vel;
            let tor = body.tor;
            body.pos += vel * dt;
            body.dir += tor * dt;
        }

        let mut corrections: Vec<(Entity, Vector2, Vector2)> = Vec::new();

        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let ea = entities[i];
                let eb = entities[j];

                let a = engine.world.get::<&Object>(ea).unwrap();
                let b = engine.world.get::<&Object>(eb).unwrap();

                if a.collision(&b) {
                    let (ca, cb, va, vb) = Object::collision_response(&a, &b);
                    corrections.push((ea, ca, va));
                    corrections.push((eb, cb, vb));
                }
            }
        }

        for (e, dp, dv) in corrections {
            let mut body = engine.world.get::<&mut Object>(e).unwrap();
            body.pos += dp;
            body.vel += dv;
        }
    }
    #[inline(always)]
    pub fn draw(engine: &mut Engine, (d, _): (&mut RaylibDrawHandle, &mut RaylibThread)) {
        let asset_server = engine.resource::<AssetServer>();
        for (e, object) in engine.world.query::<(Entity, &Object)>().iter() {
            if let Some(asset) = engine.world.get::<&Asset>(e).ok()
                && let Some(asset_server) = asset_server
                && let Some(texture) = asset_server.assets.get(asset.path)
            {
                let size = Vector2::new(texture.width as f32, texture.height as f32);
                let src_rect = Rectangle::new(0.0, 0.0, size.x, size.y);
                let dst_rect = Rectangle::new(object.pos.x, object.pos.y, size.x, size.y);
                d.draw_texture_pro(
                    texture,
                    src_rect,
                    dst_rect,
                    size / 2.0,
                    object.dir,
                    Color::WHITE,
                );
            }
            d.draw_circle_lines_v(object.pos, object.size, Color::WHITE);
        }
    }
}
