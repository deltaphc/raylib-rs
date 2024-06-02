use raylib::prelude::KeyboardKey::*;
use raylib::prelude::*;

const MAX_TUBES: usize = 100;
const FLOPPY_RADIUS: f32 = 24.0;
const TUBES_WIDTH: i32 = 80;

//----------------------------------------------------------------------------------
// Types and Structures Definition
//----------------------------------------------------------------------------------
#[derive(Default)]
struct Floppy {
    position: Vector2,
    radius: f32,
    color: Color,
}

#[derive(Default, Clone, Copy)]
struct Tubes {
    rec: Rectangle,
    color: Color,
    active: bool,
}

struct Game {
    game_over: bool,
    pause: bool,
    score: i32,
    hi_score: i32,
    floppy: Floppy,
    tubes: [Tubes; MAX_TUBES * 2],
    tubes_pos: [Vector2; MAX_TUBES],
    tubes_speed_x: f32,
    superfx: bool,
}

//------------------------------------------------------------------------------------
// Global Variables Declaration
//------------------------------------------------------------------------------------
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

impl Default for Game {
    fn default() -> Self {
        Game {
            game_over: false,
            pause: false,
            score: 0,
            hi_score: 0,
            floppy: Floppy {
                position: Vector2::default(),
                radius: 0.0,
                color: Color::default(),
            },
            tubes: [Tubes {
                rec: Rectangle::default(),
                color: Color::default(),
                active: false,
            }; MAX_TUBES * 2],
            tubes_pos: [Vector2::default(); MAX_TUBES],
            tubes_speed_x: 0.0,
            superfx: false,
        }
    }
}

//------------------------------------------------------------------------------------
// Program main entry point
//------------------------------------------------------------------------------------
fn main() {
    // Initialization
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("classic game: floppy")
        .build();

    let mut game = Game::default();

    init_game(&mut game, &rl);

    rl.set_target_fps(60);
    // Main game loop
    while !rl.window_should_close()
    // Detect window close button or ESC key
    {
        // Update and Draw
        update_draw_frame(&mut game, &mut rl, &thread);
    }
}

//------------------------------------------------------------------------------------
// Module Functions Definitions (local)
//------------------------------------------------------------------------------------

// Initialize game variables
fn init_game(game: &mut Game, rl: &RaylibHandle) {
    game.floppy.radius = FLOPPY_RADIUS;
    game.floppy.position = Vector2::new(80.0, SCREEN_HEIGHT as f32 / 2.0 - game.floppy.radius);
    game.floppy.color = Color::DARKGRAY;
    game.tubes_speed_x = 2.0;

    for i in 0..MAX_TUBES {
        game.tubes_pos[i].x = (400 + 280 * i) as f32;
        game.tubes_pos[i].y = -rl.get_random_value::<i32>(0..120) as f32;
    }

    for i in (0..MAX_TUBES * 2).step_by(2) {
        game.tubes[i].rec.x = game.tubes_pos[i / 2].x;
        game.tubes[i].rec.y = game.tubes_pos[i / 2].y;
        game.tubes[i].rec.width = TUBES_WIDTH as f32;
        game.tubes[i].rec.height = 255.0;
        game.tubes[i].color = Color::DARKGRAY;

        game.tubes[i + 1].rec.x = game.tubes_pos[i / 2].x;
        game.tubes[i + 1].rec.y = 600.0 + game.tubes_pos[i / 2].y - 255.0;
        game.tubes[i + 1].rec.width = TUBES_WIDTH as f32;
        game.tubes[i + 1].rec.height = 255.0;
        game.tubes[i + 1].color = Color::DARKGRAY;

        game.tubes[i / 2].active = true;
    }
}
//
// // Update game (one frame)
fn update_game(game: &mut Game, rl: &RaylibHandle) {
    if !game.game_over {
        if rl.is_key_pressed(KEY_P) {
            game.pause = !game.pause
        };

        if !game.pause {
            for i in 0..MAX_TUBES {
                game.tubes_pos[i].x -= game.tubes_speed_x
            }

            for i in (0..MAX_TUBES * 2).step_by(2) {
                game.tubes[i].rec.x = game.tubes_pos[i / 2].x;
                game.tubes[i + 1].rec.x = game.tubes_pos[i / 2].x;
            }

            if rl.is_key_down(KEY_SPACE) && !game.game_over {
                game.floppy.position.y -= 3.0;
            } else {
                game.floppy.position.y += 1.0;
            }

            // Check Collisions
            for i in 0..MAX_TUBES {
                if game.tubes[i]
                    .rec
                    .check_collision_circle_rec(game.floppy.position, game.floppy.radius)
                {
                    game.game_over = true;
                    game.pause = false;
                } else if (game.tubes_pos[i / 2].x < game.floppy.position.x)
                    && game.tubes[i / 2].active
                    && !game.game_over
                {
                    game.score += 100;
                    game.tubes[i / 2].active = false;

                    game.superfx = true;

                    if game.score > game.hi_score {
                        game.hi_score = game.score;
                    }
                }
            }
        }
    } else if rl.is_key_pressed(KEY_ENTER) {
        init_game(game, rl);
        game.game_over = false;
    }
}
//
// // Draw game (one frame)
fn draw_game(game: &mut Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(thread);

    d.clear_background(Color::RAYWHITE);

    if !game.game_over {
        d.draw_circle(
            game.floppy.position.x as i32,
            game.floppy.position.y as i32,
            game.floppy.radius,
            game.floppy.color,
        );

        // Draw tubes
        for i in 0..MAX_TUBES {
            d.draw_rectangle(
                game.tubes[i * 2].rec.x as i32,
                game.tubes[i * 2].rec.y as i32,
                game.tubes[i * 2].rec.width as i32,
                game.tubes[i * 2].rec.height as i32,
                game.tubes[i * 2].color,
            );
            d.draw_rectangle(
                game.tubes[i * 2 + 1].rec.x as i32,
                game.tubes[i * 2 + 1].rec.y as i32,
                game.tubes[i * 2 + 1].rec.width as i32,
                game.tubes[i * 2 + 1].rec.height as i32,
                game.tubes[i * 2 + 1].color,
            );
        }

        // Draw flashing fx (one frame only)
        if game.superfx {
            d.draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::WHITE);
            game.superfx = false;
        }

        d.draw_text(&format!("{:04}", game.score), 20, 20, 40, Color::GRAY);
        d.draw_text(
            &format!("HI-SCORE: {:04}", game.hi_score),
            20,
            70,
            20,
            Color::LIGHTGRAY,
        );

        if game.pause {
            d.draw_text(
                "GAME PAUSED",
                SCREEN_WIDTH / 2 - d.measure_text("GAME PAUSED", 40) / 2,
                SCREEN_HEIGHT / 2 - 40,
                40,
                Color::GRAY,
            );
        }
    } else {
        d.draw_text(
            "PRESS [ENTER] TO PLAY AGAIN",
            d.get_screen_width() / 2 - d.measure_text("PRESS [ENTER] TO PLAY AGAIN", 20) / 2,
            d.get_screen_height() / 2 - 50,
            20,
            Color::GRAY,
        );
    }
}

// Update and Draw (one frame)
fn update_draw_frame(game: &mut Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    update_game(game, rl);
    draw_game(game, rl, thread);
}