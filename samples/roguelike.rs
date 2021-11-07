/// Code almost verbatim from here: http://tomassedovic.github.io/roguelike-tutorial/index.html
/// This only covers up to Part 9 because I'm a human being who needs sleep. Feel free to submit a
/// PR to extend it.
/// IMHO Don't write code like this. Use ECS and other methods to have game objects and components.
/// Only do this as an exercise.
extern crate raylib;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::Rng;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use structopt::StructOpt;
use tcod::map::{FovAlgorithm, Map as FovMap};

mod options;

/// Keep the player at index zero
const PLAYER: usize = 0;

// Window size
const W: i32 = 800;
const H: i32 = 640;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const TILE_WIDTH: i32 = W / SCREEN_WIDTH;
const TILE_HEIGHT: i32 = H / SCREEN_HEIGHT;

// Size for health bars
const BAR_WIDTH: i32 = 20;
const PANEL_HEIGHT: i32 = 7;
const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

// Size for messages
const MSG_X: i32 = BAR_WIDTH + 2;
const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

// Size of the map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

// Size of the room
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

// Color of the world
const COLOR_DARK_WALL: Color = Color::new(0, 0, 100, 255);
const COLOR_DARK_GROUND: Color = Color::new(50, 50, 150, 255);

// FOV
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;

// What the inventory menu width looks like
const INVENTORY_WIDTH: i32 = 50;

// How much an item heals
const HEAL_AMOUNT: i32 = 4;

// How much damage a lightning spell does
const LIGHTNING_DAMAGE: i32 = 40;
const LIGHTNING_RANGE: i32 = 5;

// Confusion spell config
const CONFUSE_RANGE: i32 = 8;
const CONFUSE_NUM_TURNS: i32 = 10;

// Fireball spell config
const FIREBALL_RADIUS: i32 = 3;
const FIREBALL_DAMAGE: i32 = 12;

// experience and level-ups
const LEVEL_UP_BASE: i32 = 200;
const LEVEL_UP_FACTOR: i32 = 150;

const LEVEL_SCREEN_WIDTH: i32 = 40;

const CHARACTER_SCREEN_WIDTH: i32 = 30;
// We can add custom methods to raylib types with extention traits
pub trait RectExt: std::ops::Deref<Target = Rectangle> {
    fn center(&self) -> (i32, i32) {
        let r: &Rectangle = self.deref();
        let center_x = r.y + r.width / 2.0;
        let center_y = r.y + r.width / 2.0;
        (center_x as i32, center_y as i32)
    }
}

// Boom, rectangles now have a center() method
impl RectExt for &Rectangle {}

/// Tcod contains the fov map. Unlike the tutorial we won't put framebuffers and other drawing
/// things here
struct Tcod {
    fov: FovMap,
    mouse: Vector2,
}

/// This enum tells us if the player has taken an action. This is significant
/// as monsters will not take a turn unless we mark a player action
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

/// Instead of attaching the closure to a component we mark it with
/// an enum so we can make it serializable
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    /// Simple fn dispatch
    fn callback(self, object: &mut Object, game: &mut Game) {
        use DeathCallback::*;
        let callback: fn(&mut Object, &mut Game) = match self {
            Player => player_death,
            Monster => monster_death,
        };
        callback(object, game);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
/// An object that can be equipped, yielding bonuses.
struct Equipment {
    slot: Slot,
    equipped: bool,
    power_bonus: i32,
    defense_bonus: i32,
    max_hp_bonus: i32,
}

/// Player can hold three items
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
enum Slot {
    LeftHand,
    RightHand,
    Head,
}

impl std::fmt::Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Slot::LeftHand => write!(f, "left hand"),
            Slot::RightHand => write!(f, "right hand"),
            Slot::Head => write!(f, "head"),
        }
    }
}

/// Anything that can attack or do damage
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct Fighter {
    base_max_hp: i32,
    hp: i32,
    base_defense: i32,
    base_power: i32,
    xp: i32,
    on_death: DeathCallback,
}

/// Monsters have to move somehow
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Ai {
    Basic,
    Confused {
        previous_ai: Box<Ai>,
        num_turns: i32,
    },
}

/// Pickups in the world
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
enum Item {
    Heal,
    Lightning,
    Confuse,
    Fireball,
    Sword,
    Shield,
}

/// What to do with an item after using it
enum UseResult {
    UsedUp,
    UsedAndKept,
    Cancelled,
}

/// We need to Serialize Colors. Unfortunately we can't use the trait extension
/// method we did before
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct Col(u8, u8, u8, u8);

impl From<Col> for Color {
    fn from(col: Col) -> Self {
        Self::new(col.0, col.1, col.2, col.3)
    }
}

impl From<Color> for Col {
    fn from(color: Color) -> Self {
        Self(color.r, color.g, color.b, color.a)
    }
}

/// Objects in the game. Items, monsters and the player. Items go in inventory.
/// insead of the objects vector
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Object {
    x: i32,
    y: i32,
    char: String,
    color: Col,
    name: String,
    blocks: bool,
    alive: bool,
    fighter: Option<Fighter>,
    ai: Option<Ai>,
    item: Option<Item>,
    equipment: Option<Equipment>,
    level: i32,
    always_visible: bool,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, name: &str, color: Color, blocks: bool) -> Self {
        Object {
            x,
            y,
            char: char.to_string(),
            color: color.into(),
            name: name.into(),
            alive: false,
            blocks,
            fighter: None,
            ai: None,
            item: None,
            equipment: None,
            level: 1,
            always_visible: false,
        }
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// return the distance to some coordinates
    pub fn distance(&self, x: i32, y: i32) -> f32 {
        (((x - self.x).pow(2) + (y - self.y).pow(2)) as f32).sqrt()
    }

    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn power(&self, game: &Game) -> i32 {
        let base_power = self.fighter.map_or(0, |f| f.base_power);
        let bonus: i32 = self
            .get_all_equipped(game)
            .iter()
            .map(|e| e.power_bonus)
            .sum();
        base_power + bonus
    }
    pub fn defense(&self, game: &Game) -> i32 {
        let base_defense = self.fighter.map_or(0, |f| f.base_defense);
        let bonus: i32 = self
            .get_all_equipped(game)
            .iter()
            .map(|e| e.defense_bonus)
            .sum();
        base_defense + bonus
    }

    pub fn max_hp(&self, game: &Game) -> i32 {
        let base_max_hp = self.fighter.map_or(0, |f| f.base_max_hp);
        let bonus: i32 = self
            .get_all_equipped(game)
            .iter()
            .map(|e| e.max_hp_bonus)
            .sum();
        base_max_hp + bonus
    }

    pub fn take_damage(&mut self, damage: i32, game: &mut Game) -> Option<i32> {
        if let Some(fighter) = self.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage;
            }
        }

        if let Some(fighter) = self.fighter {
            if fighter.hp <= 0 {
                self.alive = false;
                fighter.on_death.callback(self, game);
                return Some(fighter.xp);
            }
        }
        None
    }

    /// heal by the given amount, without going over the maximum
    pub fn heal(&mut self, amount: i32, game: &Game) {
        let _max_hp = self.max_hp(game);
        if let Some(ref mut fighter) = self.fighter {
            fighter.hp += amount;
            if fighter.hp > fighter.base_max_hp {
                fighter.hp = fighter.base_max_hp;
            }
        }
    }

    pub fn attack(&mut self, target: &mut Object, game: &mut Game) {
        let damage = self.power(game) - target.defense(game);
        if damage > 0 {
            game.messages.add(
                &format!(
                    "{} attacks {} for {} hit points.",
                    self.name, target.name, damage
                ),
                Color::WHITE,
            );
            if let Some(xp) = target.take_damage(damage, game) {
                // yield experience to the player
                self.fighter.as_mut().unwrap().xp += xp;
            }
        } else {
            game.messages.add(
                &format!(
                    "{} attacks {} but it has no effect!",
                    self.name, target.name
                ),
                Color::WHITE,
            );
        }
    }

    /// returns a list of equipped items
    pub fn get_all_equipped(&self, game: &Game) -> Vec<Equipment> {
        if self.name == "player" {
            game.inventory
                .iter()
                .filter(|item| item.equipment.map_or(false, |e| e.equipped))
                .map(|item| item.equipment.unwrap())
                .collect()
        } else {
            vec![] // other objects have no equipment
        }
    }

    /// Equip object and show a message about it
    pub fn equip(&mut self, messages: &mut Messages) {
        if self.item.is_none() {
            messages.add(
                format!("Can't equip {:?} because it's not an Item.", self),
                Color::RED,
            );
            return;
        };
        if let Some(ref mut equipment) = self.equipment {
            if !equipment.equipped {
                equipment.equipped = true;
                messages.add(
                    format!("Equipped {} on {}.", self.name, equipment.slot),
                    Color::GREEN,
                );
            }
        } else {
            messages.add(
                format!("Can't equip {:?} because it's not an Equipment.", self),
                Color::RED,
            );
        }
    }

    /// Dequip object and show a message about it
    pub fn dequip(&mut self, messages: &mut Messages) {
        if self.item.is_none() {
            messages.add(
                format!("Can't dequip {:?} because it's not an Item.", self),
                Color::RED,
            );
            return;
        };
        if let Some(ref mut equipment) = self.equipment {
            if equipment.equipped {
                equipment.equipped = false;
                messages.add(
                    format!("Dequipped {} from {}.", self.name, equipment.slot),
                    Color::YELLOW,
                );
            }
        } else {
            messages.add(
                format!("Can't dequip {:?} because it's not an Equipment.", self),
                Color::RED,
            );
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let c: Color = self.color.into();
        d.draw_text(
            &self.char,
            self.x * TILE_WIDTH,
            self.y * TILE_HEIGHT,
            TILE_HEIGHT,
            c,
        );
    }
}

struct Transition {
    level: u32,
    value: u32,
}

/// Returns a value that depends on level. the table specifies what
/// value occurs after each level, default is 0.
fn from_dungeon_level(table: &[Transition], level: u32) -> u32 {
    table
        .iter()
        .rev()
        .find(|transition| level >= transition.level)
        .map_or(0, |transition| transition.value)
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct Tile {
    blocked: bool,
    block_sight: bool,
    explored: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Self {
            blocked: false,
            explored: false,
            block_sight: false,
        }
    }
    pub fn wall() -> Self {
        Self {
            blocked: true,
            explored: false,
            block_sight: true,
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct Messages {
    messages: Vec<(String, Col)>,
}

impl Messages {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    pub fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        self.messages.push((message.into(), color.into()));
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &(String, Col)> {
        self.messages.iter()
    }
}

type Map = Vec<Vec<Tile>>;

#[derive(Serialize, Deserialize)]
struct Game {
    pub map: Map,
    messages: Messages,
    inventory: Vec<Object>,
    dungeon_level: u32,
}

fn new_game(tcod: &mut Tcod) -> (Game, Vec<Object>) {
    // create object representing the player
    let mut player = Object::new(0, 0, '@', "player", Color::WHITE, true);
    player.alive = true;
    player.fighter = Some(Fighter {
        base_max_hp: 100,
        hp: 100,
        base_defense: 1,
        base_power: 2,
        xp: 0,
        on_death: DeathCallback::Player,
    });

    // the list of objects with just the player
    let mut objects = vec![player];

    let mut game = Game {
        // generate map (at this point it's not drawn to the screen)
        map: make_map(&mut objects, 1),
        messages: Messages::new(),
        inventory: vec![],
        dungeon_level: 1,
    };

    // initial equipment: a dagger
    let mut dagger = Object::new(0, 0, '-', "dagger", Color::BLACK, false);
    dagger.item = Some(Item::Sword);
    dagger.equipment = Some(Equipment {
        equipped: true,
        slot: Slot::LeftHand,
        max_hp_bonus: 0,
        defense_bonus: 0,
        power_bonus: 2,
    });
    game.inventory.push(dagger);

    initialise_fov(tcod, &game.map);

    // a warm welcoming message!
    game.messages.add(
        "Welcome stranger! Prepare to perish in the Tombs of the Ancient Kings.",
        Color::RED,
    );

    (game, objects)
}
/// Advance to the next level
fn next_level(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) {
    game.messages.add(
        "You take a moment to rest, and recover your strength.",
        Color::VIOLET,
    );
    let heal_hp = objects[PLAYER].max_hp(game) / 2;
    objects[PLAYER].heal(heal_hp, game);

    game.messages.add(
        "After a rare moment of peace, you descend deeper into \
         the heart of the dungeon...",
        Color::RED,
    );
    game.dungeon_level += 1;
    game.map = make_map(objects, game.dungeon_level);
    initialise_fov(tcod, &game.map);
}

fn initialise_fov(tcod: &mut Tcod, map: &Map) {
    // create the FOV map, according to the generated map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x,
                y,
                !map[x as usize][y as usize].block_sight,
                !map[x as usize][y as usize].blocked,
            );
        }
    }
}

fn make_map(objects: &mut Vec<Object>, level: u32) -> Map {
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    assert_eq!(&objects[PLAYER] as *const _, &objects[0] as *const _);
    objects.truncate(1);
    let room1 = Rectangle::new(20.0, 15.0, 10.0, 15.0);
    let room2: Rectangle = Rectangle::new(50.0, 15.0, 10.0, 15.0);

    create_room(&room1, &mut map);
    create_room(&room2, &mut map);
    create_h_tunnel(25, 55, 23, &mut map);

    let mut rooms = vec![];
    for _ in 0..MAX_ROOMS {
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

        let new_room = Rectangle::new(x as f32, y as f32, w as f32, h as f32);
        let failed = rooms
            .iter()
            .any(|other| new_room.check_collision_recs(other));

        if !failed {
            create_room(&new_room, &mut map);
            place_objects(new_room, &mut map, objects, level);

            // get the center of the room
            let (new_x, new_y) = (&new_room).center();

            if rooms.is_empty() {
                // player room
                objects[PLAYER].set_pos(new_x, new_y);
            } else {
                let (prev_x, prev_y) = (&rooms[rooms.len() - 1]).center();
                // toss a coin and pick if we move horizontally or vertically
                if rand::random() {
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }

            rooms.push(new_room)
        }
    }

    // create stairs at the center of the last room
    let (last_room_x, last_room_y) = (&rooms[rooms.len() - 1]).center();
    let mut stairs = Object::new(last_room_x, last_room_y, '<', "stairs", Color::WHITE, false);
    stairs.always_visible = true;
    objects.push(stairs);

    map
}

fn place_objects(room: Rectangle, map: &Map, objects: &mut Vec<Object>, level: u32) {
    // monster random table
    let max_monsters = from_dungeon_level(
        &[
            Transition { level: 1, value: 2 },
            Transition { level: 4, value: 3 },
            Transition { level: 6, value: 5 },
        ],
        level,
    );

    // choose random number of monsters
    let num_monsters = rand::thread_rng().gen_range(0, max_monsters + 1);

    for _ in 0..num_monsters {
        let x = rand::thread_rng().gen_range(room.x + 1.0, room.x + room.width) as i32;
        let y = rand::thread_rng().gen_range(room.y + 1.0, room.y + room.height) as i32;

        // monster random table
        let troll_chance = from_dungeon_level(
            &[
                Transition {
                    level: 3,
                    value: 15,
                },
                Transition {
                    level: 5,
                    value: 30,
                },
                Transition {
                    level: 7,
                    value: 60,
                },
            ],
            level,
        );

        let monsters = ["orc", "troll"];
        let monster_weights = [80, troll_chance];
        let moster_distribution = WeightedIndex::new(&monster_weights).unwrap();
        let mut rng = thread_rng();

        let mut monster = match monsters[moster_distribution.sample(&mut rng)] {
            "orc" => {
                // create an orc
                let mut orc = Object::new(x, y, 'o', "orc", Color::GREEN.fade(0.8), true);
                orc.fighter = Some(Fighter {
                    base_max_hp: 20,
                    hp: 20,
                    base_defense: 0,
                    base_power: 4,
                    xp: 35,
                    on_death: DeathCallback::Monster,
                });
                orc.ai = Some(Ai::Basic);

                orc
            }
            "troll" => {
                // create a troll
                let mut troll = Object::new(x, y, 'T', "troll", Color::GREEN, true);
                troll.fighter = Some(Fighter {
                    base_max_hp: 30,
                    hp: 30,
                    base_defense: 2,
                    base_power: 8,
                    xp: 100,
                    on_death: DeathCallback::Monster,
                });
                troll.ai = Some(Ai::Basic);
                troll
            }
            _ => unreachable!(),
        };

        monster.alive = true;
        objects.push(monster)
    }

    // place items
    // choose random number of items
    // maximum number of items per room
    let max_items = from_dungeon_level(
        &[
            Transition { level: 1, value: 1 },
            Transition { level: 4, value: 2 },
        ],
        level,
    );

    // item random table
    let items = [
        Item::Heal,
        Item::Lightning,
        Item::Fireball,
        Item::Confuse,
        Item::Sword,
        Item::Shield,
    ];
    let item_weights = [
        32,
        from_dungeon_level(
            &[Transition {
                level: 4,
                value: 25,
            }],
            level,
        ),
        from_dungeon_level(
            &[Transition {
                level: 6,
                value: 25,
            }],
            level,
        ),
        from_dungeon_level(
            &[Transition {
                level: 2,
                value: 10,
            }],
            level,
        ),
        from_dungeon_level(&[Transition { level: 4, value: 5 }], level),
        from_dungeon_level(
            &[Transition {
                level: 8,
                value: 15,
            }],
            level,
        ),
    ];

    // choose random number of items
    let num_items = rand::thread_rng().gen_range(0, max_items + 1);
    for _ in 0..num_items {
        // choose random spot for this item
        let x = rand::thread_rng().gen_range(room.x as i32 + 1, (room.x + room.width) as i32);
        let y = rand::thread_rng().gen_range(room.y as i32 + 1, (room.y + room.height) as i32);

        // only place it if the tile is not blocked
        if !is_blocked(x, y, map, objects) {
            let item_distribution = WeightedIndex::new(&item_weights).unwrap();
            let mut item = match items[item_distribution.sample(&mut thread_rng())] {
                Item::Heal => {
                    // create a healing potion
                    let mut object =
                        Object::new(x, y, '!', "healing potion", Color::MAGENTA, false);
                    object.item = Some(Item::Heal);
                    object
                }
                Item::Lightning => {
                    // create a lightning bolt scroll
                    let mut object =
                        Object::new(x, y, '#', "scroll of lightning bolt", Color::YELLOW, false);
                    object.item = Some(Item::Lightning);
                    object
                }
                Item::Fireball => {
                    // create a fireball scroll
                    let mut object =
                        Object::new(x, y, '#', "scroll of fireball", Color::YELLOW, false);
                    object.item = Some(Item::Fireball);
                    object
                }
                Item::Confuse => {
                    // create a confuse scroll
                    let mut object =
                        Object::new(x, y, '#', "scroll of confusion", Color::YELLOW, false);
                    object.item = Some(Item::Confuse);
                    object
                }
                Item::Shield => {
                    // create a shield
                    let mut object = Object::new(x, y, '[', "shield", Color::ORANGE, false);
                    object.item = Some(Item::Shield);
                    object.equipment = Some(Equipment {
                        equipped: false,
                        slot: Slot::LeftHand,
                        max_hp_bonus: 0,
                        defense_bonus: 1,
                        power_bonus: 0,
                    });
                    object
                }

                Item::Sword => {
                    // create a sword
                    let mut object = Object::new(x, y, '/', "sword", Color::BLACK, false);
                    object.item = Some(Item::Sword);
                    object.equipment = Some(Equipment {
                        equipped: false,
                        slot: Slot::RightHand,
                        max_hp_bonus: 0,
                        defense_bonus: 0,
                        power_bonus: 3,
                    });
                    object
                }
            };

            item.always_visible = true;
            objects.push(item);
        }
    }
}

fn is_blocked(x: i32, y: i32, map: &Map, objects: &[Object]) -> bool {
    if map[x as usize][y as usize].blocked {
        return true;
    }

    objects
        .iter()
        .any(|object| object.blocks && object.pos() == (x, y))
}

/// find closest enemy, up to a maximum range, and in the player's FOV
fn closest_monster(tcod: &Tcod, objects: &[Object], max_range: i32) -> Option<usize> {
    let mut closest_enemy = None;
    let mut closest_dist = (max_range + 1) as f32; // start with (slightly more than) maximum range

    for (id, object) in objects.iter().enumerate() {
        if (id != PLAYER)
            && object.fighter.is_some()
            && object.ai.is_some()
            && tcod.fov.is_in_fov(object.x, object.y)
        {
            // calculate distance between this object and the player
            let dist = objects[PLAYER].distance_to(object);
            if dist < closest_dist {
                // it's closer, so remember it
                closest_enemy = Some(id);
                closest_dist = dist;
            }
        }
    }
    closest_enemy
}

fn create_room(room: &Rectangle, map: &mut Map) {
    for x in ((room.x + 1.0) as usize)..((room.x + room.width) as usize) {
        for y in (room.y + 1.0) as usize..(room.y + room.height) as usize {
            map[x][y] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in x1.min(x2)..(x1.max(x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in y1.min(y2)..(y1.max(y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn get_names_under_mouse(mouse: Vector2, objects: &[Object], fov_map: &FovMap) -> String {
    let (x, y) = (mouse.x as i32 / TILE_WIDTH, mouse.y as i32 / TILE_HEIGHT);

    let names = objects
        .iter()
        .filter(|obj| obj.pos() == (x, y) && fov_map.is_in_fov(obj.x, obj.y))
        .map(|obj| obj.name.clone())
        .collect::<Vec<_>>();

    names.join(", ")
}

fn get_equipped_in_slot(slot: Slot, inventory: &[Object]) -> Option<usize> {
    for (inventory_id, item) in inventory.iter().enumerate() {
        if item
            .equipment
            .as_ref()
            .map_or(false, |e| e.equipped && e.slot == slot)
        {
            return Some(inventory_id);
        }
    }
    None
}

fn play_game(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut Vec<Object>,
) {
    // force FOV "recompute" through the game loop
    let previous_player_positon = (-1, -1);

    while !rl.window_should_close() {
        // logic
        // handle game logic
        level_up(rl, thread, game, objects);

        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON) {
            tcod.mouse = rl.get_mouse_position();
        }

        let player_action = handle_keys(rl, thread, tcod, game, objects);
        if player_action == PlayerAction::Exit {
            save_game(game, objects).unwrap();
            break;
        }
        if objects[PLAYER].alive && player_action != PlayerAction::DidntTakeTurn {
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    ai_take_turn(id, &tcod, game, objects);
                }
            }
        }

        // drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::GRAY);
        let player = &objects[PLAYER];
        let fov_recompute = previous_player_positon != (player.x, player.y);
        render_all(tcod, &mut d, game, objects, fov_recompute);
    }
}

fn main() {
    let mut opt = options::Opt::from_args();
    opt.width = 800;
    opt.height = 640;
    let (mut rl, thread) = opt.open_window("Roguelike");
    let (_w, _h) = (opt.width, opt.height);
    rl.set_target_fps(20);

    // build FOV map
    let mut tcod = Tcod {
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
        mouse: Vector2::default(),
    };

    main_menu(&mut rl, &thread, &mut tcod);
}

fn handle_keys(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut Vec<Object>,
) -> PlayerAction {
    use raylib::consts::KeyboardKey::*;
    use PlayerAction::*;

    let pressed_key = rl.get_key_pressed_number();

    if rl.is_key_down(KEY_LEFT_ALT) && rl.is_key_pressed(KEY_ENTER) {
        rl.toggle_fullscreen();
        return DidntTakeTurn;
    }

    if rl.is_key_pressed(KEY_Q) {
        return Exit;
    }

    let player_alive = objects[PLAYER].alive;
    if player_alive {
        if rl.is_key_pressed(KEY_W) {
            player_move_or_attack(0, -1, game, objects);
        } else if rl.is_key_pressed(KEY_S) {
            player_move_or_attack(0, 1, game, objects);
        } else if rl.is_key_pressed(KEY_A) {
            player_move_or_attack(-1, 0, game, objects);
        } else if rl.is_key_pressed(KEY_D) {
            player_move_or_attack(1, 0, game, objects);
        } else if rl.is_key_pressed(KEY_G) {
            let item_id = objects
                .iter()
                .position(|object| object.pos() == objects[PLAYER].pos() && object.item.is_some());
            if let Some(item_id) = item_id {
                pick_item_up(item_id, game, objects);
            }
        } else if rl.is_key_pressed(KEY_I) {
            // menus
            let mut exit = false;
            while !rl.window_should_close() {
                let inventory_index = {
                    let mut d = rl.begin_drawing(thread);
                    render_all(tcod, &mut d, game, objects, false);
                    inventory_menu(
                        &game.inventory,
                        "Press the key next to an item to use it, or any other to cancel.",
                        pressed_key,
                        &mut d,
                    )
                };
                // using items
                if let Some(inventory_index) = inventory_index {
                    use_item(rl, &thread, inventory_index, tcod, game, objects);
                    break;
                }
                if exit {
                    break;
                }
                if let Some(_key) = rl.get_key_pressed_number() {
                    exit = true;
                }
            }
        } else if rl.is_key_pressed(KEY_F) {
            let mut exit = false;
            while !rl.window_should_close() {
                let inventory_index = {
                    let mut d = rl.begin_drawing(thread);
                    render_all(tcod, &mut d, game, objects, false);
                    inventory_menu(
                        &game.inventory,
                        "Press the key next to an item to drop it, or any other to cancel.\n'",
                        pressed_key,
                        &mut d,
                    )
                };
                // using items
                if let Some(inventory_index) = inventory_index {
                    drop_item(inventory_index, game, objects);
                    break;
                }
                if exit {
                    break;
                }
                if let Some(_key) = rl.get_key_pressed_number() {
                    exit = true;
                }
            }
        } else if rl.is_key_pressed(KEY_COMMA) {
            let player_on_stairs = objects
                .iter()
                .any(|object| object.pos() == objects[PLAYER].pos() && object.name == "stairs");
            if player_on_stairs {
                next_level(tcod, game, objects);
            }
            return DidntTakeTurn;
        } else if rl.is_key_pressed(KEY_C) {
            while !rl.window_should_close() {
                let player = &objects[PLAYER];
                let level = player.level;
                let level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR;
                if let Some(fighter) = player.fighter.as_ref() {
                    let msg = format!(
                        "Character information

Level: {}
Experience: {}
Experience to level up: {}

Maximum HP: {}
Attack: {}
Defense: {}",
                        level,
                        fighter.xp,
                        level_up_xp,
                        player.max_hp(game),
                        player.power(game),
                        player.defense(game)
                    );
                    {
                        let mut d = rl.begin_drawing(thread);
                        render_all(tcod, &mut d, game, objects, false);
                        msgbox(&msg, CHARACTER_SCREEN_WIDTH, pressed_key, &mut d);
                    }
                    if let Some(_key) = rl.get_key_pressed_number() {
                        break;
                    }
                }
            }
        } else {
            return DidntTakeTurn;
        }
        return TookTurn;
    }
    return DidntTakeTurn;
}

/// add to the player's inventory and remove from the map
fn pick_item_up(object_id: usize, game: &mut Game, objects: &mut Vec<Object>) {
    if game.inventory.len() >= 26 {
        game.messages.add(
            format!(
                "Your inventory is full, cannot pick up {}.",
                objects[object_id].name
            ),
            Color::RED,
        );
    } else {
        let item = objects.swap_remove(object_id);
        game.messages
            .add(format!("You picked up a {}!", item.name), Color::GREEN);
        let index = game.inventory.len();
        let slot = item.equipment.map(|e| e.slot);
        game.inventory.push(item);

        if let Some(slot) = slot {
            if get_equipped_in_slot(slot, &game.inventory).is_none() {
                game.inventory[index].equip(&mut game.messages);
            }
        }
    }
}

fn drop_item(inventory_id: usize, game: &mut Game, objects: &mut Vec<Object>) {
    let mut item = game.inventory.remove(inventory_id);
    if item.equipment.is_some() {
        item.dequip(&mut game.messages);
    }
    item.set_pos(objects[PLAYER].x, objects[PLAYER].y);
    game.messages
        .add(format!("You dropped a {}.", item.name), Color::YELLOW);
    objects.push(item);
}

fn use_item(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    inventory_id: usize,
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],
) {
    use Item::*;
    // just call the "use_function" if it is defined
    if let Some(item) = game.inventory[inventory_id].item {
        let on_use = match item {
            Heal => cast_heal,
            Lightning => cast_lightning,
            Confuse => cast_confuse,
            Fireball => cast_fireball,
            Sword => toggle_equipment,
            Shield => toggle_equipment,
        };
        match on_use(rl, thread, inventory_id, tcod, game, objects) {
            UseResult::UsedUp => {
                // destroy after use, unless it was cancelled for some reason
                game.inventory.remove(inventory_id);
            }
            UseResult::UsedAndKept => {} // do nothing
            UseResult::Cancelled => {
                game.messages.add("Cancelled", Color::WHITE);
            }
        }
    } else {
        game.messages.add(
            format!("The {} cannot be used.", game.inventory[inventory_id].name),
            Color::WHITE,
        );
    }
}

fn cast_heal(
    _rl: &mut RaylibHandle,
    _thread: &RaylibThread,
    _inventory_id: usize,
    _tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],
) -> UseResult {
    // heal the player
    if let Some(fighter) = objects[PLAYER].fighter {
        if fighter.hp == fighter.base_max_hp {
            game.messages
                .add("You are already at full health.", Color::RED);
            return UseResult::Cancelled;
        }
        game.messages
            .add("Your wounds start to feel better!", Color::VIOLET);
        objects[PLAYER].heal(HEAL_AMOUNT, game);
        return UseResult::UsedUp;
    }
    UseResult::Cancelled
}

fn cast_lightning(
    _rl: &mut RaylibHandle,
    _thread: &RaylibThread,
    _inventory_id: usize,
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],
) -> UseResult {
    // find closest enemy (inside a maximum range and damage it)
    let monster_id = closest_monster(tcod, objects, LIGHTNING_RANGE);
    if let Some(monster_id) = monster_id {
        // zap it!
        game.messages.add(
            format!(
                "A lightning bolt strikes the {} with a loud thunder! \
                 The damage is {} hit points.",
                objects[monster_id].name, LIGHTNING_DAMAGE
            ),
            Color::BLUE,
        );
        if let Some(xp) = objects[monster_id].take_damage(LIGHTNING_DAMAGE, game) {
            objects[PLAYER].fighter.as_mut().unwrap().xp += xp;
        }
        UseResult::UsedUp
    } else {
        // no enemy found within maximum range
        game.messages
            .add("No enemy is close enough to strike.", Color::RED);
        UseResult::Cancelled
    }
}

fn cast_confuse(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    _inventory_id: usize,
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],
) -> UseResult {
    // find closest enemy in-range and confuse it
    game.messages.add(
        "Left-click an enemy to confuse it, or right-click to cancel.",
        Color::BLUE,
    );
    let monster_id = target_monster(rl, thread, tcod, game, objects, Some(CONFUSE_RANGE as f32));
    if let Some(monster_id) = monster_id {
        let old_ai = objects[monster_id].ai.take().unwrap_or(Ai::Basic);
        // replace the monster's AI with a "confused" one; after
        // some turns it will restore the old AI
        objects[monster_id].ai = Some(Ai::Confused {
            previous_ai: Box::new(old_ai),
            num_turns: CONFUSE_NUM_TURNS,
        });
        game.messages.add(
            format!(
                "The eyes of {} look vacant, as he starts to stumble around!",
                objects[monster_id].name
            ),
            Color::GREEN,
        );
        UseResult::UsedUp
    } else {
        // no enemy fonud within maximum range
        game.messages
            .add("No enemy is close enough to strike.", Color::RED);
        UseResult::Cancelled
    }
}

fn cast_fireball(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    _inventory_id: usize,
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],
) -> UseResult {
    // ask the player for a target tile to throw a fireball at
    game.messages.add(
        "Left-click a target tile for the fireball, or right-click to cancel.",
        Color::BLUE,
    );
    let (x, y) = match target_tile(rl, thread, tcod, game, objects, None) {
        Some(tile_pos) => tile_pos,
        None => return UseResult::Cancelled,
    };
    game.messages.add(
        format!(
            "The fireball explodes, burning everything within {} tiles!",
            FIREBALL_RADIUS
        ),
        Color::ORANGE,
    );

    let mut xp_to_gain = 0;
    for (id, obj) in objects.iter_mut().enumerate() {
        if obj.distance(x, y) <= FIREBALL_RADIUS as f32 && obj.fighter.is_some() {
            game.messages.add(
                format!(
                    "The {} gets burned for {} hit points.",
                    obj.name, FIREBALL_DAMAGE
                ),
                Color::ORANGE,
            );
            if let Some(xp) = obj.take_damage(FIREBALL_DAMAGE, game) {
                if id != PLAYER {
                    xp_to_gain += xp;
                }
            }
        }
    }
    objects[PLAYER].fighter.as_mut().unwrap().xp += xp_to_gain;

    UseResult::UsedUp
}

fn toggle_equipment(
    _rl: &mut RaylibHandle,
    _thread: &RaylibThread,
    inventory_id: usize,
    _tcod: &mut Tcod,
    game: &mut Game,
    _objects: &mut [Object],
) -> UseResult {
    let equipment = match game.inventory[inventory_id].equipment {
        Some(equipment) => equipment,
        None => return UseResult::Cancelled,
    };
    // if the slot is already being used, dequip whatever is there first
    if let Some(current) = get_equipped_in_slot(equipment.slot, &game.inventory) {
        game.inventory[current].dequip(&mut game.messages);
    } else {
        game.inventory[inventory_id].equip(&mut game.messages);
    }
    UseResult::UsedAndKept
}

fn player_move_or_attack(dx: i32, dy: i32, game: &mut Game, objects: &mut [Object]) {
    let x = objects[PLAYER].x + dx;
    let y = objects[PLAYER].y + dy;

    // try to find an attackable object there
    let target_id = objects
        .iter()
        .position(|object| object.fighter.is_some() && object.pos() == (x, y));

    match target_id {
        Some(target_id) => {
            let (player, target) = mut_two(PLAYER, target_id, objects);
            player.attack(target, game);
        }
        None => move_by(PLAYER, dx, dy, &game.map, objects),
    }
}

fn ai_take_turn(monster_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut [Object]) {
    use Ai::*;
    if let Some(ai) = objects[monster_id].ai.take() {
        let new_ai = match ai {
            Basic => ai_basic(monster_id, tcod, game, objects),
            Confused {
                previous_ai,
                num_turns,
            } => ai_confused(monster_id, tcod, game, objects, previous_ai, num_turns),
        };
        objects[monster_id].ai = Some(new_ai);
    }
}

fn ai_basic(monster_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut [Object]) -> Ai {
    // a basic monster takes its turn. If you can see it, it can see you
    let (monster_x, monster_y) = objects[monster_id].pos();
    if tcod.fov.is_in_fov(monster_x, monster_y) {
        if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
            // move towards player if far away
            let (player_x, player_y) = objects[PLAYER].pos();
            move_towards(monster_id, player_x, player_y, &game.map, objects);
        } else if objects[PLAYER].fighter.map_or(false, |f| f.hp > 0) {
            // close enough, attack! (if the player is still alive.)
            let (monster, player) = mut_two(monster_id, PLAYER, objects);
            monster.attack(player, game);
        }
    }
    Ai::Basic
}

fn ai_confused(
    monster_id: usize,
    _tcod: &Tcod,
    game: &mut Game,
    objects: &mut [Object],
    previous_ai: Box<Ai>,
    num_turns: i32,
) -> Ai {
    if num_turns >= 0 {
        // still confused ...
        // move in a random direction, and decrease the number of turns confused
        move_by(
            monster_id,
            rand::thread_rng().gen_range(-1, 2),
            rand::thread_rng().gen_range(-1, 2),
            &game.map,
            objects,
        );
        Ai::Confused {
            previous_ai: previous_ai,
            num_turns: num_turns - 1,
        }
    } else {
        // restore the previous AI (this one will be deleted)
        game.messages.add(
            format!("The {} is no longer confused!", objects[monster_id].name),
            Color::RED,
        );
        *previous_ai
    }
}

fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, objects: &mut [Object]) {
    let dx = target_x - objects[id].x;
    let dy = target_y - objects[id].y;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    // normalize it to length 1 (preseving direction), then round it
    // convert to integer so the movement is restricted to the map grid
    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;
    move_by(id, dx, dy, map, objects);
}

fn move_by(id: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
    let (x, y) = objects[id].pos();
    if !is_blocked(x + dx, y + dy, map, objects) {
        objects[id].set_pos(x + dx, y + dy);
    }
}

fn level_up(rl: &mut RaylibHandle, thread: &RaylibThread, game: &mut Game, objects: &mut [Object]) {
    let player = &mut objects[PLAYER];
    let level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR;
    // see if the player's experience is enough to level-up
    if player.fighter.as_ref().map_or(0, |f| f.xp) >= level_up_xp {
        // it is! level up
        player.level += 1;
        game.messages.add(
            format!(
                "Your battle skills grow stronger! You reached level {}!",
                player.level
            ),
            Color::YELLOW,
        );
        let fighter = player.fighter.as_mut().unwrap();
        let mut choice = None;
        while choice.is_none() {
            let pressed_key = rl.get_key_pressed_number();
            let mut d = rl.begin_drawing(thread);
            // keep asking until a choice is made
            choice = menu(
                "Level up! Choose a stat to raise:\n",
                &[
                    format!("Constitution (+20 HP, from {})", fighter.base_max_hp),
                    format!("Strength (+1 attack, from {})", fighter.base_power),
                    format!("Agility (+1 defense, from {})", fighter.base_defense),
                ],
                LEVEL_SCREEN_WIDTH,
                pressed_key,
                &mut d,
            );
        }
        fighter.xp -= level_up_xp;
        match choice.unwrap() {
            0 => {
                fighter.base_max_hp += 20;
                fighter.hp += 20;
            }
            1 => {
                fighter.base_power += 1;
            }
            2 => {
                fighter.base_defense += 1;
            }
            _ => unreachable!(),
        }
    }
}

fn player_death(player: &mut Object, game: &mut Game) {
    game.messages.add("You died!", Color::RED);

    player.char = "%".to_owned();
    player.color = Color::DARKPURPLE.into();
}

fn monster_death(monster: &mut Object, game: &mut Game) {
    game.messages
        .add(&format!("{} is dead!", monster.name), Color::ORANGE);
    // transform it into a nasty corpse! it doesn't block, can't be
    // attacked and doesn't move
    game.messages.add(
        format!(
            "{} is dead! You gain {} experience points.",
            monster.name,
            monster.fighter.unwrap().xp
        ),
        Color::ORANGE,
    );
    monster.char = "%".to_owned();
    monster.color = Color::DARKPURPLE.into();
    monster.blocks = false;
    monster.fighter = None;
    monster.ai = None;
    monster.name = format!("remains of {}", monster.name);
}

fn render_bar(
    d: &mut RaylibDrawHandle,
    x: i32,
    y: i32,
    total_width: i32,
    name: &str,
    value: i32,
    maximum: i32,
    bar_color: Color,
    back_color: Color,
) {
    let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

    d.draw_rectangle(
        x * TILE_WIDTH,
        y * TILE_HEIGHT,
        total_width * TILE_WIDTH,
        TILE_HEIGHT,
        back_color,
    );

    if bar_width > 0 {
        d.draw_rectangle(
            x * TILE_WIDTH,
            y * TILE_HEIGHT,
            bar_width * TILE_WIDTH,
            TILE_HEIGHT,
            bar_color,
        );
    }

    d.draw_text(
        &format!("{}: {}/{}", name, value, maximum),
        (x + total_width / 2) * TILE_WIDTH,
        y * TILE_HEIGHT,
        TILE_HEIGHT,
        Color::WHITE,
    );
}

fn main_menu(rl: &mut RaylibHandle, thread: &RaylibThread, tcod: &mut Tcod) {
    let img =
        Image::load_image("static/menu_background.png").expect("could not load background image");
    let (w, h) = (img.width(), img.height());
    let img = rl
        .load_texture_from_image(&thread, &img)
        .expect("could not load texture from image");
    img.set_texture_wrap(thread, raylib::consts::TextureWrap::TEXTURE_WRAP_CLAMP);

    while !rl.window_should_close() {
        // show the background image, at twice the regular console resolution
        let pressed_key = rl.get_key_pressed_number();
        let choice = {
            let mut d = rl.begin_drawing(thread);
            d.clear_background(Color::BLACK);
            d.draw_texture_pro(
                &img,
                Rectangle::new(0.0, 0.0, w as f32, h as f32),
                Rectangle::new(0.0, 0.0, 800.0, 640.0),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
            // Game title
            d.draw_text(
                "TOMBS OF THE ANCIENT KINGS",
                (SCREEN_WIDTH / 2) * TILE_WIDTH,
                (SCREEN_HEIGHT / 2 - 4) * TILE_HEIGHT,
                TILE_HEIGHT,
                Color::YELLOW,
            );
            d.draw_text(
                "By Yours Truly",
                (SCREEN_WIDTH / 2) * TILE_WIDTH,
                (SCREEN_HEIGHT / 2 - 2) * TILE_HEIGHT,
                TILE_HEIGHT,
                Color::YELLOW,
            );
            // show options and wait for the player's choice
            let choices = &["Play a new game", "Continue last game", "Quit"];
            menu("", choices, 24, pressed_key, &mut d)
        };
        match choice {
            Some(0) => {
                // new game
                let (mut game, mut objects) = new_game(tcod);
                play_game(rl, thread, tcod, &mut game, &mut objects);
            }
            Some(1) => {
                // load game
                match load_game() {
                    Ok((mut game, mut objects)) => {
                        initialise_fov(tcod, &game.map);
                        play_game(rl, thread, tcod, &mut game, &mut objects);
                    }
                    Err(_e) => {
                        {
                            let mut d = rl.begin_drawing(thread);
                            msgbox("\nNo saved game to load.\n", 24, pressed_key, &mut d);
                        }
                        continue;
                    }
                }
            }
            Some(2) => {
                // quit
                break;
            }
            _ => {}
        }
    }
}

fn msgbox(text: &str, width: i32, pressed_key: Option<u32>, d: &mut RaylibDrawHandle) {
    let options: &[&str] = &[];
    menu(text, options, width, pressed_key, d);
}

fn inventory_menu(
    inventory: &[Object],
    header: &str,
    pressed_key: Option<u32>,
    root: &mut RaylibDrawHandle,
) -> Option<usize> {
    // how a menu with each item of the inventory as an option
    let options = if inventory.len() == 0 {
        vec!["Inventory is empty.".into()]
    } else {
        inventory
            .iter()
            .map(|item| {
                // show additional information, in case it's equipped
                match item.equipment {
                    Some(equipment) if equipment.equipped => {
                        format!("{} (on {})", item.name, equipment.slot)
                    }
                    _ => item.name.clone(),
                }
            })
            .collect()
    };

    let inventory_index = menu(header, &options, INVENTORY_WIDTH, pressed_key, root);

    // if an item was chosen, return it
    if inventory.len() > 0 {
        inventory_index
    } else {
        None
    }
}

fn render_all(
    tcod: &mut Tcod,
    d: &mut RaylibDrawHandle,
    game: &mut Game,
    objects: &mut [Object],
    fov_recompute: bool,
) {
    if fov_recompute {
        let player = &objects[PLAYER];
        tcod.fov
            .compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let visible = tcod.fov.is_in_fov(x, y);
            let wall = game.map[x as usize][y as usize].block_sight;
            let color = match (visible, wall) {
                // outside field of view
                // (false, _) => COLOR_DARK_WALL,
                (false, true) => COLOR_DARK_WALL,
                (false, false) => COLOR_DARK_GROUND,
                // inside fov
                (true, true) => COLOR_DARK_WALL,
                (true, false) => COLOR_DARK_GROUND,
            };

            let explored = &mut game.map[x as usize][y as usize].explored;
            if visible {
                *explored = true;
            }

            if *explored {
                d.draw_rectangle(
                    x * TILE_WIDTH,
                    y * TILE_HEIGHT,
                    TILE_WIDTH,
                    TILE_HEIGHT,
                    color,
                );
            }
        }
    }

    let mut to_draw: Vec<_> = objects
        .iter()
        .filter(|o| {
            tcod.fov.is_in_fov(o.x, o.y)
                || (o.always_visible && game.map[o.x as usize][o.y as usize].explored)
        })
        .collect();
    to_draw.sort_by(|o1, o2| o1.blocks.cmp(&o2.blocks));

    for object in &to_draw {
        if tcod.fov.is_in_fov(object.x, object.y) {
            object.draw(d)
        }
    }

    // GUI
    d.draw_text(
        &format!("Dungeon Level: {} ", game.dungeon_level),
        TILE_WIDTH,
        3 * TILE_HEIGHT,
        TILE_HEIGHT,
        Color::WHITE,
    );

    let hp = objects[PLAYER].fighter.map_or(0, |f| f.hp);
    let max_hp = objects[PLAYER].max_hp(game);
    render_bar(
        d,
        1,
        1 + PANEL_Y,
        BAR_WIDTH,
        "HP",
        hp,
        max_hp,
        Color::RED,
        Color::PURPLE,
    );

    // print the game messages
    let mut y = MSG_HEIGHT as i32;
    for &(ref msg, color) in game.messages.iter().rev() {
        let msg_height = measure_text(msg, TILE_HEIGHT) as f32 / (MSG_WIDTH * TILE_WIDTH) as f32;
        let msg_height = msg_height.ceil() as i32;
        y -= msg_height;
        if y < 0 {
            break;
        }
        let color: Color = color.into();
        d.draw_text(
            msg,
            MSG_X * TILE_WIDTH,
            (PANEL_Y + y) * TILE_HEIGHT,
            TILE_HEIGHT,
            color,
        );
    }

    // Stuff under mouse
    d.draw_text(
        &get_names_under_mouse(tcod.mouse, objects, &tcod.fov),
        TILE_WIDTH,
        PANEL_HEIGHT * TILE_HEIGHT,
        TILE_HEIGHT,
        Color::WHITE,
    );
}

fn target_tile(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],
    max_range: Option<f32>,
) -> Option<(i32, i32)> {
    while !rl.window_should_close() {
        let pos = rl.get_mouse_position();
        let (x, y) = (pos.x as i32 / TILE_WIDTH, pos.y as i32 / TILE_HEIGHT);
        let in_fov = (x < MAP_WIDTH) && (y < MAP_HEIGHT) && tcod.fov.is_in_fov(x, y);
        let in_range = max_range.map_or(true, |range| objects[PLAYER].distance(x, y) <= range);
        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
            && in_fov
            && in_range
        {
            return Some((x, y));
        }

        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON) {
            return None;
        }
        // ...
        let mut d = rl.begin_drawing(thread);
        render_all(tcod, &mut d, game, objects, false);
    }
    None
}

fn target_monster(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],
    max_range: Option<f32>,
) -> Option<usize> {
    match target_tile(rl, thread, tcod, game, objects, max_range) {
        Some((x, y)) => {
            // return the first clicked monster, otherwise continue looping
            for (id, obj) in objects.iter().enumerate() {
                if obj.pos() == (x, y) && obj.fighter.is_some() && id != PLAYER {
                    return Some(id);
                }
            }
        }
        None => return None,
    }
    None
}

fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert!(first_index != second_index);
    let split_at_index = first_index.max(second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}

fn menu<T: AsRef<str>>(
    header: &str,
    options: &[T],
    width: i32,
    pressed_key: Option<u32>,
    d: &mut RaylibDrawHandle,
) -> Option<usize> {
    assert!(
        options.len() <= 26,
        "Cannot have a menu with more than 26 options."
    );

    // calculate total height for the header (after auto-wrap) and one line per option
    let header_height = measure_text(header, TILE_HEIGHT) / (width * TILE_WIDTH);
    let header_height = if header.is_empty() {
        0
    } else {
        header_height.max(1)
    };
    let height = options.len() as i32 + header_height;

    let x = SCREEN_WIDTH / 2 - width / 2;
    let y = SCREEN_HEIGHT / 2 - height / 2;
    let (x, y) = (x * TILE_WIDTH, y * TILE_HEIGHT);
    d.draw_text(header, x, y, TILE_HEIGHT, Color::WHITE);

    for (index, option_text) in options.iter().enumerate() {
        let menu_letter = (b'a' + index as u8) as char;
        let text = format!("({}) {}", menu_letter, option_text.as_ref());
        d.draw_text(
            &text,
            x,
            y + (header_height + index as i32) * TILE_HEIGHT,
            TILE_HEIGHT,
            Color::WHITE,
        );
    }

    if let Some(pressed_key) = pressed_key {
        dbg!(pressed_key);
        use std::num::Wrapping;
        let index = Wrapping(pressed_key) - Wrapping('a' as u32);
        let index: u32 = index.0;
        if (index as usize) < options.len() {
            Some(index as usize)
        } else {
            None
        }
    } else {
        None
    }
}

fn save_game(game: &Game, objects: &[Object]) -> Result<(), Box<dyn Error>> {
    let save_data = serde_json::to_string(&(game, objects))?;
    let mut file = File::create("savegame")?;
    file.write_all(save_data.as_bytes())?;
    Ok(())
}

fn load_game() -> Result<(Game, Vec<Object>), Box<dyn Error>> {
    let mut json_save_state = String::new();
    let mut file = File::open("savegame")?;
    file.read_to_string(&mut json_save_state)?;
    let result = serde_json::from_str::<(Game, Vec<Object>)>(&json_save_state)?;
    Ok(result)
}
