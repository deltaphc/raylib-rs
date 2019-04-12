use raylib::Color;

fn main() {
	let rl = raylib::init()  					// Initialize the raylib handle
		.size(800, 450)
		.title("raylib [core] example - basic window")
		.build();

	rl.set_target_fps(60);   // Limits fps

	while !rl.window_should_close() {
		// Update stuff here

		rl.begin_drawing();
		rl.clear_background(Color::RAYWHITE);
		// Draw stuff here
		rl.draw_text("Congrats! You created your first window!", 190, 200, 20, Color::LIGHTGRAY);
		rl.end_drawing();
	}
}
