extern crate raylib;
use legion::prelude::*;
use raylib::prelude::*;

const ARENA_WIDTH: i32 = 128;
const ARENA_HEIGHT: i32 = 128;
const PIXEL_SCALE: i32 = 4;
const PS: f32 = PIXEL_SCALE as f32;
const WINDOW_WIDTH: i32 = ARENA_WIDTH * PIXEL_SCALE;
const WINDOW_HEIGHT: i32 = ARENA_HEIGHT * PIXEL_SCALE;
const SQUARE: Rectangle = Rectangle::new(0.0, 0.0, 8.0, 8.0);

trait RectangleEx: std::borrow::Borrow<Rectangle> {
    fn resize(&self, scale: f32) -> Rectangle {
        let mut r = self.borrow().clone();
        r.width *= scale;
        r.height *= scale;
        r
    }

    fn project(&self, scale: f32) -> Rectangle {
        let mut r = self.borrow().clone();
        r.x *= scale;
        r.y *= scale;
        r.width *= scale;
        r.height *= scale;
        r
    }

    fn move_to(&self, x: f32, y: f32) -> Rectangle {
        let mut r = self.borrow().clone();
        r.x = x;
        r.y = y;
        r
    }

    fn tl(&self) -> Vector2 {
        let r = self.borrow();
        vec2(r.x, r.y)
    }
    fn tr(&self) -> Vector2 {
        let r = self.borrow();
        vec2(r.x + r.width, r.y)
    }
    fn bl(&self) -> Vector2 {
        let r = self.borrow();
        vec2(r.x, r.y + r.height)
    }
    fn br(&self) -> Vector2 {
        let r = self.borrow();
        vec2(r.x + r.width, r.y + r.height)
    }
}
impl RectangleEx for &Rectangle {}
impl RectangleEx for Rectangle {}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position(Vector2);
impl std::ops::Deref for Position {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Velocity(Vector2);
impl std::ops::Deref for Velocity {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Sprite(usize);
#[derive(Clone, Debug, PartialEq)]
struct SpriteIndices {
    player_anim: Animation,
    player_bullet_1: usize,
    green_enemy_anim: Animation,
}

#[derive(Clone, Debug, PartialEq)]
struct Animation {
    frames: Vec<usize>,
    current: usize,
    speed: f32,
    prog: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Health {
    health: i32,
    inv_time: f64,
    inv_dur: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Damage {
    amount: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Player {
    speed: f32,
    bullet_speed: f32,
    bullet_time: f64,
    reload_speed: f64,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            speed: ARENA_WIDTH as f32 / 2.0,
            bullet_speed: ARENA_HEIGHT as f32 / 1.0,
            bullet_time: 0.0,
            reload_speed: 0.5,
        }
    }
}

impl Player {
    fn health() -> Health {
        Health {
            health: 3,
            inv_time: 0.0,
            inv_dur: 3.0,
        }
    }
    fn damage() -> Damage {
        Damage { amount: 0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum EnemyType {
    Green,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Enemy {
    kind: EnemyType,
    speed: f32,
    bullet_speed: f32,
}

impl Enemy {
    fn green() -> Self {
        Self {
            kind: EnemyType::Green,
            speed: ARENA_HEIGHT as f32 / 10.0,
            bullet_speed: ARENA_HEIGHT as f32 / 5.0,
        }
    }

    fn anim(&self, sprite_indices: &SpriteIndices) -> Animation {
        use EnemyType::*;
        match self.kind {
            Green => sprite_indices.green_enemy_anim.clone(),
        }
    }

    fn health(&self) -> Health {
        match self.kind {
            Green => Health {
                health: 1,
                ..Default::default()
            },
        }
    }

    fn damage(&self) -> Damage {
        match self.kind {
            Green => Damage { amount: 1 },
        }
    }
}

struct Spawner {
    timer: f64,
    next_spawn: f64,
}

impl Default for Spawner {
    fn default() -> Self {
        Self {
            timer: 0.0,
            next_spawn: 5.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Bullet;

impl Bullet {
    fn health() -> Health {
        Health {
            health: 1,
            ..Default::default()
        }
    }
}

const L_PLAYER: u32 = 1;
const L_ENEMY: u32 = 1 << 2;
const L_ENEMY_BULLET: u32 = 1 << 3;
const L_PLAYER_BULLET: u32 = 1 << 4;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Collider {
    aabb: Rectangle,
    layer: u32,
    mask: u32,
}

impl Collider {
    fn new(rec: &Rectangle, layer: u32, mask: u32) -> Self {
        Self {
            aabb: rec.clone(),
            layer,
            mask,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Tint {
    start: Color,
    end: Color,
    frequency: f64,
    kill_at: f64,
}

impl Tint {
    fn color_at(&self, time: f64) -> Color {
        let channel = |a, b, f: f64, t: f64| (b as f64 - a as f64) * (f * t).sin() + a as f64;

        Color {
            r: channel(self.start.r, self.end.r, self.frequency, time) as u8,
            g: channel(self.start.g, self.end.g, self.frequency, time) as u8,
            b: channel(self.start.b, self.end.b, self.frequency, time) as u8,
            a: channel(self.start.a, self.end.a, self.frequency, time) as u8,
        }
    }

    fn invincibility(time: f64) -> Tint {
        Tint {
            start: Color::WHITE,
            end: Color::BLACK,
            frequency: 25.0,
            kill_at: time,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Bounds {
    StayIn,
    Destroy,
}

fn sprite_extents() -> (Vec<Rectangle>, SpriteIndices) {
    let extents = (0..128)
        .step_by(8)
        .flat_map(|y| {
            (0..128)
                .step_by(8)
                .map(move |x| Rectangle::new(x as f32, y as f32, 8.0, 8.0))
        })
        .collect();
    let indices = SpriteIndices {
        player_anim: Animation {
            frames: vec![1, 0, 2],
            current: 0,
            speed: 0.0,
            prog: 0.0,
        },
        player_bullet_1: 32,
        green_enemy_anim: Animation {
            frames: vec![3, 3 + 16],
            current: 3,
            speed: 5.0,
            prog: 0.0,
        },
    };
    (extents, indices)
}

fn load_sprite_sheet(rl: &mut RaylibHandle, thread: &RaylibThread) -> Texture2D {
    rl.load_texture(&thread, "static/pico8_invaders_sprites_LARGE.png")
        .expect("could not load spritesheet")
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Space Eaters")
        .build();
    let logo = raylib::prelude::Image::load_image("static/logo.png").unwrap();
    rl.set_window_icon(&logo);
    rl.set_target_fps(60);

    let (_w, _h) = (WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    // subtracting 8.0 is a hack so we only have to check the top left corner of most sprites
    // assuming they are 8x8
    let world_ex = Rectangle::new(0.0, 0.0, ARENA_WIDTH as f32, ARENA_HEIGHT as f32);

    let universe = Universe::new();
    let mut world = universe.create_world();

    let mut spawner = Spawner::default();
    let mut destroy_buf = Vec::new();
    let mut add_tint_buf = Vec::new();
    let mut remove_tint_buf = Vec::new();

    let (s_extents, s_indices) = sprite_extents();
    let s_sheet = load_sprite_sheet(&mut rl, &thread);

    // I know sprite could be a tag, but legion's documentation is so
    // terrible that I have no idea how to use tags right.
    world.insert(
        (Bounds::StayIn,),
        (0..1).map(|_| {
            (
                Position(vec2(ARENA_WIDTH as f32 / 2.0, ARENA_HEIGHT as f32 / 2.0)),
                Velocity(Vector2::zero()),
                s_indices.player_anim.clone(),
                Player::default(),
                Player::health(),
                Player::damage(),
                Collider::new(&SQUARE, L_PLAYER, L_ENEMY | L_ENEMY_BULLET),
            )
        }),
    );

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let time = rl.get_time();

        // Spawning Logic
        if spawner.timer < time {
            spawner.timer = time + spawner.next_spawn;

            let enemy = Enemy::green();
            world.insert(
                (Bounds::Destroy,),
                (0..1).map(|_| {
                    (
                        Position(vec2(ARENA_WIDTH as f32 / 2.0, ARENA_HEIGHT as f32 / 4.0)),
                        Velocity(vec2(0.0, 1.0) * enemy.speed),
                        enemy.anim(&s_indices),
                        enemy,
                        enemy.health(),
                        enemy.damage(),
                        Collider::new(&SQUARE, L_ENEMY, L_PLAYER | L_PLAYER_BULLET),
                    )
                }),
            );
        }

        // Player Logic
        let query = <(
            Write<Position>,
            Write<Velocity>,
            Write<Animation>,
            Write<Player>,
            Read<Collider>,
        )>::query();
        let mut player_shoot = None;
        for (pos, mut vel, mut anim, mut player, col) in query.iter(&mut world) {
            use raylib::consts::KeyboardKey::*;
            // Move stuff
            let right = rl.is_key_down(KEY_D);
            let left = rl.is_key_down(KEY_A);
            let down = rl.is_key_down(KEY_S);
            let up = rl.is_key_down(KEY_W);
            let x = match (right, left) {
                (true, _) => 1.0,
                (_, true) => -1.0,
                _ => 0.0,
            };
            let y = match (up, down) {
                (true, _) => -1.0,
                (_, true) => 1.0,
                _ => 0.0,
            };

            if left {
                anim.current = anim.frames[0];
            } else if right {
                anim.current = anim.frames[2];
            } else {
                anim.current = anim.frames[1];
            }

            vel.0 = vec2(x, y) * player.speed;

            // Bullet logic
            if rl.is_key_down(KEY_SPACE) && player.bullet_time < time {
                player.bullet_time = time + player.reload_speed;
                player_shoot = Some((
                    Position(pos.0),
                    Velocity(vec2(0.0, -player.bullet_speed)),
                    Sprite(s_indices.player_bullet_1),
                    Bullet,
                    Bullet::health(),
                    Damage { amount: 1 },
                    Collider::new(&SQUARE, L_PLAYER_BULLET, L_ENEMY),
                ));
            }
        }
        // Do the shooting
        if let Some(player_shoot) = player_shoot {
            world.insert((Bounds::Destroy,), (0..1).map(|_| player_shoot));
        }

        // Things that move
        let query = <(Write<Position>, Read<Velocity>)>::query();
        for (mut pos, vel) in query.iter(&mut world) {
            pos.0 += vel.0 * dt;
        }

        // Make sure player stays in bounds
        let query = <(Write<Position>, Read<Collider>)>::query().filter(tag_value(&Bounds::StayIn));
        for (mut pos, col) in query.iter(&mut world) {
            let at = col.aabb.move_to(pos.x, pos.y);
            let mut bounds = world_ex.clone();
            let min_y = bounds.y;
            let max_y = bounds.br().y - col.aabb.height;
            bounds.y += col.aabb.height;
            bounds.height -= 2.0 * col.aabb.height;

            if !bounds.check_collision_recs(&at) {
                if at.tl().x > bounds.tr().x {
                    pos.0.x = bounds.x;
                } else if at.tr().x < bounds.x {
                    pos.0.x = bounds.tr().x;
                }
                pos.0.y = pos.y.max(min_y).min(max_y);
            }
        }

        // Collision checking
        {
            // Damage checking
            let mut entities: Vec<_> =
                <(Read<Position>, Read<Collider>, Read<Damage>, Write<Health>)>::query()
                    .iter(&mut world)
                    .collect();
            let mut damage_done = vec![None; entities.len()];
            for i in 0..entities.len() {
                let (a_pos, a_col, a_dmg, ..) = &entities[i];
                let a_rec = a_col.aabb.move_to(a_pos.x, a_pos.y);
                for j in (i + 1)..entities.len() {
                    let (b_pos, b_col, b_dmg, ..) = &entities[j];
                    let b_rec = b_col.aabb.move_to(b_pos.x, b_pos.y);
                    if a_rec.check_collision_recs(&b_rec) && (a_col.mask & b_col.layer) > 0 {
                        damage_done[i] = Some(b_dmg.amount);
                        damage_done[j] = Some(a_dmg.amount);
                    }
                }
            }
            for (i, dmg) in damage_done.iter().enumerate() {
                if let Some(dmg) = dmg {
                    let health = &mut entities[i].3;
                    if health.inv_time < time && *dmg != 0 {
                        health.health -= dmg;
                        health.inv_time = time + health.inv_dur;
                        println!("took damage, {}, {:?}", dmg, health);
                    }
                }
            }
        }

        // Destroy stuff out of bounds

        let query = <(Read<Position>, Read<Collider>)>::query().filter(tag_value(&Bounds::Destroy));
        for (ent, (pos, col)) in query.iter_entities(&mut world) {
            let at = col.aabb.move_to(pos.x, pos.y);
            if !world_ex.check_collision_recs(&at) {
                destroy_buf.push(ent);
            }
        }

        // Destroy stuff if out of health or give it an invincibility tint
        let query = <(Read<Health>, TryWrite<Tint>)>::query();
        for (ent, (h, tint)) in query.iter_entities(&mut world) {
            if h.health <= 0 {
                destroy_buf.push(ent);
            }
            if h.inv_time != 0.0 && h.inv_time > time {
                if let Some(mut tint) = tint {
                    if tint.kill_at != h.inv_time {
                        // Pretty sure they are different tints in this case.
                        *tint = Tint::invincibility(h.inv_time);
                    }
                } else {
                    add_tint_buf.push((ent, Tint::invincibility(h.inv_time)))
                }
            }
        }

        // Draw Stuff
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::ORANGE);
        let query = <(Read<Position>, Read<Sprite>)>::query();
        for (pos, sprite) in query.iter(&mut world) {
            d.draw_texture_pro(
                &s_sheet,
                s_extents[sprite.0],
                s_extents[sprite.0].move_to(pos.x, pos.y).project(PS),
                vec2(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }
        let query = <(Read<Position>, Read<Animation>, TryRead<Tint>)>::query();
        for (pos, anim, tint) in query.iter(&mut world) {
            let tint = tint.map(|t| t.color_at(time)).unwrap_or(Color::WHITE);
            d.draw_texture_pro(
                &s_sheet,
                s_extents[anim.current],
                s_extents[anim.current].move_to(pos.x, pos.y).project(PS),
                vec2(0.0, 0.0),
                0.0,
                tint,
            );
        }

        // Draw explosion

        // Cleanup
        for ent in destroy_buf.drain(..) {
            world.delete(ent);
        }

        for (ent, tint) in add_tint_buf.drain(..) {
            world.add_component(ent, tint);
        }

        let query = <(Read<Tint>)>::query();
        for (ent, tint) in query.iter_entities(&mut world) {
            if tint.kill_at < time {
                remove_tint_buf.push(ent);
            }
        }

        for ent in remove_tint_buf.drain(..) {
            world.remove_component::<Tint>(ent);
        }

        // for ext in &s_extents {
        //     d.draw_texture_rec(&s_sheet, ext, vec2(ext.x, ext.y), Color::WHITE);
        // }
    }
}
