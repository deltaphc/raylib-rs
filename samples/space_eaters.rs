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
    speed: f64,
    next_frame: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Health {
    health: i32,
    inv_time: f64,
    inv_dur: f64,
}

impl Health {
    fn is_inv(&self, time: f64) -> bool {
        self.inv_time != 0.0 && self.inv_time > time
    }
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
        Damage { amount: 1 }
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
            next_spawn: 0.1,
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

trait Lerp: Clone + Copy + std::fmt::Debug + Send + Sync + 'static {
    fn lerp(start: Self, end: Self, f: f64, t: f64) -> Self;
}

#[derive(Clone, Copy, Debug)]
struct TimedLerp<T: Lerp> {
    start: T,
    end: T,
    now: f64,
    clamp: Option<f64>,
    frequency: f64,
    kill_at: Option<f64>,
}

impl<T: Lerp> TimedLerp<T> {
    fn val(val: T, now: f64) -> Self {
        Self {
            start: val,
            end: val,
            now,
            clamp: None,
            frequency: 0.0,
            kill_at: None,
        }
    }

    fn val_at(&self, time: f64) -> T {
        T::lerp(
            self.start,
            self.end,
            self.frequency,
            self.clamp
                .unwrap_or(std::f64::INFINITY)
                .min(time - self.now),
        )
    }

    fn is_dead(&self, time: f64) -> bool {
        if let Some(kill_at) = self.kill_at {
            return kill_at < time;
        }
        return false;
    }
}

impl<T: Lerp + Default> TimedLerp<T> {
    fn dur(dur: f64, now: f64) -> Self {
        Self {
            start: T::default(),
            end: T::default(),
            now,
            clamp: None,
            frequency: 0.0,
            kill_at: Some(now + dur),
        }
    }
}

struct TimedLerpTracker<T: Lerp> {
    add_buf: Vec<(Entity, TimedLerp<T>)>,
    remove_buf: Vec<Entity>,
}

impl<T: Lerp> TimedLerpTracker<T> {
    fn new() -> Self {
        Self {
            add_buf: Vec::new(),
            remove_buf: Vec::new(),
        }
    }

    fn add(&mut self, ent: (Entity, TimedLerp<T>)) {
        self.add_buf.push(ent);
    }

    fn update(&mut self, world: &mut World, time: f64) -> Vec<Entity> {
        for (ent, comp) in self.add_buf.drain(..) {
            world.add_component(ent, comp);
        }

        let query = <Read<TimedLerp<T>>>::query();
        for (ent, comp) in query.iter_entities(world) {
            if comp.is_dead(time) {
                self.remove_buf.push(ent);
            }
        }

        let things: Vec<_> = self.remove_buf.drain(..).collect();

        for ent in &things {
            world.remove_component::<TimedLerp<T>>(*ent);
        }
        things
    }
}

impl Lerp for Color {
    fn lerp(start: Self, end: Self, frequency: f64, time: f64) -> Self {
        let channel = |a, b, f: f64, t: f64| (b as f64 - a as f64) * (f * t).sin() + a as f64;

        Color {
            r: channel(start.r, end.r, frequency, time) as u8,
            g: channel(start.g, end.g, frequency, time) as u8,
            b: channel(start.b, end.b, frequency, time) as u8,
            a: channel(start.a, end.a, frequency, time) as u8,
        }
    }
}

type Tint = TimedLerp<Color>;

impl Tint {
    fn color(color: Color, now: f64) -> Tint {
        Tint::val(color, now)
    }

    fn color_over(start: Color, end: Color, over: f64, now: f64, kill_at: Option<f64>) -> Tint {
        Tint {
            start,
            end,
            now,
            clamp: Some(std::f64::consts::PI / 2.0),
            frequency: 1.0,
            kill_at,
        }
    }

    fn color_at(&self, time: f64) -> Color {
        self.val_at(time)
    }

    fn invincibility(time: f64, now: f64) -> Tint {
        Tint {
            start: Color::WHITE,
            end: Color::BLACK,
            now,
            clamp: None,
            frequency: 25.0,
            kill_at: Some(time),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Lifetime;
impl Lerp for Lifetime {
    fn lerp(start: Self, end: Self, frequency: f64, time: f64) -> Self {
        Lifetime
    }
}
type ParticleLifetime = TimedLerp<Lifetime>;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Bounds {
    StayIn,
    Destroy,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Hidden(bool);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Shape {
    Circle(f32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Particle;

struct ParticleSystem {
    sys: Box<dyn PSImpl>,
}

impl ParticleSystem {
    fn update(
        &mut self,
        rl: &mut RaylibHandle,
        world: &mut World,
        ctrl: &mut ParticleControler,
    ) -> bool {
        self.sys.update(rl, world, ctrl)
    }
}

trait PSImpl: Send + Sync {
    fn update(
        &mut self,
        rl: &mut RaylibHandle,
        world: &mut World,
        ctrl: &mut ParticleControler,
    ) -> bool;
}

struct PSTraceBox {}

impl PSImpl for PSTraceBox {
    fn update(
        &mut self,
        rl: &mut RaylibHandle,
        world: &mut World,
        ctrl: &mut ParticleControler,
    ) -> bool {
        use raylib::consts::MouseButton::*;
        if rl.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
            let now = rl.get_time();
            let m_pos = rl.get_mouse_position();
            // heck if i know what I don't have to divide by PS here
            // let m_pos = vec2(m_pos.x / PS, m_pos.y / PS);
            let entity = ctrl.pool.take();
            {
                let mut pos = world.get_component_mut::<Position>(entity).unwrap();
                pos.0 = m_pos;
            }
            {
                let mut shape = world.get_component_mut::<Shape>(entity).unwrap();
                *shape = Shape::Circle(10.0);
            }
            {
                let mut tint = world.get_component_mut::<Tint>(entity).unwrap();
                *tint = Tint::color_over(Color::RED, Color::ORANGE, 1.0, now, None);
            }

            {
                world.add_component(entity, ParticleLifetime::dur(1.0, now));
            }

            world.add_tag(entity, Hidden(false));
        }
        false
    }
}

struct ParticleControler {
    pool: Pool<Entity>,
    tracker: TimedLerpTracker<Lifetime>,
}

impl ParticleControler {
    fn new(init: &[Entity]) -> Self {
        Self {
            pool: Pool::new(init),
            tracker: TimedLerpTracker::new(),
        }
    }

    fn put(&mut self, ents: &[Entity]) {
        let mut i = 0;
        for e in ents {
            for (j, slot) in (&mut self.pool.objects[i..]).iter_mut().enumerate() {
                if slot.is_none() {
                    slot.replace(*e);
                    i = j;
                }
            }
        }
    }
}

struct Pool<T: Copy> {
    objects: Vec<Option<T>>,
}

impl<T: Copy> Pool<T> {
    fn new(init: &[T]) -> Pool<T> {
        let objects = init.into_iter().map(|obj| Some(*obj)).collect();
        Pool { objects }
    }

    fn take(&mut self) -> T {
        let t = self
            .objects
            .iter_mut()
            .filter(|t| t.is_some())
            .next()
            .expect("pool out of objects");
        t.take().unwrap()
    }
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
            next_frame: 0.0,
        },
        player_bullet_1: 32,
        green_enemy_anim: Animation {
            frames: vec![3, 3 + 16],
            current: 0,
            speed: 1.0,
            next_frame: 0.0,
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

    let mut tint_tracker = TimedLerpTracker::new();

    let (s_extents, s_indices) = sprite_extents();
    let s_sheet = load_sprite_sheet(&mut rl, &thread);

    // particle system stuff
    let particles = world.insert(
        (Hidden(true),),
        (0..1000).map(|_| {
            (
                Position(Vector2::zero()),
                Velocity(Vector2::zero()),
                Shape::Circle(0.0),
                Tint::color(Color::WHITE, rl.get_time()),
            )
        }),
    );
    let mut particle_ctrl = ParticleControler::new(particles);
    let mut particle_systems = vec![ParticleSystem {
        sys: Box::new(PSTraceBox {}),
    }];

    // I know sprite could be a tag, but legion's documentation is so
    // terrible that I have no idea how to use tags right.
    world.insert(
        (Bounds::StayIn, Hidden(false)),
        (0..1).map(|_| {
            (
                Position(vec2(ARENA_WIDTH as f32 / 2.0, ARENA_HEIGHT as f32 / 2.0)),
                Velocity(Vector2::zero()),
                s_indices.player_anim.clone(),
                Sprite(0),
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
                (Bounds::Destroy, Hidden(false)),
                (0..1).map(|_| {
                    (
                        Position(vec2(
                            (ARENA_WIDTH as f64 / 2.0 * time.cos() + ARENA_WIDTH as f64 / 2.0)
                                as f32,
                            1.0,
                        )),
                        Velocity(vec2(0.0, 1.0) * enemy.speed),
                        enemy.anim(&s_indices),
                        Sprite(0),
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
                anim.current = 0;
            } else if right {
                anim.current = 2;
            } else {
                anim.current = 1;
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
            world.insert(
                (Bounds::Destroy, Hidden(false)),
                (0..1).map(|_| player_shoot),
            );
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
                let (a_pos, a_col, a_dmg, a_health) = &entities[i];
                let a_rec = a_col.aabb.move_to(a_pos.x, a_pos.y);
                if a_health.is_inv(time) {
                    continue;
                }
                for j in (i + 1)..entities.len() {
                    let (b_pos, b_col, b_dmg, b_health) = &entities[j];
                    let b_rec = b_col.aabb.move_to(b_pos.x, b_pos.y);
                    if b_health.is_inv(time) {
                        continue;
                    }
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
                        // println!("took damage, {}, {:?}", dmg, health);
                    }
                }
            }
        }

        // Do particle system stuff
        let mut to_remove = Vec::new();
        for (i, sys) in particle_systems.iter_mut().enumerate() {
            if sys.update(&mut rl, &mut world, &mut particle_ctrl) {
                to_remove.push(i)
            }
        }
        for i in to_remove {
            particle_systems.swap_remove(i);
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
                    if tint.is_dead(time) {
                        // Pretty sure they are different tints in this case.
                        *tint = Tint::invincibility(h.inv_time, time);
                    }
                } else {
                    tint_tracker.add((ent, Tint::invincibility(h.inv_time, time)));
                }
            }
        }

        // Animate stuff
        let query = <Write<Animation>>::query();
        for mut anim in query.iter(&mut world) {
            if anim.next_frame < time {
                anim.current = (anim.current + 1) % anim.frames.len();
                anim.next_frame = time + 1.0 / anim.speed;
            }
        }

        // Set Sprite Animations
        let query = <(Read<Animation>, Write<Sprite>)>::query();
        for (anim, mut sprite) in query.iter(&mut world) {
            sprite.0 = anim.frames[anim.current];
        }

        // Draw Stuff
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            // Draw sprites
            let query = <(Read<Position>, Read<Sprite>, TryRead<Tint>)>::query()
                .filter(tag_value(&Hidden(false)));
            for (pos, sprite, tint) in query.iter(&mut world) {
                let tint = tint.map(|t| t.color_at(time)).unwrap_or(Color::WHITE);
                d.draw_texture_pro(
                    &s_sheet,
                    s_extents[sprite.0],
                    s_extents[sprite.0].move_to(pos.x, pos.y).project(PS),
                    vec2(0.0, 0.0),
                    0.0,
                    tint,
                );
            }

            // Draw shapes
            let query = <(Read<Position>, Read<Shape>, TryRead<Tint>)>::query()
                .filter(tag_value(&Hidden(false)));
            for (pos, shape, tint) in query.iter(&mut world) {
                let tint = tint.map(|t| t.color_at(time)).unwrap_or(Color::WHITE);
                use Shape::*;
                match *shape {
                    Circle(radius) => {
                        d.draw_circle(pos.x as i32, pos.y as i32, radius, tint);
                    }
                }
            }
        }

        // Draw explosion

        // Cleanup
        tint_tracker.update(&mut world, time);
        let dead = particle_ctrl.tracker.update(&mut world, time);
        if dead.len() > 0 {
            particle_ctrl.put(&dead);
        }

        // Make sure to call this last
        for ent in destroy_buf.drain(..) {
            world.delete(ent);
        }

        // for ext in &s_extents {
        //     d.draw_texture_rec(&s_sheet, ext, vec2(ext.x, ext.y), Color::WHITE);
        // }
    }
}

#[cfg(test)]
mod tests {
    use legion::prelude::*;

    struct Position;
    #[test]
    fn check_legion() {
        let mut universe = Universe::new();
        let mut world = universe.create_world();

        let e = world.insert((), (0..1).map(|_| (Position,)))[0];

        for _ in <Read<Position>>::query().iter(&mut world) {
            assert_eq!(1, 1);
        }

        world.remove_component::<Position>(e);

        for _ in <Read<Position>>::query().iter(&mut world) {
            assert_eq!(1, 2);
        }
    }
}
