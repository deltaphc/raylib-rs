use raylib::color::Color;
use raylib::consts::KeyboardKey;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::{RaylibHandle};
use std::array;
use std::cmp::PartialEq;
use std::collections::VecDeque;
use std::fmt::{Debug};


#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

const MAP_MAX: usize = 9;
#[derive(Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    // Move one unit in the given cardinal direction with Roll-around behaviour
    fn move_direction(&mut self, direction: Direction) {
        const MIN: usize = 0;

        // increment the direction, with roll around to 0 and MAX over boundaries
        match direction {
            Direction::North => {
                if self.y == MIN {
                    self.y = MAP_MAX;
                } else {
                    self.y -= 1;
                }
            }
            Direction::South => {
                if self.y >= MAP_MAX {
                    self.y = MIN;
                } else {
                    self.y += 1;
                }
            }
            Direction::West => {
                if self.x == MIN {
                    self.x = MAP_MAX;
                } else {
                    self.x -= 1;
                }
            }
            Direction::East => {
                if self.x >= MAP_MAX {
                    self.x = MIN;
                } else {
                    self.x += 1;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Snake {
    length: usize,
    facing: Direction,
    position: Position,
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            facing: Direction::North,
            length: 1,
            position: Position { x: 0, y: 0 },
        }
    }
}

impl Snake {
    fn extend(&mut self) {
        self.length += 1;
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
enum GameCell {
    #[default]
    Empty,
    SnakeHead,
    Food,
    SnakeTail,
}

#[derive(Debug)]
struct GameMap {
    cells: [[GameCell; 10]; 10],
    displayed_snake_segments: VecDeque<Position>,
}

enum SnakeState {
    FoodEaten,
    Stuck,
    Moved,
}

#[derive(PartialEq)]
enum GameState {
    Starting,
    Paused,
    Running,
    Lost,
    Won,
}

impl GameMap {
    fn reset(&mut self, snake: &mut Snake) {
        let (map2, snake2) = GameMap::new_with_snake();
        *snake = snake2;
        *self = map2;

        return;
    }

    fn new_with_snake() -> (Self, Snake) {
        let mut snake = Snake::default();
        let row: [GameCell; 10] = array::from_fn(|_| GameCell::default());
        let all = array::from_fn(|_| row.clone());

        let mut map = GameMap {
            cells: all,
            displayed_snake_segments: VecDeque::new(),
        };

        let snake_pos = Position::new(4, 5);
        map.set_cell_by_pos(snake_pos.clone(), GameCell::SnakeHead);
        snake.position = snake_pos;

        map.set_cell_by_pos(Position::new(3, 3), GameCell::Food);

        return (map, snake);
    }


    fn set_cell_by_pos(&mut self, position: Position, new_state: GameCell) {
        self.cells[position.y][position.x] = new_state;
    }

    fn get_cell_by_pos(&self, position: Position) -> GameCell {
        return self.cells[position.y][position.x].clone();
    }

    fn move_snake(&mut self, snake: &mut Snake) -> SnakeState {
        // get last pos
        let prev_pos = snake.position.clone();

        Position::move_direction(&mut snake.position, snake.facing.clone());

        // check if food was eaten or a collision happened
        let snake_move = match self.get_cell_by_pos(snake.position.clone()) {
            GameCell::SnakeTail | GameCell::SnakeHead => SnakeState::Stuck,
            GameCell::Empty => SnakeState::Moved,
            GameCell::Food => SnakeState::FoodEaten,
        };

        // apply new head to map
        self.set_cell_by_pos(snake.position.clone(), GameCell::SnakeHead);

        // if new tail segments, advance the last tail position to the front
        if snake.length - 1 > self.displayed_snake_segments.len() {
            self.set_cell_by_pos(prev_pos.clone(), GameCell::SnakeTail);
            self.displayed_snake_segments.push_front(prev_pos.clone());
        }
        // if no new tail segments
        else {
            // move last tail segment to the front
            if let Some(to_erase) = self.displayed_snake_segments.pop_back() {
                self.set_cell_by_pos(to_erase, GameCell::Empty);
                self.set_cell_by_pos(prev_pos.clone(), GameCell::SnakeTail);
                self.displayed_snake_segments.push_front(prev_pos.clone());
            } else {
                // if no tail segments at all, ignore the last cell
                self.set_cell_by_pos(prev_pos, GameCell::Empty);
            }
        }

        return snake_move;
    }

    fn generate_food(&mut self, rl: &RaylibHandle) {
        // list all the empty cells
        let empty = {
            let mut empties = Vec::with_capacity(MAP_MAX * MAP_MAX);

            for (yi, y) in self.cells.iter().enumerate() {
                for (xi, x) in y.iter().enumerate() {
                    if *x == GameCell::Empty {
                        let empty = Position::new(xi, yi);
                        empties.push(empty);
                    }
                }
            }

            empties
        };

        // pick one of these random empty cells
        let max = empty.len();

        let range = 0i32..(max as i32) - 1;
        let random: i32 = rl.get_random_value(range);

        // set the cell
        let chosen = empty[random as usize].clone();
        self.set_cell_by_pos(chosen, GameCell::Food);
    }
}

fn draw_game(draw: &mut RaylibDrawHandle, map: &GameMap, window: &WindowDimensions, score: usize) {
    draw.clear_background(Color::BLACK);

    // draw grid
    let step_size = 50;
    for x in (window.active_start_height..window.active_height + 1).step_by(step_size) {
        draw.draw_line(
            x,
            window.active_start_width,
            x,
            window.active_height,
            Color::WHEAT,
        );
    }
    for y in (window.active_start_width..window.active_width + 1).step_by(step_size) {
        draw.draw_line(
            window.active_start_height,
            y,
            window.active_width,
            y,
            Color::WHEAT,
        );
    }

    // draw map
    for (i, x) in (window.active_start_height..window.active_height)
        .step_by(step_size)
        .enumerate()
    {
        for (j, y) in (window.active_start_width..window.active_width)
            .step_by(step_size)
            .enumerate()
        {
            let cell_type: &GameCell = &map.cells[j][i];
            let cell_color = match cell_type {
                GameCell::SnakeHead => Some(Color::GREEN),
                GameCell::Empty => None,
                GameCell::Food => Some(Color::RED),
                GameCell::SnakeTail => Some(Color::LIGHTGREEN),
            };

            if let Some(color) = cell_color {
                let rect_x = x + ((step_size as f32 * 0.1) as i32);
                let rect_y = y + ((step_size as f32 * 0.1) as i32);
                let rect_width = (step_size as f32 * 0.80) as _;
                let rect_height = (step_size as f32 * 0.80) as _;

                draw.draw_rectangle(rect_x, rect_y, rect_width, rect_height, color);
            }
        }
    }

    // draw score
    draw.draw_text(
        "Score: ",
        window.active_width + step_size as i32,
        step_size as _,
        20,
        Color::BLUE,
    );
    let score_text = score.to_string();
    draw.draw_text(
        &score_text,
        window.active_width + step_size as i32,
        (step_size as f32 * 1.5) as i32,
        20,
        Color::RED,
    );

    // draw help
    draw.draw_text(
        "Help: ",
        window.active_width + step_size as i32,
        step_size as i32 * 3,
        20,
        Color::BLUE,
    );
    draw.draw_text(
        "Use the arrow keys to move",
        window.active_width + step_size as i32,
        (step_size as f32 * 3.5) as i32,
        10,
        Color::RED,
    );
    draw.draw_text(
        "and P to pause.",
        window.active_width + step_size as i32,
        (step_size as f32 * 3.75) as i32,
        10,
        Color::RED,
    );

    draw.draw_text(
        "Or, you can use W,A,S,D to move",
        window.active_width + step_size as i32,
        (step_size as f32 * 4.0) as i32,
        10,
        Color::RED,
    );
    draw.draw_text(
        "and space to pause.",
        window.active_width + step_size as i32,
        (step_size as f32 * 4.25) as i32,
        10,
        Color::RED,
    );
}

struct WindowDimensions {
    width: i32,
    height: i32,
    active_width: i32,
    active_height: i32,
    active_start_height: i32,
    active_start_width: i32,
}

impl WindowDimensions {
    fn new(
        width: i32,
        height: i32,
        active_width: i32,
        active_height: i32,
        active_start_height: i32,
        active_start_width: i32,
    ) -> WindowDimensions {
        WindowDimensions {
            width,
            height,
            active_width,
            active_height,
            active_start_height,
            active_start_width,
        }
    }
}

struct Ticker {
    count: usize,
    max: usize,
}

impl Ticker {
    fn reset(&mut self) {
        self.count = 0;
    }

    fn tick(&mut self) {
        self.count += 1;
        if self.count > self.max {
            self.count = 0;
        }
    }

    fn is_at_edge(&self) -> bool {
        self.count == self.max
    }

    fn new(max: usize) -> Ticker {
        Ticker { count: 0, max }
    }
}

fn key_to_direction(key: &Option<KeyboardKey>) -> Option<Direction> {
    match key {
        Some(KeyboardKey::KEY_UP | KeyboardKey::KEY_W) => Some(Direction::North),
        Some(KeyboardKey::KEY_DOWN | KeyboardKey::KEY_S) => Some(Direction::South),
        Some(KeyboardKey::KEY_LEFT | KeyboardKey::KEY_A) => Some(Direction::West),
        Some(KeyboardKey::KEY_RIGHT | KeyboardKey::KEY_D) => Some(Direction::East),
        Some(_) => None,
        None => None,
    }
}

fn is_pause_key(key: &Option<KeyboardKey>) -> bool {
    match key {
        Some(KeyboardKey::KEY_P | KeyboardKey::KEY_SPACE) => true,
        _ => false,
    }
}

fn main() {
    let window = WindowDimensions::new(800, 600, 550, 550, 50, 50);

    let (mut map, mut snake) = GameMap::new_with_snake();
    let mut score = 0;
    let mut game_state = GameState::Starting;

    let (mut rl, mut thd) = raylib::init()
        .width(window.width)
        .height(window.height)
        .title("Snake")
        .build();

    const FPS: i32 = 60;

    rl.set_target_fps(FPS as u32);

    rl.set_random_seed(0x23521);

    let mut ticker = Ticker::new(FPS as usize);

    while !rl.window_should_close() {
        ticker.tick();

        {
            match game_state {
                GameState::Starting => {
                    score = 0;
                    map.reset(&mut snake);
                    ticker.reset();

                    game_state = GameState::Running;
                }
                GameState::Running => {
                    let key = rl.get_key_pressed();
                    let new_facing_opt = key_to_direction(&key);

                    if is_pause_key(&key) {
                        game_state = GameState::Paused;
                        continue;
                    }

                    // check for win condition
                    let mut tally = 0;
                    for row in map.cells.iter() {
                        for item in row.iter() {
                            if *item == GameCell::Empty {
                                tally += 1;
                            }
                        }
                    }
                    if tally < 1 {
                        game_state = GameState::Won;
                        continue;
                    }

                    if let Some(new_facing) = new_facing_opt {
                        snake.facing = new_facing;
                    }

                    // move snake
                    if ticker.is_at_edge() {
                        let snake_state = map.move_snake(&mut snake);
                        match snake_state {
                            SnakeState::Moved => {}
                            SnakeState::FoodEaten => {
                                score += 1;
                                snake.extend();
                                map.generate_food(&rl);
                            }
                            SnakeState::Stuck => {
                                game_state = GameState::Lost;
                            }
                        }
                    }

                    {
                        let mut draw = rl.begin_drawing(&thd);
                        draw_game(&mut draw, &map, &window, score);
                    }
                }
                GameState::Lost => {
                    {
                        let mut draw = rl.begin_drawing(&mut thd);

                        write_center(
                            "Game over. Press P to restart\n",
                            &mut draw,
                            window.width,
                            window.height,
                            50,
                        );
                    }
                    let key = rl.get_key_pressed();
                    if is_pause_key(&key) {
                        game_state = GameState::Starting
                    }
                }

                GameState::Paused => {
                    {
                        let mut draw = rl.begin_drawing(& thd);
                        write_center(
                            "Paused. Press P to resume\n",
                            &mut draw,
                            window.width,
                            window.height,
                            50,
                        );
                    }
                    let key = rl.get_key_pressed();

                    if is_pause_key(&key) {
                        game_state = GameState::Running
                    }
                }

                GameState::Won => {
                    {
                        let mut draw = rl.begin_drawing(&mut thd);
                        write_center("You win!", &mut draw, window.width, window.height, 50);
                    }
                    let key = rl.get_key_pressed();
                    if is_pause_key(&key) {
                        game_state = GameState::Starting;
                    }
                }
            }
        }
    }
}

fn write_center(
    text: &str,
    draw: &mut RaylibDrawHandle,
    screen_width: i32,
    screen_height: i32,
    fontsize: i32,
) {
    let text_width = draw.measure_text(text, fontsize);
    let textx = screen_width / 2 - text_width / 2;
    let texty = screen_height / 2 - fontsize;

    draw.draw_rectangle(
        textx - 10,
        texty - 10,
        text_width + 20,
        fontsize + 20,
        Color::WHEAT,
    );
    draw.draw_text(text, textx, texty, fontsize, Color::RED);
}
