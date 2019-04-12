use raylib::Color;
use raylib::consts::*;

fn main() {
	let rl = raylib::init()
		.size(800, 450)
		.title("raylib [core] example - mouse input")
		.build();

	rl.set_target_fps(60);

	let mut ball_color = Color::DARKBLUE;

	while !rl.window_should_close() {
		let ball_position = rl.get_mouse_position();

		if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON as i32) {
			ball_color = Color::MAROON;
		} else if rl.is_mouse_button_pressed(MOUSE_MIDDLE_BUTTON as i32) {
			ball_color = Color::LIME;
		} else if rl.is_mouse_button_pressed(MOUSE_RIGHT_BUTTON as i32) {
			ball_color = Color::DARKBLUE;
		}

		rl.begin_drawing();

		rl.clear_background(Color::RAYWHITE);
		rl.draw_circle_v(ball_position, 40.0, ball_color);

		rl.draw_text("move ball with mouse and click mouse button to change color", 10, 10, 20, Color::DARKGRAY);

		rl.end_drawing();
	}
}
