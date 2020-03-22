use rand::prelude::*;
use raylib::prelude::*;
use shipyard::prelude::*;

const ARENA_WIDTH: f32 = 640.0;
const ARENA_HEIGHT: f32 = 480.0;
// Bounds where paddles live
const ARENA_BOUNDS: Rectangle = Rectangle::new(
    ARENA_WIDTH / 6.0,
    0.0,
    ARENA_WIDTH - (2.0 * ARENA_WIDTH / 6.0),
    ARENA_HEIGHT,
);
const PADDLE_SIZE: Rectangle = Rectangle::new(0.0, 0.0, 10.0, 50.0);
const WALL_SIZE: Rectangle = Rectangle::new(0.0, 0.0, ARENA_WIDTH, 50.0);
const GOAL_SIZE: Rectangle = Rectangle::new(0.0, 0.0, 50.0, ARENA_HEIGHT);
const BALL_RADIUS: f32 = 5.0;

pub trait RectExt: std::borrow::BorrowMut<Rectangle> + std::borrow::Borrow<Rectangle> {
    fn center_at(&self, pos: &Vector2) -> Rectangle {
        let r = self.borrow();
        Rectangle::new(
            pos.x - r.width / 2.0,
            pos.y - r.height / 2.0,
            r.width,
            r.height,
        )
    }

    fn center(&self) -> Vector2 {
        let r = self.borrow();
        Vector2::new(r.x + r.width / 2.0, r.y + r.height / 2.0)
    }
}

impl RectExt for Rectangle {}

/// Components
/// - Position, Velocity, Color, AltColor, Shape, Paddle, Ball, GameCtrl (for time management)

mod components {
    use super::*;
    #[derive(Debug)]
    pub struct Position(pub Vector2);
    #[derive(Debug)]
    pub struct Velocity(pub Vector2);

    #[derive(Debug)]
    pub struct MColor(pub Color);

    #[derive(Debug)]
    pub struct AltColor(pub Color);

    #[derive(Debug)]
    pub enum Shape {
        Rect(Rectangle),
        Circle(f32),
    }

    #[derive(Copy, Clone, Debug)]
    pub enum Controller {
        Player,
        AI,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Paddle {
        pub speed: f32,
        pub ctrl: Controller,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Ball(pub f32);

    #[repr(u8)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum CollisionMask {
        Ball = 1,
        Paddle = 1 << 1,
        Wall = 1 << 2,
        Goal = 1 << 3,
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum CollisionResolver {
        Bounce,
        Stop,
        Trigger,
    }

    #[derive(Clone, Debug)]
    pub struct Collider {
        pub rect: Rectangle,
        pub mask: CollisionMask,
        pub collide_with: u8,
        pub resolver: CollisionResolver,
    }

    #[derive(Debug)]
    pub struct CollisionResult {
        pub a: EntityId,
        pub acol: Collider,
        pub b: EntityId,
        pub bcol: Collider,
    }

    impl Collider {
        pub fn ball(rect: Rectangle) -> Collider {
            use CollisionMask::*;
            Collider {
                rect,
                mask: Ball,
                collide_with: Paddle as u8 | Wall as u8 | Goal as u8,
                resolver: CollisionResolver::Bounce,
            }
        }

        pub fn paddle(rect: Rectangle) -> Collider {
            use CollisionMask::*;
            Collider {
                rect,
                mask: Paddle,
                collide_with: Wall as u8 | Ball as u8,
                resolver: CollisionResolver::Stop,
            }
        }

        pub fn wall(rect: Rectangle) -> Collider {
            use CollisionMask::*;
            Collider {
                rect,
                mask: Wall,
                collide_with: Paddle as u8 | Goal as u8,
                resolver: CollisionResolver::Trigger,
            }
        }

        pub fn goal(rect: Rectangle) -> Collider {
            use CollisionMask::*;
            Collider {
                rect,
                mask: Goal,
                collide_with: Ball as u8,
                resolver: CollisionResolver::Trigger,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct GameConfig {
        pub lpaddle: Paddle,
        pub rpaddle: Paddle,
        pub ball: Ball,
    }

    #[derive(Clone, Debug)]
    pub enum GameState {
        Reset(GameConfig),
        Playing,
        Paused,
    }

    #[derive(Debug)]
    pub struct GameCtrl;

    #[derive(Debug, Default)]
    pub struct TimeKeeper {
        pub real_time: f32,
        pub game_time: f32,
        pub game_delta_time: f32,
        pub real_delta_time: f32,
    }

    #[derive(Debug)]
    pub struct GameWindow {
        pub width: f32,
        pub height: f32,
    }

    #[derive(Default)]
    pub struct DrawState {
        pub game_fb: Option<RenderTexture2D>,
    }
}
pub use components::*;

/// Systems
/// DrawSys, MoveSys, InitSys
pub mod systems {
    use super::*;
    #[system(ResetSys)]
    pub fn run(
        mut entities: &mut Entities,
        mut state: Unique<&mut GameState>,
        mut pos: &mut Position,
        mut vel: &mut Velocity,
        mut mcol: &mut MColor,
        mut shape: &mut Shape,
        mut paddle: &mut Paddle,
        mut ball: &mut Ball,
        mut colliders: &mut Collider,
    ) {
        match *state {
            GameState::Reset(ref config) => {
                // Add Left paddles
                let offset = ARENA_BOUNDS.x;
                entities.add_entity(
                    (
                        &mut pos,
                        &mut vel,
                        &mut mcol,
                        &mut shape,
                        &mut colliders,
                        &mut paddle,
                    ),
                    (
                        Position(vec2(offset, ARENA_HEIGHT / 2.0)),
                        Velocity(Vector2::zero()),
                        MColor(Color::WHITE),
                        Shape::Rect(PADDLE_SIZE),
                        Collider::paddle(PADDLE_SIZE),
                        config.lpaddle,
                    ),
                );
                // Add Left paddles
                entities.add_entity(
                    (
                        &mut pos,
                        &mut vel,
                        &mut mcol,
                        &mut shape,
                        &mut colliders,
                        &mut paddle,
                    ),
                    (
                        Position(vec2(ARENA_WIDTH - offset, ARENA_HEIGHT / 2.0)),
                        Velocity(Vector2::zero()),
                        MColor(Color::WHITE),
                        Shape::Rect(PADDLE_SIZE),
                        Collider::paddle(PADDLE_SIZE),
                        config.rpaddle,
                    ),
                );
                // ADD
                // Add Ball
                // let angle: f32 = random::<f32>() * 2.0 * std::f32::consts::PI;
                let angle = (1.0 / 4.0) * std::f32::consts::PI;
                let ball = entities.add_entity(
                    (
                        &mut pos,
                        &mut vel,
                        &mut mcol,
                        &mut shape,
                        &mut colliders,
                        &mut ball,
                    ),
                    (
                        Position(vec2(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0)),
                        Velocity(vec2(angle.cos(), angle.sin()) * config.ball.0),
                        MColor(Color::WHITE),
                        Shape::Circle(BALL_RADIUS),
                        Collider::ball(Rectangle::new(
                            0.0,
                            0.0,
                            2.0 * BALL_RADIUS,
                            2.0 * BALL_RADIUS,
                        )),
                        config.ball,
                    ),
                );

                // ADD Walls
                entities.add_entity(
                    (&mut pos, &mut colliders),
                    (
                        Position(vec2(ARENA_WIDTH / 2.0, WALL_SIZE.height / 2.0)),
                        Collider::wall(WALL_SIZE),
                    ),
                );
                // ADD Walls
                entities.add_entity(
                    (&mut pos, &mut colliders),
                    (
                        Position(vec2(
                            ARENA_WIDTH / 2.0,
                            ARENA_HEIGHT - WALL_SIZE.height / 2.0,
                        )),
                        Collider::wall(WALL_SIZE),
                    ),
                );

                // ADD Goals
                entities.add_entity(
                    (&mut pos, &mut colliders),
                    (
                        Position(vec2(0.0, ARENA_HEIGHT / 2.0)),
                        Collider::goal(GOAL_SIZE),
                    ),
                );

                // ADD Goals
                entities.add_entity(
                    (&mut pos, &mut colliders),
                    (
                        Position(vec2(ARENA_WIDTH, ARENA_HEIGHT / 2.0)),
                        Collider::goal(GOAL_SIZE),
                    ),
                );

                println!("Ball: {:?}", ball);
                *state = GameState::Playing;
            }
            _ => {}
        }
    }

    #[system(TimeKeeperSys)]
    pub fn run(
        rl: Unique<NonSend<&RaylibHandle>>,
        mut time: Unique<&mut TimeKeeper>,
        state: Unique<&GameState>,
    ) {
        let dt = rl.get_frame_time();
        match *state {
            GameState::Playing => {
                time.game_time += dt;
                time.game_delta_time = dt;
            }
            _ => {
                time.game_delta_time = 0.0;
            }
        }
        time.real_time += dt;
        time.real_delta_time = dt;
    }

    #[system(MoveSys)]
    pub fn run(time: Unique<&TimeKeeper>, mut pos: &mut Position, vel: &Velocity) {
        (&mut pos, &vel).iter().for_each(|(p, v)| {
            p.0 = p.0 + (v.0 * time.game_delta_time);
        });
    }

    #[system(CollisionSys)]
    pub fn run(
        entities: &Entities,
        pos: &Position,
        mut collider: &mut Collider,
        mut results: &mut CollisionResult,
    ) {
        // Get rid of all results
        let ents: Vec<_> = (&results).iter().with_id().map(|(id, _)| id).collect();
        for e in ents {
            results.delete(e)
        }

        (&pos, &mut collider).iter().for_each(|(p, c)| {
            c.rect = c.rect.center_at(&p.0);
        });

        let cols: Vec<_> = (&collider).iter().with_id().collect();
        for i in 0..cols.len() {
            let (a, acol) = cols[i];
            for j in (i + 1)..cols.len() {
                let (b, bcol) = cols[j];
                if acol.rect.check_collision_recs(&bcol.rect)
                    && ((acol.collide_with & bcol.mask as u8) != 0)
                {
                    if acol.resolver != CollisionResolver::Trigger {
                        println!(
                            "collision: {:?}, {:?}, {:?}, {:?}",
                            a, b, acol.mask, bcol.mask
                        );
                    }
                    entities.add_component(
                        &mut results,
                        CollisionResult {
                            a,
                            acol: acol.clone(),
                            b,
                            bcol: bcol.clone(),
                        },
                        a,
                    );

                    entities.add_component(
                        &mut results,
                        CollisionResult {
                            b: a,
                            bcol: acol.clone(),
                            a: b,
                            acol: bcol.clone(),
                        },
                        b,
                    );
                }
            }
        }
    }

    #[system(CollisionResolveSys)]
    pub fn run(
        time: Unique<&TimeKeeper>,
        ball: &Ball,
        paddle: &Paddle,
        result: &CollisionResult,
        mut pos: &mut Position,
        mut vel: &mut Velocity,
    ) {
        // Bounces and stay in
        // Walls have no velocity so this doesn't trigger for them
        (&result, &mut pos, &mut vel).iter().for_each(|(r, p, v)| {
            if r.bcol.mask == CollisionMask::Goal {
                println!("scored!");
                p.0 = vec2(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0);
                return;
            }
            // https://gamedev.stackexchange.com/questions/29786/a-simple-2d-rectangle-collision-algorithm-that-also-determines-which-sides-that
            let w = 0.5 * (r.acol.rect.width + r.bcol.rect.width);
            let h = 0.5 * (r.acol.rect.height + r.bcol.rect.height);
            let dx = r.acol.rect.center().x - r.bcol.rect.center().x;
            let dy = r.acol.rect.center().y - r.bcol.rect.center().y;

            let bcenter = r.bcol.rect.center();

            if (dx.abs() <= w && dy.abs() <= h) {
                let wy = w * dy;
                let hx = h * dx;
                // Undo last velocity
                p.0 = p.0 - (v.0 * 2.0 * time.game_delta_time);
                if (wy > hx) {
                    if (wy > -hx) {
                        /* collision at the top */
                        p.0.y = bcenter.y + h + 1.0;
                        v.0.y = -v.0.y;
                    } else {
                        /* on the left */
                        if r.acol.resolver == CollisionResolver::Bounce {
                            p.0.x = bcenter.x - w - 1.0;
                            v.0.x = -v.0.x;
                        }
                    }
                } else {
                    if (wy > -hx) {
                        /* on the right */
                        if r.acol.resolver == CollisionResolver::Bounce {
                            p.0.x = bcenter.x + w + 1.0;
                            v.0.x = -v.0.x;
                        }
                    } else {
                        /* at the bottom */
                        p.0.y = bcenter.y - h - 1.0;
                        v.0.y = -v.0.y;
                    }
                }
            }
        });
    }

    #[system(PaddleControlSys)]
    pub fn run(ball: &Ball, paddle: &Paddle, pos: &Position, mut vel: &mut Velocity) {
        let (_, ball_pos) = (&ball, &pos).iter().next().unwrap();

        (&paddle, &pos, &mut vel).iter().for_each(|(pad, p, v)| {
            // If ball is beyond us then don't do anything
            if !ARENA_BOUNDS.check_collision_point_rec(ball_pos.0) {
                v.0 = Vector2::zero();
                return;
            }
            match pad.ctrl {
                // If ball is behind us, stop all movement
                Controller::AI => {
                    let d = (ball_pos.0.y - p.0.y);
                    if d.abs() < PADDLE_SIZE.height / 3.0 {
                        v.0 = Vector2::zero();
                        return;
                    }
                    v.0 = vec2(0.0, (ball_pos.0.y - p.0.y).signum()) * pad.speed;
                }
                _ => {}
            }
        });
    }

    #[system(DrawSys)]
    pub fn run(
        mut rl: Unique<NonSend<&mut RaylibHandle>>,
        thread: Unique<NonSendSync<&RaylibThread>>,
        window: Unique<&GameWindow>,
        mut dstate: Unique<&mut DrawState>,
        pos: &Position,
        mcolor: &MColor,
        shape: &Shape,
        colliders: &Collider,
        results: &CollisionResult,
    ) {
        let mut frame_buffer = dstate.game_fb.get_or_insert_with(|| {
            rl.load_render_texture(&thread, ARENA_WIDTH as u32, ARENA_HEIGHT as u32)
                .unwrap()
        });

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::PURPLE);
        // Draw game to texture
        {
            let mut d = d.begin_texture_mode(frame_buffer);
            d.clear_background(Color::BLACK);
            (&pos, &mcolor, &shape)
                .iter()
                .for_each(|(p, c, shape)| match shape {
                    Shape::Circle(rad) => {
                        d.draw_circle(p.0.x as i32, p.0.y as i32, *rad, c.0);
                    }
                    Shape::Rect(rect) => {
                        let rect = rect.center_at(&p.0);
                        d.draw_rectangle(
                            rect.x as i32,
                            rect.y as i32,
                            rect.width as i32,
                            rect.height as i32,
                            c.0,
                        );
                    }
                });

            (&colliders,)
                .iter()
                .for_each(|col| d.draw_rectangle_lines_ex(col.rect, 2, Color::PINK));

            (&results,).iter().for_each(|r| {
                let place = r.acol.rect.get_collision_rec(&r.bcol.rect).unwrap();
                d.draw_rectangle_lines_ex(place, 2, Color::RED);
            })
        }
        // Draw texture to full screen.
        let hscale = window.height / frame_buffer.texture.height as f32;
        d.draw_texture_pro(
            frame_buffer.texture(),
            Rectangle::new(
                0.0,
                0.0,
                frame_buffer.texture.width as f32,
                -frame_buffer.texture.height as f32,
            ),
            Rectangle::new(0.0, 0.0, window.width, window.height),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
}
pub use systems::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Pong").build();
    let logo = raylib::prelude::Image::load_image("static/logo.png").unwrap();
    rl.set_window_icon(&logo);
    rl.set_target_fps(60);

    let world = World::new();
    world.add_unique_non_send_sync(rl);
    world.add_unique_non_send_sync(thread);

    world.add_unique(TimeKeeper::default());
    world.add_unique(GameWindow {
        width: 640.0,
        height: 480.0,
    });
    world.add_unique(DrawState::default());
    world.add_unique(GameState::Reset(GameConfig {
        lpaddle: Paddle {
            speed: ARENA_HEIGHT / 6.0,
            ctrl: Controller::AI,
        },
        rpaddle: Paddle {
            speed: ARENA_HEIGHT / 5.0,
            ctrl: Controller::AI,
        },
        ball: Ball(ARENA_WIDTH / 2.0),
    }));

    while !world
        .borrow::<Unique<NonSend<&RaylibHandle>>>()
        .window_should_close()
    {
        world.run_system::<ResetSys>();
        world.run_system::<PaddleControlSys>();

        world.run_system::<TimeKeeperSys>();
        world.run_system::<CollisionSys>();
        world.run_system::<CollisionResolveSys>();
        world.run_system::<MoveSys>();
        world.run_system::<DrawSys>();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bitwise_because_im_dumb() {
        assert_eq!(2, 1 << 1);
    }
}
