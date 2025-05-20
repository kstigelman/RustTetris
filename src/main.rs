extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod tetromino;
mod board;

use piston_window::types::Color;
use piston_window::*;
use crate::draw::to_coord_u32;
use crate::game::Game;

const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

fn main () {
    let (width, height) = (10, 20);

    let mut window: PistonWindow = WindowSettings::new ("Tetris", [to_coord_u32(width), to_coord_u32(height)])
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new (width, height);

    while let Some(event) = window.next() {
        if let Some (Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key)
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });
        /*window.draw_2d (e: &event, f: |c: Context, g, _| {
            clear(BACK_COLOR, g);
            game.draw(con, g);
        });*/

        event.update(|arg| {
            game.update(arg.dt);
        });
    }

}