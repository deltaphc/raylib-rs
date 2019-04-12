use raylib::{Color, Vector2};
use raylib::consts::*;

fn main() {
	let screen_width: i32 = 800;
	let screen_height: i32 = 450;

	let rl = raylib::init()
		.size(screen_width, screen_height)
		.title("raylib [core] example - keyboard input")
		.build();

	let mut ball_position = Vector2 { x: screen_width as f32/2.0, y: screen_height as f32/2.0 };

	rl.set_target_fps(60);

	while !rl.window_should_close() {
		if rl.is_key_down(KEY_RIGHT as i32) {
			ball_position.x += 0.8;
		}
		if rl.is_key_down(KEY_LEFT as i32) {
			ball_position.x -= 0.8;
		}
		if rl.is_key_down(KEY_UP as i32) {
			ball_position.y -= 0.8;
		}
		if rl.is_key_down(KEY_DOWN as i32) {
			ball_position.y += 0.8;
		}

		rl.begin_drawing();
		rl.clear_background(Color::RAYWHITE);
		
		rl.draw_text("move the ball with arrow keys", 10, 10, 20, Color::DARKGRAY);
		rl.draw_circle_v(ball_position, 50.0, Color::MAROON);

		rl.end_drawing();
	}
}
