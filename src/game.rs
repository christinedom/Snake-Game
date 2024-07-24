use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};

use crate::draw::{draw_rectangle, draw_circle};
use crate::snake::{Direction, Snake};

const APPLE_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const APPLE_STEM_COLOR: Color = [0.45, 0.32, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.50, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.00, 0.00, 0.00, 0.7];

const DEFAULT_MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;
const MIN_MOVING_PERIOD: f64 = 0.02;
const MAX_MOVING_PERIOD: f64 = 0.2;
const SPEED_CHANGE_STEP: f64 = 0.01;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
    moving_period: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            moving_period: DEFAULT_MOVING_PERIOD,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            Key::W => {
                self.increase_speed();
                None
            }
            Key::S => {
                self.decrease_speed();
                None
            }
            Key::R => {
                self.reset_speed();
                None
            }
            _ => Some(self.snake.head_direction()),
        };

        if let Some(dir) = dir {
            if dir != self.snake.head_direction().opposite() {
                self.update_snake(Some(dir));
            }
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_circle(APPLE_COLOR, self.food_x, self.food_y, 0.5, con, g);
            draw_rectangle(APPLE_STEM_COLOR, self.food_x, self.food_y - 1, 1, 1, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > self.moving_period {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
        self.moving_period = DEFAULT_MOVING_PERIOD;
    }

    fn increase_speed(&mut self) {
        self.moving_period = (self.moving_period - SPEED_CHANGE_STEP).max(MIN_MOVING_PERIOD);
    }

    fn decrease_speed(&mut self) {
        self.moving_period = (self.moving_period + SPEED_CHANGE_STEP).min(MAX_MOVING_PERIOD);
    }

    fn reset_speed(&mut self) {
        self.moving_period = DEFAULT_MOVING_PERIOD;
    }
}