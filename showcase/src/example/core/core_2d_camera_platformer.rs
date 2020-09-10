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

use raylib::consts::KeyboardKey::*;
pub use raylib::prelude::*;

const G: f32 = 400.0;
const PLAYER_JUMP_SPD: f32 = 350.0;
const PLAYER_HOR_SPD: f32 = 200.0;

#[derive(Default)]
struct Player {
    position: Vector2,
    speed: f32,
    canJump: bool,
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
    let screenWidth = 800;
    let screenHeight = 450;

    rl.set_window_title(thread, "raylib [core] example - 2d camera platformer");
    rl.set_window_size(screenWidth, screenHeight);

    let mut player = Player {
        position: rvec2(400, 280),
        speed: 0.0,
        canJump: false,
    };

    let mut envItems = [
        EnvItem::new(rrect(0, 0, 1000, 400), false, Color::LIGHTGRAY),
        EnvItem::new(rrect(0, 400, 1000, 200), true, Color::GRAY),
        EnvItem::new(rrect(300, 200, 400, 10), true, Color::GRAY),
        EnvItem::new(rrect(250, 300, 100, 10), true, Color::GRAY),
        EnvItem::new(rrect(650, 300, 100, 10), true, Color::GRAY),
    ];

    let mut camera = Camera2D {
        target: player.position,
        offset: rvec2(screenWidth / 2, screenHeight / 2),
        rotation: 0.0,
        zoom: 1.0,
    };

    // Store pointers to the multiple update camera functions
    let cameraUpdaters = [
        update_camera_center,
        update_camera_center_inside_map,
        update_camera_center_smooth_follow,
        update_camera_even_out_on_landing,
        update_camera_player_bounds_push,
    ];

    let mut cameraOption = 0;

    let cameraDescriptions = [
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
        let deltaTime = rl.get_frame_time();

        update_player(rl, &mut player, &envItems, deltaTime);

        camera.zoom += rl.get_mouse_wheel_move() as f32 * 0.05;

        if (camera.zoom > 3.0) {
            camera.zoom = 3.0;
        } else if (camera.zoom < 0.25) {
            camera.zoom = 0.25;
        }

        if (rl.is_key_pressed(KEY_R)) {
            camera.zoom = 1.0;
            player.position = rvec2(400, 280);
        }

        if (rl.is_key_pressed(KEY_C)) {
            cameraOption = (cameraOption + 1) % cameraUpdaters.len();
        }

        // Call update camera function by its pointer
        cameraUpdaters[cameraOption](
            rl,
            &mut camera,
            &player,
            &envItems,
            deltaTime,
            screenWidth as f32,
            screenHeight as f32,
        );
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::LIGHTGRAY);

        {
            let mut d = d.begin_mode2D(camera);

            for ei in &envItems {
                d.draw_rectangle_rec(ei.rect, ei.color);
            }

            let playerRect = rrect(
                player.position.x - 20.0,
                player.position.y - 40.0,
                40.0,
                40.0,
            );
            d.draw_rectangle_rec(playerRect, Color::RED);
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
            cameraDescriptions[cameraOption],
            40,
            140,
            10,
            Color::DARKGRAY,
        );

        //----------------------------------------------------------------------------------
    });
}

fn update_player(rl: &RaylibHandle, player: &mut Player, envItems: &[EnvItem], delta: f32) {
    if (rl.is_key_down(KEY_LEFT)) {
        player.position.x -= PLAYER_HOR_SPD * delta;
    }
    if (rl.is_key_down(KEY_RIGHT)) {
        player.position.x += PLAYER_HOR_SPD * delta;
    }
    if (rl.is_key_down(KEY_SPACE) && player.canJump) {
        player.speed = -PLAYER_JUMP_SPD;
        player.canJump = false;
    }

    let mut hitObstacle = false;
    for ei in envItems {
        let p = &mut player.position;
        if ei.blocking
            && ei.rect.x <= p.x
            && ei.rect.x + ei.rect.width >= p.x
            && ei.rect.y >= p.y
            && ei.rect.y < p.y + player.speed * delta
        {
            hitObstacle = true;
            player.speed = 0.0;
            p.y = ei.rect.y;
        }
    }

    if !hitObstacle {
        player.position.y += player.speed * delta;
        player.speed += G * delta;
        player.canJump = false;
    } else {
        player.canJump = true;
    }
}

fn update_camera_center(
    rl: &RaylibHandle,
    camera: &mut Camera2D,
    player: &Player,
    envItems: &[EnvItem],
    delta: f32,
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
    envItems: &[EnvItem],
    delta: f32,
    width: f32,
    height: f32,
) {
    camera.target = player.position;
    camera.offset = rvec2(width / 2.0, height / 2.0);
    let mut minX = 1000.0;
    let mut minY = 1000.0;
    let mut maxX = -1000.0;
    let mut maxY = -1000.0;

    for ei in envItems {
        minX = ei.rect.x.min(minX);
        maxX = (ei.rect.x + ei.rect.width).max(maxX);
        minY = ei.rect.y.min(minY);
        maxY = (ei.rect.y + ei.rect.height).max(maxY);
    }

    let max = rl.get_world_to_screen2D(rvec2(maxX, maxY), *camera);
    let min = rl.get_world_to_screen2D(rvec2(minX, minY), *camera);

    if (max.x < width) {
        camera.offset.x = width - (max.x - width / 2.0);
    }
    if (max.y < height) {
        camera.offset.y = height - (max.y - height / 2.0);
    }
    if (min.x > 0.0) {
        camera.offset.x = width / 2.0 - min.x;
    }
    if (min.y > 0.0) {
        camera.offset.y = height / 2.0 - min.y;
    }
}

fn update_camera_center_smooth_follow(
    rl: &RaylibHandle,
    camera: &mut Camera2D,
    player: &Player,
    envItems: &[EnvItem],
    delta: f32,
    width: f32,
    height: f32,
) {
    let minSpeed = 30.0;
    let minEffectLength = 10.0;
    let fractionSpeed = 0.8;

    camera.offset = rvec2(width / 2.0, height / 2.0);
    let diff = player.position - camera.target;
    let length = diff.length();

    if (length > minEffectLength) {
        let speed = (fractionSpeed * length).max(minSpeed);
        camera.target = camera.target + (diff * speed * delta / length);
    }
}

fn update_camera_even_out_on_landing(
    rl: &RaylibHandle,
    camera: &mut Camera2D,
    player: &Player,
    envItems: &[EnvItem],
    delta: f32,
    width: f32,
    height: f32,
) {
    static mut evenOutSpeed: f32 = 700.0;
    static mut eveningOut: bool = false;
    static mut evenOutTarget: f32 = 0.0;

    camera.offset = rvec2(width / 2.0, height / 2.0);
    camera.target.x = player.position.x;

    unsafe {
        if (eveningOut) {
            if (evenOutTarget > camera.target.y) {
                camera.target.y += evenOutSpeed * delta;

                if (camera.target.y > evenOutTarget) {
                    camera.target.y = evenOutTarget;
                    eveningOut = false;
                }
            } else {
                camera.target.y -= evenOutSpeed * delta;

                if (camera.target.y < evenOutTarget) {
                    camera.target.y = evenOutTarget;
                    eveningOut = false;
                }
            }
        } else {
            if (player.canJump && (player.speed == 0.0) && (player.position.y != camera.target.y)) {
                eveningOut = true;
                evenOutTarget = player.position.y;
            }
        }
    }
}

fn update_camera_player_bounds_push(
    rl: &RaylibHandle,
    camera: &mut Camera2D,
    player: &Player,
    envItems: &[EnvItem],
    delta: f32,
    width: f32,
    height: f32,
) {
    let bbox = rvec2(0.2, 0.2);

    let bboxWorldMin = rl.get_world_to_screen2D(
        rvec2((1.0 - bbox.x) * 0.5 * width, (1.0 - bbox.y) * 0.5 * height),
        *camera,
    );
    let bboxWorldMax = rl.get_world_to_screen2D(
        rvec2((1.0 + bbox.x) * 0.5 * width, (1.0 + bbox.y) * 0.5 * height),
        *camera,
    );
    camera.offset = rvec2((1.0 - bbox.x) * 0.5 * width, (1.0 - bbox.y) * 0.5 * height);

    if (player.position.x < bboxWorldMin.x) {
        camera.target.x = player.position.x;
    }
    if (player.position.y < bboxWorldMin.y) {
        camera.target.y = player.position.y;
    }
    if (player.position.x > bboxWorldMax.x) {
        camera.target.x = bboxWorldMin.x + (player.position.x - bboxWorldMax.x);
    }
    if (player.position.y > bboxWorldMax.y) {
        camera.target.y = bboxWorldMin.y + (player.position.y - bboxWorldMax.y);
    }
}
