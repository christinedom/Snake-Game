extern crate piston_window;
extern crate rand;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;
use crate::draw::{to_coord_u32, draw_rectangle};

const BACK_COLOR: Color = [0.82, 0.96, 0.82, 1.0];
const BORDER_COLOR: Color = [0.00, 0.50, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);

            // Draw simple instructions
            draw_rectangle(BORDER_COLOR, 0, height, width, 3, &c, g);
            let instructions = [
                "Controls: Arrow keys to move",
                "W: Speed up, S: Slow down, R: Reset speed",
            ];
            for (i, &instruction) in instructions.iter().enumerate() {
                let y = height + i as i32 + 1;
                for (j, _) in instruction.chars().enumerate() {
                    draw_rectangle(GAMEOVER_COLOR, j as i32, y, 1, 1, &c, g);
                }
            }
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}