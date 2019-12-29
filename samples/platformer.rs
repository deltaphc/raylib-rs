extern crate raylib;
use legion::prelude::*;
use rand::Rng;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

const PPU: i32 = 16;
const fPPU: f32 = PPU as f32;
// Control all physics to make game less floaty https://www.youtube.com/watch?v=hG9SzQxaCm8
const JUMP_HEIGHT: f32 = 2.0 * fPPU;
const TIME_TO_PEAK: f32 = 0.5;
const MAX_SPEED: f32 = 3.0 * fPPU;
const TIME_TO_MAX_SPEED: f32 = 1.0;
// Derived physics
// const GRAVITY: f32 = fPPU * 9.8;
const GRAVITY: f32 = 2.0 * JUMP_HEIGHT / (TIME_TO_PEAK * TIME_TO_PEAK);
const JUMP_VELOCITY: f32 = 2.0 * JUMP_HEIGHT / TIME_TO_PEAK;
const DOWN: Vector2 = Vector2::new(0.0, 1.0);
const UP: Vector2 = Vector2::new(0.0, -1.0);

const TILE_COLLIDER: Rectangle = Rectangle::new(-fPPU / 2.0, -fPPU / 2.0, fPPU, fPPU);

// Add helper methods to rectangle
trait RectEx: std::borrow::Borrow<Rectangle> {
    fn x2(&self) -> f32 {
        let r = self.borrow();
        r.x + r.width
    }
    fn y2(&self) -> f32 {
        let r = self.borrow();
        r.y + r.height
    }
}

impl RectEx for Rectangle {}

// Define our entity data types
#[derive(Clone, Copy, Default, Debug, PartialEq)]
struct Position(Vector2);

#[derive(Clone, Copy, Default, Debug, PartialEq)]
struct Velocity(Vector2);

#[derive(Clone, Debug)]
struct Collider(Rectangle);

#[derive(Clone, Debug)]
struct Player {
    color: Color,
    jumping: bool,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            color: Color::WHITE,
            jumping: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PhysStatus {
    Static,
    Kinematic,
}

impl Default for PhysStatus {
    fn default() -> PhysStatus {
        PhysStatus::Static
    }
}

// Physics
#[derive(Clone, Copy, Default, Debug, PartialEq)]
struct Physics {
    status: PhysStatus,
    fall_mult: Option<f32>,
    jump_mult: Option<f32>,

    collision: Option<Rectangle>,
}
struct World {
    width: i32,
    height: i32,
}

fn main() {
    // initialize raylib
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Logo");
    let (w, h) = (opt.width, opt.height);

    // initialize legion
    let universe = Universe::new();
    let mut world = universe.create_world();

    // Insert player
    world.insert(
        (),
        vec![(
            Position(vec2(w as f32 / 2.0, h as f32 / 2.0)),
            Velocity::default(),
            Player::default(),
            Physics {
                status: PhysStatus::Kinematic,
                fall_mult: Some(2.0),
                ..Default::default()
            },
            Collider(TILE_COLLIDER),
        )],
    );

    while !rl.window_should_close() {
        use raylib::consts::KeyboardKey::*;
        // Game Logic
        let dt = rl.get_frame_time();

        // Player Movement
        let player = <(Write<Velocity>, Write<Physics>, Write<Player>)>::query();
        for (mut vel, mut phys, mut player) in player.iter(&mut world) {
            if let Some(ref collision) = phys.collision {
                player.jumping = false;
                if rl.is_key_pressed(KEY_SPACE) {
                    vel.0.y -= JUMP_VELOCITY;
                    player.jumping = true;

                    player.color = Color {
                        r: rand::thread_rng().gen(),
                        g: rand::thread_rng().gen(),
                        b: rand::thread_rng().gen(),
                        a: 255,
                    };
                }
            }
            // If we aren't holding down space increase gravity
            if !rl.is_key_down(KEY_SPACE) {
                phys.jump_mult = Some(2.0);
            } else if player.jumping {
                phys.jump_mult = None;
            }
            // Left and right
            if rl.is_key_down(KEY_A) {
                vel.0.x = lerp(vel.0.x, -MAX_SPEED, 0.1);
            } else if rl.is_key_down(KEY_D) {
                vel.0.x = lerp(vel.0.x, MAX_SPEED, 0.1);
            } else {
                vel.0.x = lerp(vel.0.x, 0.0, 0.5);
            }
        }

        // physics step
        let physics = <(
            Write<Position>,
            Write<Velocity>,
            Read<Collider>,
            Write<Physics>,
        )>::query();
        let bounds = Rectangle::new(
            PPU as f32,
            PPU as f32,
            (w - 2 * PPU) as f32,
            (h - 2 * PPU) as f32,
        );

        for (mut pos, mut vel, collider, mut phy) in physics.iter(&mut world) {
            if phy.status == PhysStatus::Static {
                continue;
            }
            let mut p = pos.0;
            let mut v = vel.0;
            let col = collider.0;
            // Velocity Verlet
            let gravity = if v.y > 0.0 {
                phy.fall_mult.unwrap_or(1.0) * GRAVITY
            } else {
                phy.jump_mult.unwrap_or(1.0) * GRAVITY
            };
            p = p + v * dt + vec2(0.0, gravity) * 0.5 * dt * dt;
            v = v + DOWN * gravity * dt;

            // Reset collisions
            phy.collision = None;

            // Check out of bounds
            // Check if any of the four courners are out of bounds;
            let obj = Rectangle::new(p.x + col.x, p.y + col.y, col.width, col.height);
            let tl = vec2(obj.x, obj.y);
            let tr = vec2(obj.x + obj.width, obj.y);
            let bl = vec2(obj.x, obj.y + obj.height);
            let br: Vector2 = vec2(obj.x + obj.width, obj.y + obj.height);
            let out_of_bounds = !bounds.check_collision_point_rec(tl)
                || !bounds.check_collision_point_rec(tr)
                || !bounds.check_collision_point_rec(bl)
                || !bounds.check_collision_point_rec(br);

            // let out_of_bounds = !bounds.check_collision_recs(&obj);
            if out_of_bounds {
                // Set the collision based on degree of penetration
                // Basically pretend that the object was colliding with a collider the exact same size as it

                let collision = Rectangle::new(
                    // x
                    if tr.x > bounds.x2() {
                        bounds.x2()
                    } else if tl.x < bounds.x {
                        bounds.x - obj.width
                    } else {
                        obj.x
                    },
                    // y
                    if br.y > bounds.y2() {
                        bounds.y2()
                    } else if tr.y < bounds.x {
                        bounds.y - obj.height
                    } else {
                        obj.y
                    },
                    obj.width,
                    obj.height,
                );
                phy.collision = Some(collision);
                v.y = 0.0;
                let hw = obj.width / 2.0;
                let hh = obj.height / 2.0;
                p.x = p.x.max(bounds.x + hw).min(bounds.x + bounds.width - hw);
                p.y = p.y.max(bounds.y + hh).min(bounds.y + bounds.height - hh);
            }

            // Set values
            pos.0 = p;
            vel.0 = v;
        }

        // Drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::GRAY);
        // Draw world bounds
        d.draw_rectangle_lines_ex(&bounds, 2, Color::BLACK);
        // Draw the player
        let player_d = <(Read<Position>, Read<Collider>, Read<Player>)>::query();
        for (pos, collider, player) in player_d.iter(&mut world) {
            let col = collider.0;
            let pos = pos.0;
            let obj = Rectangle::new(pos.x + col.x, pos.y + col.y, col.width, col.height);
            d.draw_rectangle_rec(obj, player.color);
        }

        // Debug physics
        let phys_d = <(Read<Position>, Read<Physics>)>::query();
        for (pos, phys) in phys_d.iter(&mut world) {
            d.draw_circle_v(pos.0, 2.0, Color::RED);
            if let Some(ref collision) = phys.collision {
                d.draw_rectangle_lines_ex(collision, 2, Color::RED)
            }
        }
    }
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    return start * (1.0 - t) + end * t;
}
