extern crate raylib;
use legion::prelude::*;
use raylib::prelude::*;
use structopt::StructOpt;

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
        let mut r = self.borrow();
        vec2(r.x, r.y)
    }
    fn tr(&self) -> Vector2 {
        let mut r = self.borrow();
        vec2(r.x + r.width, r.y)
    }
    fn bl(&self) -> Vector2 {
        let mut r = self.borrow();
        vec2(r.x, r.y + r.height)
    }
    fn br(&self) -> Vector2 {
        let mut r = self.borrow();
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum EnemyType {
    Green,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Enemy {
    kind: EnemyType,
    health: i32,
    speed: f32,
    bullet_speed: f32,
}

impl Enemy {
    fn green() -> Self {
        Self {
            kind: EnemyType::Green,
            health: 1,
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
}

struct Spawner {
    timer: f32,
    next_spawn: f32,
}

impl Default for Spawner {
    fn default() -> Self {
        Self {
            timer: 0.0,
            next_spawn: 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Bullet;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Collider {
    aabb: Rectangle,
}

impl Collider {
    fn new(rec: &Rectangle) -> Self {
        Self { aabb: rec.clone() }
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
    let mut destroy = Vec::new();

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
                Collider::new(&SQUARE),
            )
        }),
    );

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let time = rl.get_time();
        spawner.timer += dt;

        // Spawning Logic
        if spawner.timer > spawner.next_spawn {
            spawner.timer = 0.0;
            spawner.next_spawn = std::f32::INFINITY;

            let enemy = Enemy::green();
            world.insert(
                (Bounds::Destroy,),
                (0..1).map(|_| {
                    (
                        Position(vec2(ARENA_WIDTH as f32 / 2.0, ARENA_HEIGHT as f32 / 4.0)),
                        Velocity(vec2(0.0, 1.0) * enemy.speed),
                        enemy.anim(&s_indices),
                        enemy,
                        Collider::new(&SQUARE),
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
        )>::query();
        let mut player_shoot = None;
        for (pos, mut vel, mut anim, mut player) in query.iter(&mut world) {
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
                player.bullet_time += player.reload_speed;
                player_shoot = Some((
                    Position(pos.0),
                    Velocity(vec2(0.0, -player.bullet_speed)),
                    Sprite(s_indices.player_bullet_1),
                    Bullet,
                    Collider::new(&SQUARE),
                ));
            }
        }
        if let Some(player_shoot) = player_shoot {
            world.insert((Bounds::Destroy,), (0..1).map(|_| player_shoot));
        }

        // Things that move
        let query = <(Write<Position>, Read<Velocity>)>::query();
        for (mut pos, vel) in query.iter(&mut world) {
            let old_pos = pos.0;
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

        // Destroy stuff out of bounds

        let query = <(Read<Position>, Read<Collider>)>::query().filter(tag_value(&Bounds::Destroy));
        for (ent, (pos, col)) in query.iter_entities(&mut world) {
            let at = col.aabb.move_to(pos.x, pos.y);
            if !world_ex.check_collision_recs(&at) {
                destroy.push(ent);
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
        let query = <(Read<Position>, Read<Animation>)>::query();
        for (pos, anim) in query.iter(&mut world) {
            d.draw_texture_pro(
                &s_sheet,
                s_extents[anim.current],
                s_extents[anim.current].move_to(pos.x, pos.y).project(PS),
                vec2(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }

        // Cleanup
        for ent in destroy.drain(..) {
            world.delete(ent);
        }

        // for ext in &s_extents {
        //     d.draw_texture_rec(&s_sheet, ext, vec2(ext.x, ext.y), Color::WHITE);
        // }
    }
}
