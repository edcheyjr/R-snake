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

///Background Color
const BG_COLOR: Color = [0.5, 0.5, 0.2, 1.0];

fn main() {
    let mut is_device_info_printed = false;
    let (width, height) = (30, 30); // 750 * 750 game window while block is 25

    //creating a new window
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
            clear(BG_COLOR, g);
            if !is_device_info_printed {
                println!("FOR DEBUGGING");
                println!("{:?}", device.get_info());
                println!("Intial game object{:?}", game);
                is_device_info_printed = true;
            }
            game.draw(&c, g);
        });

        // piston window update function
        event.update(|arg| {
            // println!("{:?}", game); // debug game on every update
            game.update(arg.dt);
        });
    }
    println!("Last game object{:?}", game);
}
