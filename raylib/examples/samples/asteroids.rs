extern crate raylib;

use raylib::prelude::*;
use structopt::StructOpt;

mod options;

struct Game {
    game_over: bool,
    pause: bool,
    victory: bool,
    player: Player,
    big_meteors: Vec<Meteor>,
    medium_meteors: Vec<Meteor>,
    small_meteors: Vec<Meteor>,
    shots: Vec<Shoot>,
    destroyed_meteor_count: u32,
    medium_meteor_count: u32,
    small_meteor_count: u32,
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

#[derive(Default)]
struct Meteor {
    position: Vector2,
    speed: Vector2,
    radius: f32,
    active: bool,
    color: Color,
}

#[derive(Default)]
struct Shoot {
    position: Vector2,
    speed: Vector2,
    radius: f32,
    rotation: f32,
    life_spawn: u8,
    active: bool,
    color: Color,
}

impl Default for Game {
    fn default() -> Game {
        let game_over = false;
        let pause = false;
        let victory = false;

        let player = Player::default();
        let mut big_meteors = Vec::new();
        for _ in 0..MAX_BIG_METEORS {
            big_meteors.push(Meteor::default());
        }
        let mut medium_meteors = Vec::new();
        for _ in 0..MAX_MEDIUM_METEORS {
            medium_meteors.push(Meteor::default());
        }
        let mut small_meteors = Vec::new();
        for _ in 0..MAX_SMALL_METEORS {
            small_meteors.push(Meteor::default());
        }
        let mut shots = Vec::new();
        for _ in 0..MAX_SHOTS {
            shots.push(Shoot::default());
        }

        let destroyed_meteor_count = 0;
        let medium_meteor_count = 0;
        let small_meteor_count = 0;

        Game {
            game_over,
            pause,
            victory,
            player,
            big_meteors,
            medium_meteors,
            small_meteors,
            shots,
            destroyed_meteor_count,
            medium_meteor_count,
            small_meteor_count,
        }
    }
}

const SHIP_HEIGHT: f32 = 10f32 / 0.363970f32;
const PLAYER_SPEED: f32 = 6f32;
const MAX_BIG_METEORS: usize = 4;
const MAX_MEDIUM_METEORS: usize = 8;
const MAX_SMALL_METEORS: usize = 16;
const METEORS_SPEED: f32 = 2f32;
const MAX_SHOTS: usize = 10;

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
    let half_width = width / 2.0;
    let half_height = height / 2.0;

    game.player.position = Vector2::new(half_width, half_height - (SHIP_HEIGHT / 2f32));
    game.player.acceleration = 0f32;
    game.player.collider = Vector3::new(
        game.player.position.x + game.player.rotation.to_radians().sin() * (SHIP_HEIGHT / 2.5),
        game.player.position.y - game.player.rotation.to_radians().cos() * (SHIP_HEIGHT / 2.5),
        12f32,
    );
    game.player.color = Color::MAROON;

    game.destroyed_meteor_count = 0;
    game.medium_meteor_count = 0;
    game.small_meteor_count = 0;

    for shot in &mut game.shots {
        shot.position = Vector2::default();
        shot.speed = Vector2::default();
        shot.radius = 2f32;
        shot.active = false;
        shot.life_spawn = 0;
        shot.color = Color::BLACK;
    }

    let mut correct_range = false;

    for meteor in &mut game.big_meteors {
        let mut x: i32 = rl.get_random_value(0..width as i32);

        while !correct_range {
            if x > half_width as i32 - 150 && x < half_width as i32 + 150 {
                x = rl.get_random_value(0..width as i32);
            } else {
                correct_range = true;
            }
        }

        correct_range = false;

        let mut y: i32 = rl.get_random_value(0..height as i32);

        while !correct_range {
            if y > half_height as i32 - 150 && y < half_height as i32 + 150 {
                y = rl.get_random_value(0..height as i32);
            } else {
                correct_range = true;
            }
        }

        correct_range = false;

        let mut vel_x: i32 = rl.get_random_value(-METEORS_SPEED as i32..METEORS_SPEED as i32);
        let mut vel_y: i32 = rl.get_random_value(-METEORS_SPEED as i32..METEORS_SPEED as i32);

        while !correct_range {
            if vel_x == 0 && vel_y == 0 {
                vel_x = rl.get_random_value(-METEORS_SPEED as i32..METEORS_SPEED as i32);
                vel_y = rl.get_random_value(-METEORS_SPEED as i32..METEORS_SPEED as i32);
            } else {
                correct_range = true;
            }
        }

        meteor.position = Vector2::new(x as f32, y as f32);
        meteor.speed = Vector2::new(vel_x as f32, vel_y as f32);
        meteor.radius = 40f32;
        meteor.active = true;
        meteor.color = Color::BLUE;
    }

    for meteor in &mut game.medium_meteors {
        meteor.position = Vector2::new(-100f32, -100f32);
        meteor.speed = Vector2::default();
        meteor.radius = 20f32;
        meteor.active = false;
        meteor.color = Color::BLUE;
    }

    for meteor in &mut game.small_meteors {
        meteor.position = Vector2::new(-100f32, -100f32);
        meteor.speed = Vector2::default();
        meteor.radius = 10f32;
        meteor.active = false;
        meteor.color = Color::BLUE;
    }
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
            } else if game.player.acceleration > 0f32 {
                game.player.acceleration -= 0.02;
            } else if game.player.acceleration < 0f32 {
                game.player.acceleration = 0f32;
            }

            if rl.is_key_down(KEY_DOWN) {
                if game.player.acceleration > 0f32 {
                    game.player.acceleration -= 0.04;
                } else if game.player.acceleration < 0f32 {
                    game.player.acceleration = 0f32;
                }
            }

            game.player.position.x += game.player.speed.x * game.player.acceleration;
            game.player.position.y -= game.player.speed.y * game.player.acceleration;

            let (width, height) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);

            if game.player.position.x > width + SHIP_HEIGHT {
                game.player.position.x = -SHIP_HEIGHT;
            } else if game.player.position.x < -SHIP_HEIGHT {
                game.player.position.x = width + SHIP_HEIGHT;
            }

            if game.player.position.y > height + SHIP_HEIGHT {
                game.player.position.y = -SHIP_HEIGHT;
            } else if game.player.position.y < -SHIP_HEIGHT {
                game.player.position.y = height + SHIP_HEIGHT;
            }

            if rl.is_key_pressed(KEY_SPACE) {
                for shot in &mut game.shots {
                    if !shot.active {
                        shot.position = Vector2::new(
                            game.player.position.x
                                + game.player.rotation.to_radians().sin() * SHIP_HEIGHT,
                            game.player.position.y
                                - game.player.rotation.to_radians().cos() * SHIP_HEIGHT,
                        );
                        shot.active = true;
                        shot.speed.x = 1.5 * game.player.rotation.to_radians().sin() * PLAYER_SPEED;
                        shot.speed.y = 1.5 * game.player.rotation.to_radians().cos() * PLAYER_SPEED;
                        shot.rotation = game.player.rotation;
                        break;
                    }
                }
            }

            for shot in &mut game.shots {
                if shot.active {
                    shot.life_spawn += 1;

                    shot.position.x += shot.speed.x;
                    shot.position.y -= shot.speed.y;

                    if shot.position.x > width + shot.radius {
                        shot.active = false;
                        shot.life_spawn = 0;
                    } else if shot.position.x < -shot.radius {
                        shot.active = false;
                        shot.life_spawn = 0;
                    }

                    if shot.position.y > height + shot.radius {
                        shot.active = false;
                        shot.life_spawn = 0;
                    } else if shot.position.y < -shot.radius {
                        shot.active = false;
                        shot.life_spawn = 0;
                    }

                    if shot.life_spawn >= 60 {
                        shot.position = Vector2::default();
                        shot.speed = Vector2::default();
                        shot.life_spawn = 0;
                        shot.active = false;
                    }

                    for meteor in &mut game.big_meteors {
                        if meteor.active
                            && check_collision_circles(
                                shot.position,
                                shot.radius,
                                meteor.position,
                                meteor.radius,
                            )
                        {
                            shot.active = false;
                            shot.life_spawn = 0;

                            meteor.active = false;
                            game.destroyed_meteor_count += 1;

                            for _ in 0..2 {
                                if game.medium_meteor_count % 2 == 0 {
                                    game.medium_meteors[game.medium_meteor_count as usize]
                                        .position =
                                        Vector2::new(meteor.position.x, meteor.position.y);
                                    game.medium_meteors[game.medium_meteor_count as usize].speed =
                                        Vector2::new(
                                            shot.rotation.to_radians().cos() * METEORS_SPEED * -1.0,
                                            shot.rotation.to_radians().sin() * METEORS_SPEED * -1.0,
                                        );
                                } else {
                                    game.medium_meteors[game.medium_meteor_count as usize]
                                        .position =
                                        Vector2::new(meteor.position.x, meteor.position.y);
                                    game.medium_meteors[game.medium_meteor_count as usize].speed =
                                        Vector2::new(
                                            shot.rotation.to_radians().cos() * METEORS_SPEED,
                                            shot.rotation.to_radians().sin() * METEORS_SPEED,
                                        );
                                }

                                game.medium_meteors[game.medium_meteor_count as usize].active =
                                    true;
                                game.medium_meteor_count += 1;
                            }

                            break;
                        }
                    }

                    for meteor in &mut game.medium_meteors {
                        if meteor.active
                            && check_collision_circles(
                                shot.position,
                                shot.radius,
                                meteor.position,
                                meteor.radius,
                            )
                        {
                            shot.active = false;
                            shot.life_spawn = 0;

                            meteor.active = false;
                            game.destroyed_meteor_count += 1;

                            for _ in 0..2 {
                                if game.small_meteor_count % 2 == 0 {
                                    game.small_meteors[game.small_meteor_count as usize].position =
                                        Vector2::new(meteor.position.x, meteor.position.y);
                                    game.small_meteors[game.small_meteor_count as usize].speed =
                                        Vector2::new(
                                            shot.rotation.to_radians().cos() * METEORS_SPEED * -1.0,
                                            shot.rotation.to_radians().sin() * METEORS_SPEED * -1.0,
                                        );
                                } else {
                                    game.small_meteors[game.small_meteor_count as usize].position =
                                        Vector2::new(meteor.position.x, meteor.position.y);
                                    game.small_meteors[game.small_meteor_count as usize].speed =
                                        Vector2::new(
                                            shot.rotation.to_radians().cos() * METEORS_SPEED,
                                            shot.rotation.to_radians().sin() * METEORS_SPEED,
                                        );
                                }

                                game.small_meteors[game.small_meteor_count as usize].active = true;
                                game.small_meteor_count += 1;
                            }

                            break;
                        }
                    }

                    for meteor in &mut game.small_meteors {
                        if meteor.active
                            && check_collision_circles(
                                shot.position,
                                shot.radius,
                                meteor.position,
                                meteor.radius,
                            )
                        {
                            shot.active = false;
                            shot.life_spawn = 0;

                            meteor.active = false;
                            game.destroyed_meteor_count += 1;

                            break;
                        }
                    }
                }
            }

            game.player.collider = Vector3::new(
                game.player.position.x
                    + game.player.rotation.to_radians().sin() * (SHIP_HEIGHT / 2.5),
                game.player.position.y
                    - game.player.rotation.to_radians().cos() * (SHIP_HEIGHT / 2.5),
                12f32,
            );

            for meteor in &game.big_meteors {
                if meteor.active
                    && check_collision_circles(
                        Vector2::new(game.player.collider.x, game.player.collider.y),
                        game.player.collider.z,
                        meteor.position,
                        meteor.radius,
                    )
                {
                    game.game_over = true;
                }
            }

            for meteor in &game.medium_meteors {
                if meteor.active
                    && check_collision_circles(
                        Vector2::new(game.player.collider.x, game.player.collider.y),
                        game.player.collider.z,
                        meteor.position,
                        meteor.radius,
                    )
                {
                    game.game_over = true;
                }
            }

            for meteor in &game.small_meteors {
                if meteor.active
                    && check_collision_circles(
                        Vector2::new(game.player.collider.x, game.player.collider.y),
                        game.player.collider.z,
                        meteor.position,
                        meteor.radius,
                    )
                {
                    game.game_over = true;
                }
            }

            for meteor in &mut game.big_meteors {
                if meteor.active {
                    meteor.position.x += meteor.speed.x;
                    meteor.position.y += meteor.speed.y;

                    if meteor.position.x > width + meteor.radius {
                        meteor.position.x = -meteor.radius;
                    } else if meteor.position.x < 0f32 - meteor.radius {
                        meteor.position.x = width + meteor.radius;
                    }

                    if meteor.position.y > height + meteor.radius {
                        meteor.position.y = -meteor.radius;
                    } else if meteor.position.y < 0f32 - meteor.radius {
                        meteor.position.y = height + meteor.radius;
                    }
                }
            }

            for meteor in &mut game.medium_meteors {
                if meteor.active {
                    meteor.position.x += meteor.speed.x;
                    meteor.position.y += meteor.speed.y;

                    if meteor.position.x > width + meteor.radius {
                        meteor.position.x = -meteor.radius;
                    } else if meteor.position.x < 0f32 - meteor.radius {
                        meteor.position.x = width + meteor.radius;
                    }

                    if meteor.position.y > height + meteor.radius {
                        meteor.position.y = -meteor.radius;
                    } else if meteor.position.y < 0f32 - meteor.radius {
                        meteor.position.y = height + meteor.radius;
                    }
                }
            }

            for meteor in &mut game.small_meteors {
                if meteor.active {
                    meteor.position.x += meteor.speed.x;
                    meteor.position.y += meteor.speed.y;

                    if meteor.position.x > width + meteor.radius {
                        meteor.position.x = -meteor.radius;
                    } else if meteor.position.x < 0f32 - meteor.radius {
                        meteor.position.x = width + meteor.radius;
                    }

                    if meteor.position.y > height + meteor.radius {
                        meteor.position.y = -meteor.radius;
                    } else if meteor.position.y < 0f32 - meteor.radius {
                        meteor.position.y = height + meteor.radius;
                    }
                }
            }
        }

        if game.destroyed_meteor_count
            == MAX_BIG_METEORS as u32 + MAX_MEDIUM_METEORS as u32 + MAX_SMALL_METEORS as u32
        {
            game.victory = true;
        }
    } else if rl.is_key_pressed(KEY_ENTER) {
        init_game(game, rl);
        game.game_over = false;
    }
}

fn draw_game(game: &Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let (width, height) = (rl.get_screen_width(), rl.get_screen_height());
    let mut d = rl.begin_drawing(thread);

    let half_width = width / 2;
    let half_height = height / 2;

    d.clear_background(Color::RAYWHITE);

    if !game.game_over {
        let cosf = f32::cos(game.player.rotation.to_radians());
        let sinf = f32::sin(game.player.rotation.to_radians());
        let v1 = Vector2::new(
            game.player.position.x + sinf * SHIP_HEIGHT,
            game.player.position.y - cosf * SHIP_HEIGHT,
        );
        let v2 = Vector2::new(
            game.player.position.x - cosf * 10f32,
            game.player.position.y - sinf * 10f32,
        );
        let v3 = Vector2::new(
            game.player.position.x + cosf * 10f32,
            game.player.position.y + sinf * 10f32,
        );
        d.draw_triangle(v1, v2, v3, game.player.color);

        for meteor in &game.big_meteors {
            if meteor.active {
                d.draw_circle_v(meteor.position, meteor.radius, meteor.color);
            } else {
                d.draw_circle_v(
                    meteor.position,
                    meteor.radius,
                    Color::fade(&Color::LIGHTGRAY, 0.3),
                );
            }
        }

        for meteor in &game.medium_meteors {
            if meteor.active {
                d.draw_circle_v(meteor.position, meteor.radius, meteor.color);
            } else {
                d.draw_circle_v(
                    meteor.position,
                    meteor.radius,
                    Color::fade(&Color::LIGHTGRAY, 0.3),
                );
            }
        }

        for meteor in &game.small_meteors {
            if meteor.active {
                d.draw_circle_v(meteor.position, meteor.radius, meteor.color);
            } else {
                d.draw_circle_v(
                    meteor.position,
                    meteor.radius,
                    Color::fade(&Color::LIGHTGRAY, 0.3),
                );
            }
        }

        for shot in &game.shots {
            if shot.active {
                d.draw_circle_v(shot.position, shot.radius, shot.color);
            }
        }

        if game.victory {
            d.draw_text(
                "VICTORY",
                half_width - d.measure_text("VICTORY", 20),
                half_height,
                20,
                Color::LIGHTGRAY,
            );
        }

        if game.pause {
            d.draw_text(
                "GAME PAUSED",
                half_width - d.measure_text("GAME PAUSED", 40),
                half_height - 40,
                40,
                Color::GRAY,
            );
        }
    } else {
        d.draw_text(
            "PRESS [ENTER] TO PLAY AGAIN",
            half_width - d.measure_text("PRESS [ENTER] TO PLAY AGAIN", 20),
            half_height - 50,
            20,
            Color::GRAY,
        );
    }
}
