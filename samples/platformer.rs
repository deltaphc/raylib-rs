extern crate raylib;
use legion::prelude::*;
use rand::Rng;
use raylib::prelude::*;
use structopt::StructOpt;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod options;

type GameResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const PPU: i32 = 16;
const fPPU: f32 = PPU as f32;

// const MAX_RUN: f32 = 90.0;
// const RUN_ACCEL: f32 = 1000.0;
// const RUN_REDUCE: f32 = 400.0;
// const AIR_MULT: f32 = 0.65;

// const HOLDING_MAX_RUN: f32 = 70.0;
// const HOLD_MIN_TIME: f32 = 0.35;

// const JUMP_GRACE_TIME: f32 = 0.1;
// const JUMP_SPEED: f32 = -105.0;
// const JUMP_HBOOST: f32 = 40.0;
// const VAR_JUMP_TIME: f32 = 0.2;

// const WALL_JUMP_CHECK_DIST: f32 = 3.0;
// const WALL_JUMP_FORCE_TIME: f32 = 0.16;
// const WALL_JUMP_H_SPEED: f32 = MAX_RUN + JUMP_SPEED;

// Control all physics to make game less floaty https://www.youtube.com/watch?v=hG9SzQxaCm8
const JUMP_HEIGHT: f32 = 3.0 * fPPU;
const TIME_TO_PEAK: f32 = 0.5;
const MAX_SPEED: f32 = 2.0 * fPPU;
const TIME_TO_MAX_SPEED: f32 = 0.2;
const TIME_TO_SLOW_DOWN: f32 = 0.1;
// Derived physics
// const GRAVITY: f32 = fPPU * 9.8;
const X_ACC: f32 = MAX_SPEED / TIME_TO_MAX_SPEED;
const X_FRIC: f32 = MAX_SPEED / TIME_TO_SLOW_DOWN;
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
    fn left(&self) -> f32 {
        let r = self.borrow();
        r.x
    }
    fn right(&self) -> f32 {
        let r = self.borrow();
        r.x2()
    }
    fn top(&self) -> f32 {
        let r = self.borrow();
        r.y
    }
    fn bottom(&self) -> f32 {
        let r = self.borrow();
        r.y2()
    }
    fn center(&self) -> Vector2 {
        let r = self.borrow();
        Vector2::new(r.x + r.width / 2.0, r.y + r.height / 2.0)
    }

    fn move_by(&self, v: Vector2) -> Rectangle {
        let mut r = self.borrow().clone();
        r.x += v.x;
        r.y += v.y;
        r
    }
    fn scaled(&self, x_scale: f32, y_scale: f32) -> Rectangle {
        let r = self.borrow();
        Rectangle::new(
            r.x * x_scale,
            r.y * y_scale,
            r.width * x_scale,
            r.height * y_scale,
        )
    }
    fn scaled_by(&self, v: &Vector2) -> Rectangle {
        self.scaled(v.x, v.y)
    }
}

trait RectMutEx: std::borrow::BorrowMut<Rectangle> {
    fn set_left(&mut self, v: f32) {
        let mut r = self.borrow_mut();
        r.x = v;
    }
    fn set_right(&mut self, v: f32) {
        let mut r = self.borrow_mut();
        r.x = v - r.width;
    }
    fn set_top(&mut self, v: f32) {
        let mut r = self.borrow_mut();
        r.y = v;
    }
    fn set_bottom(&mut self, v: f32) {
        let mut r = self.borrow_mut();
        r.y = v - r.height;
    }
}

impl RectEx for Rectangle {}
impl RectMutEx for Rectangle {}

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
    stop_timer: (f32, f32),
}

impl Default for Player {
    fn default() -> Player {
        Player {
            color: Color::WHITE,
            jumping: false,
            stop_timer: (0.0, 0.0),
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
#[derive(Clone, Default, Debug, PartialEq)]
struct Physics {
    status: PhysStatus,
    fall_mult: Option<f32>,
    jump_mult: Option<f32>,

    collision: Vec<(i32, Rectangle)>,
}

struct TileMap {
    textures: Vec<Texture2D>,
    map: tiled::Map,
    map_width: i32,
    map_height: i32,
    tile_width: i32,
    tile_height: i32,
    img_tiles: Vec<Rectangle>,
}

impl TileMap {
    fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        tilemap: impl AsRef<Path>,
        tilesheet: impl AsRef<Path>,
    ) -> GameResult<Self> {
        let file = File::open(tilemap).unwrap();
        let reader = BufReader::new(file);
        let map = tiled::parse_with_path(reader, tilesheet.as_ref()).unwrap();
        println!("{:?}", map);
        // load map texture
        let textures: std::result::Result<Vec<_>, _> = map.tilesets[0]
            .images
            .iter()
            .map(|i| {
                rl.load_texture(
                    &thread,
                    tilesheet
                        .as_ref()
                        .parent()
                        .unwrap()
                        .join(&i.source)
                        .to_str()
                        .unwrap(),
                )
            })
            .collect();
        let textures = textures?;
        let map_width = map.width as i32;
        let map_height = map.height as i32;
        let tile_width = map.tilesets[0].tile_width as i32;
        let tile_height = map.tilesets[0].tile_height as i32;
        let img_width = map.tilesets[0].images[0].width as i32;
        let img_height = map.tilesets[0].images[0].height as i32;
        let mut recs = vec![];
        for y in (0..(img_height / tile_height)) {
            for x in (0..(img_width / tile_width)) {
                recs.push(Rectangle::new(
                    (x * tile_width) as f32,
                    (y * tile_height) as f32,
                    tile_width as f32,
                    tile_height as f32,
                ));
            }
        }
        let img_tiles = recs;

        Ok(TileMap {
            textures,
            map,
            map_width,
            map_height,
            tile_width,
            tile_height,
            img_tiles,
        })
    }

    fn tile(&self, layer: i32, x: i32, y: i32) -> i32 {
        self.map.layers[layer as usize].tiles[y as usize][x as usize] as i32
    }

    fn world_to_map(&self, w: f32, h: f32) -> Vector2 {
        Vector2::new(self.map_width as f32 / w, self.map_height as f32 / h)
    }

    fn map_to_world(&self, w: f32, h: f32) -> Vector2 {
        Vector2::new(w / self.map_width as f32, h / self.map_height as f32)
    }

    fn collide_object(
        &self,
        vel: &mut Vector2,
        rec: &mut Rectangle,
        last: &Rectangle,
    ) -> Vec<(i32, Rectangle)> {
        // If we hit the boundaries, end it
        if rec.left() < 0.0 {
            rec.set_left(0.0);
            vel.x = 0.0;
        }
        if rec.right() > self.map.width as f32 {
            rec.set_right(self.map.width as f32);
            vel.x = 0.0;
        }
        if rec.top() < 0.0 {
            rec.set_top(0.0);
            vel.y = 0.0;
        }
        if rec.bottom() > self.map.height as f32 {
            rec.set_bottom(self.map.height as f32);
            vel.y = 0.0;
        }

        let bottom = rec.bottom().floor();
        let top = rec.top().floor();
        let left = rec.left().floor();
        let right = rec.right().floor();

        let mut collisions = vec![];

        // test top left
        let value = self.tile(0, left as i32, top as i32);
        if let Err(col) = TileMap::collide(value, vel, rec, last, left, top) {
            collisions.push((value, col));
        }

        // test top right
        let value = self.tile(0, right as i32, top as i32);
        if let Err(col) = TileMap::collide(value, vel, rec, last, right, top) {
            collisions.push((value, col));
        }

        // test bottom left
        let value = self.tile(0, left as i32, bottom as i32);
        if let Err(col) = TileMap::collide(value, vel, rec, last, left, bottom) {
            collisions.push((value, col));
        }

        // test bottom right
        let value = self.tile(0, right as i32, bottom as i32);
        if let Err(col) = TileMap::collide(value, vel, rec, last, right, bottom) {
            collisions.push((value, col));
        }
        collisions
    }

    fn collide(
        tile: i32,
        vel: &mut Vector2,
        rec: &mut Rectangle,
        last: &Rectangle,
        tile_x: f32,
        tile_y: f32,
    ) -> std::result::Result<(), Rectangle> {
        match tile {
            0 => (),
            15 => {
                TileMap::collide_platform_top(vel, rec, last, tile_y)?;
                TileMap::collide_platform_right(vel, rec, last, tile_x + 1.0)?;
                TileMap::collide_platform_left(vel, rec, last, tile_x)?;
                TileMap::collide_platform_bottom(vel, rec, last, tile_y + 1.0)?;
            }
            _ => unimplemented!("collided with tile unhandled {:?}", tile),
        }

        Ok(())
    }

    fn collide_platform_bottom(
        vel: &mut Vector2,
        rec: &mut Rectangle,
        last: &Rectangle,
        tile_bottom: f32,
    ) -> std::result::Result<(), Rectangle> {
        if rec.top() < tile_bottom && last.top() >= tile_bottom {
            rec.set_top(tile_bottom + 0.01);
            vel.y = 0.0;
            let mut col = rec.clone();
            col.set_bottom(tile_bottom);
            return Err(col);
        }
        Ok(())
    }

    fn collide_platform_left(
        vel: &mut Vector2,
        rec: &mut Rectangle,
        last: &Rectangle,
        tile_left: f32,
    ) -> std::result::Result<(), Rectangle> {
        if rec.right() > tile_left && last.right() <= tile_left {
            rec.set_right(tile_left - 0.01);
            vel.x = 0.0;
            let mut col = rec.clone();
            col.set_left(tile_left);
            return Err(col);
        }
        Ok(())
    }

    fn collide_platform_right(
        vel: &mut Vector2,
        rec: &mut Rectangle,
        last: &Rectangle,
        tile_right: f32,
    ) -> std::result::Result<(), Rectangle> {
        if rec.left() < tile_right && last.left() >= tile_right {
            rec.set_left(tile_right + 0.05);
            vel.x = 0.0;
            let mut col = rec.clone();
            col.set_right(tile_right);
            return Err(col);
        }
        Ok(())
    }

    fn collide_platform_top(
        vel: &mut Vector2,
        rec: &mut Rectangle,
        last: &Rectangle,
        tile_top: f32,
    ) -> std::result::Result<(), Rectangle> {
        if rec.bottom() > tile_top && last.bottom() <= tile_top {
            rec.set_bottom(tile_top - 0.01);
            vel.y = 0.0;
            let mut col = rec.clone();
            col.set_top(tile_top);
            return Err(col);
        }
        Ok(())
    }
}

struct World {
    width: i32,
    height: i32,
}

fn main() {
    // initialize raylib
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Platformer");
    let (w, h) = (opt.width, opt.height);

    let t_map = TileMap::new(
        &mut rl,
        &thread,
        "static/platformer/platformer.tmx",
        "static/platformer/platformPack_tilesheet@2.tsx",
    )
    .unwrap();

    let w2m = t_map.world_to_map(w as f32, h as f32);
    let m2w = t_map.map_to_world(w as f32, h as f32);
    // initialize legion
    let universe = Universe::new();
    let mut world = universe.create_world();

    let center = vec2(w as f32 / 2.0, h as f32 / 2.0);
    // Insert player
    world.insert(
        (),
        vec![(
            Position(center),
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
        let player = <(
            Read<Position>,
            Write<Velocity>,
            Write<Physics>,
            Write<Player>,
        )>::query();
        for (pos, mut vel, mut phys, mut player) in player.iter(&mut world) {
            if phys.collision.len() > 0 && phys.collision[0].1.top() * m2w.y > pos.0.y {
                player.jumping = false;
                if rl.is_key_down(KEY_SPACE) {
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
            if !player.jumping {
                if rl.is_key_down(KEY_A) {
                    vel.0.x = (vel.0.x - X_ACC * dt).min(-MAX_SPEED);
                    player.stop_timer = (vel.0.x, 0.0);
                } else if rl.is_key_down(KEY_D) {
                    vel.0.x = (vel.0.x + X_ACC * dt).max(MAX_SPEED);
                    player.stop_timer = (vel.0.x, 0.0);
                } else {
                    player.stop_timer.1 = (player.stop_timer.1 + dt).min(TIME_TO_MAX_SPEED);
                    let (sv, t) = player.stop_timer;
                    vel.0.x = sv + (-sv / TIME_TO_MAX_SPEED) * t;
                }
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
        let mut planed = Rectangle::default();
        for (mut pos, mut vel, collider, mut phy) in physics.iter(&mut world) {
            if phy.status == PhysStatus::Static {
                continue;
            }
            let mut p = pos.0.clone();
            let mut v = vel.0.clone();
            let col = collider.0;
            // Velocity Verlet
            let gravity = if v.y > 0.0 {
                phy.fall_mult.unwrap_or(1.0) * GRAVITY
            } else {
                phy.jump_mult.unwrap_or(1.0) * GRAVITY
            };
            let dp = v * dt + vec2(0.0, gravity) * 0.5 * dt * dt;
            // p = p + dx;
            v = v + DOWN * gravity * dt;

            // Reset collisions
            // phy.collision = None;

            // Transform coordinates to tile maps to make things easier
            // handle collisions on x axis
            phy.collision = Vec::new();
            let (dx, dvx) = {
                let p = p + Vector2::new(dp.x, 0.0);
                let mut cr = col.clone().move_by(p).scaled_by(&w2m);
                let lr = col.clone().move_by(pos.0).scaled_by(&w2m);
                let mut v_copy = Vector2::new(v.x * w2m.x, 0.0);
                phy.collision
                    .extend(t_map.collide_object(&mut v_copy, &mut cr, &lr));
                let np = cr.scaled_by(&m2w).center();
                (np.x, v_copy.x * m2w.y)
            };
            p.x = dx;
            let (dy, dvy) = {
                let p = p + Vector2::new(0.0, dp.y);
                let mut cr = col.clone().move_by(p).scaled_by(&w2m);
                let lr = col.clone().move_by(pos.0).scaled_by(&w2m);
                let mut v_copy = Vector2::new(0.0, v.y * w2m.y);
                phy.collision
                    .extend(t_map.collide_object(&mut v_copy, &mut cr, &lr));
                let np = cr.scaled_by(&m2w).center();
                (np.y, v_copy.y * m2w.y)
            };
            p.y = dy;
            v = Vector2::new(dvx, dvy);

            // let cr = Rectangle::new(p.x + col.x, p.y + col.y, col.width, col.height);
            // planed = cr.clone();
            // let lr: Rectangle =
            //     Rectangle::new(pos.0.x + col.x, pos.0.y + col.y, col.width, col.height);

            // let mut velt = Vector2::new(v.x * w2m.x, v.y * w2m.y);
            // let mut crt = Rectangle::new(
            //     cr.x * w2m.x,
            //     cr.y * w2m.y,
            //     cr.width * w2m.x,
            //     cr.height * w2m.y,
            // );
            // let lrt = Rectangle::new(
            //     lr.x * w2m.x,
            //     lr.y * w2m.y,
            //     lr.width * w2m.x,
            //     lr.height * w2m.y,
            // );

            // let collisions = t_map.collide_object(&mut velt, &mut crt, &lrt);
            // if collisions.len() > 0 {
            //     phy.collision = collisions;
            // }
            // // Transform back to world space
            // let mut p = Vector2::new(crt.x * m2w.x - col.x, crt.y * m2w.y - col.y);
            // let v = Vector2::new(velt.x * m2w.x, velt.y * m2w.y);

            // Set values
            pos.0 = p;
            vel.0 = v;
        }

        // Drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::GRAY);
        // Draw world bounds
        d.draw_rectangle_lines_ex(&bounds, 2, Color::BLACK);

        // Draw the world
        for x in 0..t_map.map.width as i32 {
            for y in 0..t_map.map.height as i32 {
                let tile = t_map.tile(0, x, y);
                match tile {
                    0 => {}
                    n => {
                        let source = t_map.img_tiles[(tile - 1) as usize];
                        // we need to scale to fit the window
                        let tw = w as f32 / t_map.map.width as f32;
                        let th = h as f32 / t_map.map.height as f32;
                        let out = Rectangle::new(x as f32 * tw, y as f32 * th, tw, th);
                        d.draw_texture_pro(
                            &t_map.textures[0],
                            source,
                            out,
                            Vector2::zero(),
                            0.0,
                            Color::WHITE,
                        );
                    }
                }
            }
        }

        // Draw the player
        let player_d = <(Read<Position>, Read<Collider>, Read<Player>)>::query();
        for (pos, collider, player) in player_d.iter(&mut world) {
            let col = collider.0;
            let pos = pos.0;
            let obj = Rectangle::new(pos.x + col.x, pos.y + col.y, col.width, col.height);
            d.draw_rectangle_rec(obj, player.color);

            // debug player
            d.draw_rectangle_lines_ex(planed, 1, Color::BLUE);
        }

        // Debug physics
        let phys_d = <(Read<Position>, Read<Physics>)>::query();
        for (pos, phys) in phys_d.iter(&mut world) {
            d.draw_circle_v(pos.0, 2.0, Color::RED);
            for (_, collision) in &phys.collision {
                d.draw_rectangle_lines_ex(collision.scaled(m2w.x, m2w.y), 2, Color::RED)
            }
        }
    }
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    return start * (1.0 - t) + end * t;
}
