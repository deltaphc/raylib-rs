extern crate raylib;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

const PLAYER_MAX_LIFE: i32 = 5;
const LINES_OF_BRICKS: usize = 5;
const BRICKS_PER_LINE: usize = 20;

#[derive(Default)]
struct Player {
    pub position: Vector2,
    pub size: Vector2,
    pub life: i32,
}

#[derive(Default)]
struct Ball {
    position: Vector2,
    speed: Vector2,
    radius: i32,
    active: bool,
}

#[derive(Default)]
struct Brick {
    position: Vector2,
    active: bool,
}

struct Game {
    game_over: bool,
    pause: bool,
    player: Player,
    ball: Ball,
    bricks: Vec<Vec<Brick>>,
    brick_size: Vector2,
}

impl Default for Game {
    fn default() -> Game {
        let game_over = false;
        let pause = false;

        let player = Player::default();
        let ball = Ball::default();
        let mut bricks = Vec::new();
        for _ in 0..LINES_OF_BRICKS {
            let mut v = Vec::new();
            for _ in 0..BRICKS_PER_LINE {
                v.push(Brick::default());
            }
            bricks.push(v);
        }
        let brick_size = Vector2::default();
        Game {
            game_over,
            pause,
            player,
            ball,
            brick_size,
            bricks,
        }
    }
}

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Arkanoid");
    let (_w, _h) = (opt.width, opt.height);

    let _game_over = false;
    let _pause = false;

    let mut game = Game::default();

    init_game(&mut game, &rl);

    while !rl.window_should_close() {
        update_game(&mut game, &rl);
        draw_game(&game, &mut rl, &thread);
    }
}

fn init_game(game: &mut Game, rl: &RaylibHandle) {
    let (w, h) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);
    game.brick_size = Vector2::new(rl.get_screen_width() as f32 / BRICKS_PER_LINE as f32, 40.0);

    // Initialize player
    game.player.position = Vector2::new(
        rl.get_screen_width() as f32 / 2.0,
        rl.get_screen_height() as f32 * 7.0 / 8.0,
    );
    game.player.size = Vector2::new(rl.get_screen_width() as f32 / 10.0, 20.0);
    game.player.life = PLAYER_MAX_LIFE;

    // Initialize ball
    game.ball.position = Vector2::new(w / 2.0, h * 7.0 / 7.0 - 30.0);
    game.ball.speed = Vector2::default();
    game.ball.radius = 7;
    game.ball.active = false;

    // Initialize bricks
    let initial_down_position = 50.0;

    for i in 0..LINES_OF_BRICKS {
        for j in 0..BRICKS_PER_LINE {
            game.bricks[i][j].position = Vector2::new(
                j as f32 * game.brick_size.x + game.brick_size.x / 2.0,
                i as f32 * game.brick_size.y + initial_down_position,
            );
            game.bricks[i][j].active = true;
        }
    }
}

fn update_game(game: &mut Game, rl: &RaylibHandle) {
    use raylib::consts::KeyboardKey::*;
    let (w, h) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);

    if !game.game_over {
        if rl.is_key_pressed(KEY_P) {
            game.pause = !game.pause;
        }

        if !game.pause {
            // player movement logic
            if rl.is_key_down(KEY_LEFT) {
                game.player.position.x -= 5.0;
            }
            if game.player.position.x - game.player.size.x / 2.0 <= 0.0 {
                game.player.position.x = game.player.size.x / 2.0;
            }
            if rl.is_key_down(KEY_RIGHT) {
                game.player.position.x += 5.0;
            }
            if game.player.position.x + game.player.size.x / 2.0 >= w {
                game.player.position.x = w - game.player.size.x / 2.0;
            }

            // Ball launching logic
            if !game.ball.active {
                if rl.is_key_pressed(KEY_SPACE) {
                    game.ball.active = true;
                    game.ball.speed = Vector2::new(0.0, -5.0);
                }
            }

            // Ball movement logic
            if game.ball.active {
                game.ball.position += game.ball.speed;
            } else {
                game.ball.position = Vector2::new(game.player.position.x, h * 7.0 / 8.0 - 30.0);
            }

            // Collision logic: ball vs walls
            if game.ball.position.x + game.ball.radius as f32 >= w
                || game.ball.position.x - game.ball.radius as f32 <= 0.0
            {
                game.ball.speed.x *= -1.0;
            }
            if game.ball.position.y - game.ball.radius as f32 <= 0.0 {
                game.ball.speed.y *= -1.0;
            }
            if game.ball.position.y + game.ball.radius as f32 >= h {
                game.ball.speed = Vector2::default();
                game.ball.active = false;
                game.player.life -= 1;
            }

            // Collision logic: ball vs player
            let r = Rectangle::new(
                game.player.position.x - game.player.size.x / 2.0,
                game.player.position.y - game.player.size.y / 2.0,
                game.player.size.x,
                game.player.size.y,
            );
            if r.check_collision_circle_rec(game.ball.position, game.ball.radius as f32) {
                if game.ball.speed.y > 0.0 {
                    game.ball.speed.y *= -1.0;
                    game.ball.speed.x = (game.ball.position.x - game.player.position.x)
                        / (game.player.size.x / 2.0)
                        * 5.0;
                }
            }

            // Collision logic: ball vs bricks
            for i in 0..LINES_OF_BRICKS {
                for j in 0..BRICKS_PER_LINE {
                    if game.bricks[i][j].active {
                        // Hit below
                        if (game.ball.position.y - game.ball.radius as f32
                            <= game.bricks[i][j].position.y + game.brick_size.y / 2.0)
                            && (game.ball.position.y - game.ball.radius as f32
                                > game.bricks[i][j].position.y
                                    + game.brick_size.y / 2.0
                                    + game.ball.speed.y)
                            && ((game.ball.position.x - game.bricks[i][j].position.x).abs()
                                < game.brick_size.x / 2.0 + game.ball.radius as f32 * 2.0 / 3.0)
                            && game.ball.speed.y < 0.0
                        {
                            game.bricks[i][j].active = false;
                            game.ball.speed.y *= -1.0;
                        }
                        // Hit above
                        else if game.ball.position.y + game.ball.radius as f32
                            >= game.bricks[i][j].position.y - game.brick_size.y / 2.0
                            && (game.ball.position.y + game.ball.radius as f32)
                                .partial_cmp(
                                    &(game.bricks[i][j].position.y - game.brick_size.y / 2.0
                                        + game.ball.speed.y),
                                )
                                .unwrap()
                                == std::cmp::Ordering::Less
                            && (game.ball.position.x - game.bricks[i][j].position.x).abs()
                                < game.brick_size.x / 2.0 + game.ball.radius as f32 * 2.0 / 3.0
                            && game.ball.speed.y > 0.0
                        {
                            game.bricks[i][j].active = false;
                            game.ball.speed.y *= -1.0;
                        }
                        // Hit Left
                        else if ((game.ball.position.x + game.ball.radius as f32)
                            >= (game.bricks[i][j].position.x - game.brick_size.x / 2.0))
                            && ((game.ball.position.x + game.ball.radius as f32)
                                < (game.bricks[i][j].position.x - game.brick_size.x / 2.0
                                    + game.ball.speed.x))
                            && (((game.ball.position.y - game.bricks[i][j].position.y).abs())
                                < (game.brick_size.y / 2.0 + game.ball.radius as f32 * 2.0 / 3.0))
                            && (game.ball.speed.x > 0.0)
                        {
                            game.bricks[i][j].active = false;
                            game.ball.speed.x *= -1.0;
                        }
                        // Hit right
                        else if ((game.ball.position.x - game.ball.radius as f32)
                            <= (game.bricks[i][j].position.x + game.brick_size.x / 2.0))
                            && ((game.ball.position.x - game.ball.radius as f32)
                                > (game.bricks[i][j].position.x
                                    + game.brick_size.x / 2.0
                                    + game.ball.speed.x))
                            && (((game.ball.position.y - game.bricks[i][j].position.y).abs())
                                < (game.brick_size.y / 2.0 + game.ball.radius as f32 * 2.0 / 3.0))
                            && (game.ball.speed.x < 0.0)
                        {
                            game.bricks[i][j].active = false;
                            game.ball.speed.x *= -1.0;
                        }
                    }
                }
            }

            // Game over life
            if game.player.life <= 0 {
                game.game_over = true;
            } else {
                game.game_over = true;
                for i in 0..LINES_OF_BRICKS {
                    for j in 0..BRICKS_PER_LINE {
                        if game.bricks[i][j].active {
                            game.game_over = false;
                        }
                    }
                }
            }
        }
    } else {
        if rl.is_key_pressed(KEY_ENTER) {
            init_game(game, rl);
            game.game_over = false;
        }
    }
}

fn draw_game(game: &Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let (w, h) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::RAYWHITE);
    if !game.game_over {
        // Draw player bar
        d.draw_rectangle(
            (game.player.position.x - game.player.size.x / 2.0) as i32,
            (game.player.position.y - game.player.size.y / 2.0) as i32,
            game.player.size.x as i32,
            game.player.size.y as i32,
            Color::BLACK,
        );

        // Draw player lives
        for i in 0..game.player.life {
            d.draw_rectangle(20 + 30 * i, h as i32 - 30, 35, 10, Color::LIGHTGRAY);
        }

        // Draw ball
        d.draw_circle_v(game.ball.position, game.ball.radius as f32, Color::MAROON);

        // Draw bricks
        for i in 0..LINES_OF_BRICKS {
            for j in 0..BRICKS_PER_LINE {
                if game.bricks[i][j].active {
                    if (i + j) % 2 == 0 {
                        d.draw_rectangle(
                            (game.bricks[i][j].position.x - game.brick_size.x / 2.0) as i32,
                            (game.bricks[i][j].position.y - game.brick_size.y / 2.0) as i32,
                            game.brick_size.x as i32,
                            game.brick_size.y as i32,
                            Color::GRAY,
                        );
                    } else {
                        d.draw_rectangle(
                            (game.bricks[i][j].position.x - game.brick_size.x / 2.0) as i32,
                            (game.bricks[i][j].position.y - game.brick_size.y / 2.0) as i32,
                            game.brick_size.x as i32,
                            game.brick_size.y as i32,
                            Color::DARKGRAY,
                        );
                    }
                }
            }
        }

        if game.pause {
            d.draw_text(
                "Game Pause",
                (w / 2.0) as i32 - d.measure_text("Game Paused", 40) / 2,
                (h / 2.0 - 40.0) as i32,
                40,
                Color::GRAY,
            );
        }
    } else {
        d.draw_text(
            "PRESS [ENTER] TO PLAY AGAIN",
            (w / 2.0) as i32 - d.measure_text("PRESS [ENTER] TO PLAY AGAIN", 20) / 2,
            (h / 2.0) as i32 - 50,
            20,
            Color::GRAY,
        );
    }
}
