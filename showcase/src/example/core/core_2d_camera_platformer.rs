/*******************************************************************************************
*
*   raylib [core] example - 2d camera platformer
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by arvyy (@arvyy) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 arvyy (@arvyy)
*
********************************************************************************************/


pub use raylib::prelude::*;

const G: f32 = 400.0;
const PLAYER_JUMP_SPD: f32 = 350.0;
const PLAYER_HOR_SPD: f32 = 200.0;

#[derive(Default)]
struct Player {
    position: Vector2,
    speed: f32,
    can_jump: bool,
}

struct EnvItem {
    rect: Rectangle,
    blocking: bool,
    color: Color,
}

impl EnvItem {
    fn new(rect: Rectangle, blocking: bool, color: Color) -> Self {
        Self {
            rect,
            blocking,
            color,
        }
    }
}

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_title(thread, "raylib [core] example - 2d camera platformer");
    rl.set_window_size(screen_width, screen_height);

    let mut player = Player {
        position: rvec2(400, 280),
        speed: 0.0,
        can_jump: false,
    };

    let env_items = [
        EnvItem::new(rrect(0, 0, 1000, 400), false, Color::LIGHTGRAY),
        EnvItem::new(rrect(0, 400, 1000, 200), true, Color::GRAY),
        EnvItem::new(rrect(300, 200, 400, 10), true, Color::GRAY),
        EnvItem::new(rrect(250, 300, 100, 10), true, Color::GRAY),
        EnvItem::new(rrect(650, 300, 100, 10), true, Color::GRAY),
    ];

    let mut camera = Camera2D {
        target: player.position,
        offset: rvec2(screen_width / 2, screen_height / 2),
        rotation: 0.0,
        zoom: 1.0,
    };

    // Store pointers to the multiple update camera functions
    let camera_updaters = [
        update_camera_center,
        update_camera_center_inside_map,
        update_camera_center_smooth_follow,
        update_camera_even_out_on_landing,
        update_camera_player_bounds_push,
    ];

    let mut camera_option = 0;

    let camera_description = [
        "Follow player center",
        "Follow player center, but clamp to map edges",
        "Follow player center; smoothed",
        "Follow player center horizontally; updateplayer center vertically after landing",
        "Player push camera on getting too close to screen edge",
    ];

    rl.set_target_fps(60);
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () {
        // Update
        //----------------------------------------------------------------------------------
        let delta_time = rl.get_frame_time();

        update_player(rl, &mut player, &env_items, delta_time);

        camera.zoom += rl.get_mouse_wheel_move() as f32 * 0.05;

        if camera.zoom > 3.0 {
            camera.zoom = 3.0;
        } else if camera.zoom < 0.25 {
            camera.zoom = 0.25;
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_R) {
            camera.zoom = 1.0;
            player.position = rvec2(400, 280);
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_C) {
            camera_option = (camera_option + 1) % camera_updaters.len();
        }

        // Call update camera function by its pointer
        camera_updaters[camera_option](
            rl,
            &mut camera,
            &player,
            &env_items,
            delta_time,
            screen_width as f32,
            screen_height as f32,
        );
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::LIGHTGRAY);

        {
            let mut d = d.begin_mode2D(camera);

            for ei in &env_items {
                d.draw_rectangle_rec(ei.rect, ei.color);
            }

            let player_rect = rrect(
                player.position.x - 20.0,
                player.position.y - 40.0,
                40.0,
                40.0,
            );
            d.draw_rectangle_rec(player_rect, Color::RED);
        }

        d.draw_text("Controls:", 20, 20, 10, Color::BLACK);
        d.draw_text("- Right/Left to move", 40, 40, 10, Color::DARKGRAY);
        d.draw_text("- Space to jump", 40, 60, 10, Color::DARKGRAY);
        d.draw_text(
            "- Mouse Wheel to Zoom in-out, R to reset zoom",
            40,
            80,
            10,
            Color::DARKGRAY,
        );
        d.draw_text("- C to change camera mode", 40, 100, 10, Color::DARKGRAY);
        d.draw_text("Current camera mode:", 20, 120, 10, Color::BLACK);
        d.draw_text(
            camera_description[camera_option],
            40,
            140,
            10,
            Color::DARKGRAY,
        );

        //----------------------------------------------------------------------------------
    });
}

fn update_player(rl: &RaylibHandle, player: &mut Player, env_items: &[EnvItem], delta: f32) {
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT) {
        player.position.x -= PLAYER_HOR_SPD * delta;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_RIGHT) {
        player.position.x += PLAYER_HOR_SPD * delta;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) && player.can_jump {
        player.speed = -PLAYER_JUMP_SPD;
        player.can_jump = false;
    }

    let mut hit_obstacle = false;
    for ei in env_items {
        let p = &mut player.position;
        if ei.blocking
            && ei.rect.x <= p.x
            && ei.rect.x + ei.rect.width >= p.x
            && ei.rect.y >= p.y
            && ei.rect.y < p.y + player.speed * delta
        {
            hit_obstacle = true;
            player.speed = 0.0;
            p.y = ei.rect.y;
        }
    }

    if !hit_obstacle {
        player.position.y += player.speed * delta;
        player.speed += G * delta;
        player.can_jump = false;
    } else {
        player.can_jump = true;
    }
}

fn update_camera_center(
    _rl: &RaylibHandle,
    camera: &mut Camera2D,
    player: &Player,
    _env_items: &[EnvItem],
    _delta: f32,
    width: f32,
    height: f32,
) {
    camera.offset = rvec2(width / 2.0, height / 2.0);
    camera.target = player.position;
}

fn update_camera_center_inside_map(
    rl: &RaylibHandle,
    camera: &mut Camera2D,
    player: &Player,
    env_items: &[EnvItem],
    _delta: f32,
    width: f32,
    height: f32,
) {
    camera.target = player.position;
    camera.offset = rvec2(width / 2.0, height / 2.0);
    let mut min_x = 1000.0;
    let mut min_y = 1000.0;
    let mut max_x = -1000.0;
    let mut max_y = -1000.0;

    for ei in env_items {
        min_x = ei.rect.x.min(min_x);
        max_x = (ei.rect.x + ei.rect.width).max(max_x);
        min_y = ei.rect.y.min(min_y);
        max_y = (ei.rect.y + ei.rect.height).max(max_y);
    }

    let max = rl.get_world_to_screen2D(rvec2(max_x, max_y), *camera);
    let min = rl.get_world_to_screen2D(rvec2(min_x, min_y), *camera);

    if max.x < width {
        camera.offset.x = width - (max.x - width / 2.0);
    }
    if max.y < height {
        camera.offset.y = height - (max.y - height / 2.0);
    }
    if min.x > 0.0 {
        camera.offset.x = width / 2.0 - min.x;
    }
    if min.y > 0.0 {
        camera.offset.y = height / 2.0 - min.y;
    }
}

fn update_camera_center_smooth_follow(
    _rl: &RaylibHandle,
    camera: &mut Camera2D,
    player: &Player,
    _env_items: &[EnvItem],
    delta: f32,
    width: f32,
    height: f32,
) {
    let min_speed = 30.0;
    let min_effect_length = 10.0;
    let fraction_speed = 0.8;

    camera.offset = rvec2(width / 2.0, height / 2.0);
    let diff = player.position - camera.target;
    let length = diff.length();

    if length > min_effect_length {
        let speed = (fraction_speed * length).max(min_speed);
        camera.target = camera.target + (diff * speed * delta / length);
    }
}

fn update_camera_even_out_on_landing(
    _rl: &RaylibHandle,
    camera: &mut Camera2D,
    player: &Player,
    _env_items: &[EnvItem],
    delta: f32,
    width: f32,
    height: f32,
) {
    static mut EVEN_OUT_SPEED: f32 = 700.0;
    static mut EVEN_OUT: bool = false;
    static mut EVEN_OUT_TARGET: f32 = 0.0;

    camera.offset = rvec2(width / 2.0, height / 2.0);
    camera.target.x = player.position.x;

    unsafe {
        if EVEN_OUT {
            if EVEN_OUT_TARGET > camera.target.y {
                camera.target.y += EVEN_OUT_SPEED * delta;

                if camera.target.y > EVEN_OUT_TARGET {
                    camera.target.y = EVEN_OUT_TARGET;
                    EVEN_OUT = false;
                }
            } else {
                camera.target.y -= EVEN_OUT_SPEED * delta;

                if camera.target.y < EVEN_OUT_TARGET {
                    camera.target.y = EVEN_OUT_TARGET;
                    EVEN_OUT = false;
                }
            }
        } else {
            if player.can_jump && (player.speed == 0.0) && (player.position.y != camera.target.y) {
                EVEN_OUT = true;
                EVEN_OUT_TARGET = player.position.y;
            }
        }
    }
}

fn update_camera_player_bounds_push(
    rl: &RaylibHandle,
    camera: &mut Camera2D,
    player: &Player,
    _env_items: &[EnvItem],
    _delta: f32,
    width: f32,
    height: f32,
) {
    let bbox = rvec2(0.2, 0.2);

    let bbox_world_min = rl.get_world_to_screen2D(
        rvec2((1.0 - bbox.x) * 0.5 * width, (1.0 - bbox.y) * 0.5 * height),
        *camera,
    );
    let bbox_world_max = rl.get_world_to_screen2D(
        rvec2((1.0 + bbox.x) * 0.5 * width, (1.0 + bbox.y) * 0.5 * height),
        *camera,
    );
    camera.offset = rvec2((1.0 - bbox.x) * 0.5 * width, (1.0 - bbox.y) * 0.5 * height);

    if player.position.x < bbox_world_min.x {
        camera.target.x = player.position.x;
    }
    if player.position.y < bbox_world_min.y {
        camera.target.y = player.position.y;
    }
    if player.position.x > bbox_world_max.x {
        camera.target.x = bbox_world_min.x + (player.position.x - bbox_world_max.x);
    }
    if player.position.y > bbox_world_max.y {
        camera.target.y = bbox_world_min.y + (player.position.y - bbox_world_max.y);
    }
}
