//! Die if you touch fire
#[macro_use]
extern crate specs_derive;
use raylib::prelude::*;
use specs::prelude::*;
use std::collections::HashMap;

use structopt::StructOpt;

mod options;

/// Assume square grid
const TILE_COUNT: i32 = 20;
const MARGIN: i32 = 2;

#[derive(Clone, Component)]
struct Pos(i32, i32);

impl From<&Pos> for Vector2 {
    fn from(pos: &Pos) -> Vector2 {
        Vector2::new(pos.0 as f32, pos.1 as f32)
    }
}

impl From<Pos> for (i32, i32) {
    fn from(val: Pos) -> Self {
        (val.0, val.1)
    }
}

#[derive(Component)]
struct Fire;

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Player;

type EntityMap = HashMap<(i32, i32), Entity>;

#[derive(PartialEq, Clone, Copy)]
enum GameState {
    PLAYING,
    LOST,
}

struct DeathSys;

impl<'a> System<'a> for DeathSys {
    type SystemData = (
        WriteExpect<'a, GameState>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Fire>,
    );

    fn run(&mut self, (mut gs, players, fire): Self::SystemData) {
        // Touch fire then die
        if (&players, &fire).join().nth(0).is_some() {
            *gs = GameState::LOST;
            println!("Lost");
        }
    }
}

struct PlayerSys;

impl<'a> System<'a> for PlayerSys {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, RaylibHandle>,
        ReadExpect<'a, EntityMap>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, Pos>,
    );

    fn run(&mut self, (ents, rl, emap, mut players, pos): Self::SystemData) {
        use raylib::consts::KeyboardKey::*;

        let player = (&*ents, &pos, &players).join().nth(0).unwrap();

        let mut new_pos = player.1.clone();
        if rl.is_key_pressed(KEY_D) {
            new_pos.0 += 1;
        } else if rl.is_key_pressed(KEY_A) {
            new_pos.0 -= 1;
        } else if rl.is_key_pressed(KEY_W) {
            new_pos.1 -= 1;
        } else if rl.is_key_pressed(KEY_S) {
            new_pos.1 += 1;
        } else {
            return;
        }

        let p_ent = player.0;

        match emap.get(&new_pos.into()) {
            Some(e) => {
                players.insert(*e, Player).unwrap();
                players.remove(p_ent);
            }
            _ => println!("Nothing"),
        }
    }
}

// System is not thread safe
struct DrawSys {
    thread: RaylibThread,
}
impl<'a> System<'a> for DrawSys {
    type SystemData = (
        WriteExpect<'a, RaylibHandle>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Tile>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Fire>,
    );

    fn run(&mut self, (mut rl, player, tiles, pos, fire): Self::SystemData) {
        let (_, sh) = (rl.get_screen_width(), rl.get_screen_height());
        let tw = sh / TILE_COUNT - 2 * MARGIN;

        let margin = Vector2::new(MARGIN as f32, MARGIN as f32);
        let size = Vector2::new(tw as f32, tw as f32) + margin;
        let tile_size = Vector2::new(tw as f32, tw as f32);

        let mut d = rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);
        // draw the tiles
        for (pos, _) in (&pos, &tiles).join() {
            let p: Vector2 = pos.into();
            d.draw_rectangle_v(p * size + margin, tile_size, Color::RAYWHITE);
        }
        // draw the fire tiles
        for (pos, _, _) in (&pos, &tiles, &fire).join() {
            let p: Vector2 = pos.into();
            d.draw_rectangle_v(p * size + margin, tile_size, Color::RED);
        }
        // draw the player tiles
        for (pos, _, _) in (&pos, &tiles, &player).join() {
            let p: Vector2 = pos.into();
            d.draw_rectangle_v(p * size + margin, tile_size, Color::GREEN);
        }
    }
}

fn main() {
    let opt = options::Opt::from_args();
    let (rl, thread) = opt.open_window("Specs Example");
    let (_w, _h) = (opt.width, opt.height);

    let mut world = World::new();
    register_components(&mut world);
    let emap = init_world(&rl, &mut world);

    world.insert(rl);
    // Raylib Thread is not safe to send between threads, but we can force it with an ARC
    // It's up to the user to ensure the only systems that use it are
    // thread local otherwise you will segfault
    world.insert(emap);
    world.insert(GameState::PLAYING);
    let mut dispatcher = DispatcherBuilder::new()
        .with(DeathSys, "death_sys", &[])
        .with(PlayerSys, "player_sys", &[])
        // Drawing must be done on the same thread
        .with_thread_local(DrawSys { thread })
        .build();
    dispatcher.setup(&mut world);
    while !window_should_close(&world) && !player_lost(&world) {
        dispatcher.dispatch(&mut world);
    }
}

fn window_should_close(world: &World) -> bool {
    let rl = world.read_resource::<RaylibHandle>();
    rl.window_should_close()
}

fn player_lost(world: &World) -> bool {
    let gs = world.read_resource::<GameState>();
    *gs == GameState::LOST
}

fn register_components(world: &mut World) {
    world.register::<Tile>();
    world.register::<Pos>();
    world.register::<Player>();
    world.register::<Fire>();
}

fn init_world(rl: &RaylibHandle, world: &mut World) -> EntityMap {
    let (_, sh) = (rl.get_screen_width(), rl.get_screen_height());
    let _tw = sh / TILE_COUNT;

    let mut placed_player = false;
    let mut emap = EntityMap::new();

    for x in 0..TILE_COUNT {
        for y in 0..TILE_COUNT {
            let mut eb = world.create_entity().with(Tile).with(Pos(x, y));
            if !placed_player && rl.get_random_value::<i32>(0..100) < 10 {
                placed_player = true;
                eb = eb.with(Player);
            } else if rl.get_random_value::<i32>(0..100) < 10 {
                eb = eb.with(Fire);
            }

            let e = eb.build();
            emap.insert((x, y), e);
        }
    }
    emap
}
