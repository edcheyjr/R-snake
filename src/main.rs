extern crate piston_window;
extern crate rand;

mod draw;
mod food;
mod game;
mod snake;

use draw::to_coord_u32;
use piston_window::types::Color;
use piston_window::*;

use game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20); // 500 * 500 game window
    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        // piston window draw 2d function
        window.draw_2d(&event, |c, g, device| {
            clear(BACK_COLOR, g);
            println!("{:?}", device.get_info().version);
            game.draw(&c, g);
        });

        // piston window update function
        event.update(|arg| {
            // println!("{:?}", game); // debug game on every update
            game.update(arg.dt);
        });
    }
    println!("{:?}", game);
}
