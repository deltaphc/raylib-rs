extern crate raylib;

use raylib::prelude::*;
use structopt::StructOpt;

mod options;

struct Game {
    game_over: bool,
    pause: bool,
    victory: bool,
    player: Player,
}

#[derive(Default)]
struct Player {
    position: Vector2,
    speed: Vector2,
    acceleration: f32,
    rotation: f32,
    collider: Vector3,
    color: Color,
}

impl Default for Game {
    fn default() -> Game {
        let game_over = false;
        let pause = false;
        let victory = false;

        let player = Player::default();

        Game {
            game_over,
            pause,
            victory,
            player
        }
    }
}

const SHIP_HEIGHT : f32 = 10f32 / 0.363970f32;
const PLAYER_SPEED : f32 = 6f32;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Asteroids");
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
    let (width, height) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);

    game.player.position = Vector2::new(width / 2f32, height / 2f32 - (SHIP_HEIGHT / 2f32));
    game.player.collider = Vector3::new(game.player.position.x + game.player.rotation.to_radians().sin() * (SHIP_HEIGHT / 2.5),
                                        game.player.position.y - game.player.rotation.to_radians().cos() * (SHIP_HEIGHT / 2.5),
                                        12f32);
    game.player.color = Color::MAROON;
}

fn update_game(game: &mut Game, rl: &RaylibHandle) {
    use raylib::consts::KeyboardKey::*;
    if !game.game_over {
        if rl.is_key_pressed(KEY_P) {
            game.pause = !game.pause;
        }

        if !game.pause {
            if rl.is_key_down(KEY_LEFT) {
                game.player.rotation -= 5f32;
            }
            if rl.is_key_down(KEY_RIGHT) {
                game.player.rotation += 5f32;
            }

            game.player.speed.x = game.player.rotation.to_radians().sin() * PLAYER_SPEED;
            game.player.speed.y = game.player.rotation.to_radians().cos() * PLAYER_SPEED;

            if rl.is_key_down(KEY_UP) {
                if game.player.acceleration < 1f32 {
                    game.player.acceleration += 0.04;
                }
            }
            else {
                if game.player.acceleration > 0f32 {
                    game.player.acceleration -= 0.02;
                }
                else if game.player.acceleration < 0f32 {
                    game.player.acceleration = 0f32;
                }
            }

            if rl.is_key_down(KEY_DOWN) {
                if game.player.acceleration > 0f32 {
                    game.player.acceleration -= 0.04;
                }
                else if game.player.acceleration < 0f32 {
                    game.player.acceleration = 0f32;
                }
            }

            game.player.position.x += game.player.speed.x * game.player.acceleration;
            game.player.position.y -= game.player.speed.y * game.player.acceleration;

            let (width, height) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);

            if game.player.position.x > width + SHIP_HEIGHT {
                game.player.position.x = -SHIP_HEIGHT;
            }
            else if game.player.position.x < -SHIP_HEIGHT {
                 game.player.position.x = width + SHIP_HEIGHT;
            }

            if game.player.position.y > height + SHIP_HEIGHT {
                game.player.position.y = -SHIP_HEIGHT;
            }
            else if game.player.position.y < -SHIP_HEIGHT {
                game.player.position.y = height + SHIP_HEIGHT;
            }
        }
    }
    else {
        if rl.is_key_pressed(KEY_ENTER) {
            init_game(game, rl);
            game.game_over = false;
        }
    }
}

fn draw_game(game: &Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let (width, height) = (rl.get_screen_width() as i32, rl.get_screen_height() as i32);
    let mut d = rl.begin_drawing(thread);

    let half_width = width / 2;
    let half_height = height / 2;

    d.clear_background(Color::RAYWHITE);

    if !game.game_over {
        let cosf = f32::cos(game.player.rotation.to_radians());
        let sinf = f32::sin(game.player.rotation.to_radians());
        let v1 = Vector2::new(game.player.position.x + sinf * SHIP_HEIGHT,game.player.position.y - cosf * SHIP_HEIGHT);
        let v2 = Vector2::new(game.player.position.x - cosf * 10f32, game.player.position.y - sinf * 10f32);
        let v3 = Vector2::new(game.player.position.x + cosf * 10f32, game.player.position.y + sinf * 10f32);
        d.draw_triangle(v1, v2, v3, game.player.color);

        if game.victory {
            d.draw_text("VICTORY", half_width - measure_text("VICTORY", 20), half_height, 20, Color::LIGHTGRAY);
        }

        if game.pause {
            d.draw_text("GAME PAUSED", half_width - measure_text("GAME PAUSED", 40), half_height - 40, 40, Color::GRAY);
        }
    }
    else {
        d.draw_text("PRESS [ENTER] TO PLAY AGAIN", half_width - measure_text("PRESS [ENTER] TO PLAY AGAIN", 20), half_height - 50, 20, Color::GRAY);
    }

}
