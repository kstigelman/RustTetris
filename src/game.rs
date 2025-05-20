use piston_window::types::Color;
use piston_window::*;

use crate::board::{Board};
use crate::draw::{draw_rectangle, to_coord_u32};
use crate::tetromino::{self, Direction, Tetromino};


const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

const MOVING_PERIOD: f64 = 1.0;
const RESTART_TIME: f64 = 1.0;


pub struct Game {
    pub current_tetromino: Tetromino,

    pub width: i32,
    pub height: i32,

    pub elapsed_time: f64,
    pub game_over: bool,

    pub board: Board,

    pub score: f64,
    move_speed: f64,
}

impl Game {
    pub fn new (width: i32, height: i32) -> Game {
        let w: u32 = to_coord_u32(width);
        let h: u32 = to_coord_u32(height);
        let mut sz: usize =  (w * h) as usize;
        Game {
            current_tetromino: Tetromino::new(4, 0),
            width,
            height,
            elapsed_time: 0.0,
            game_over: false,
            score: 0.0,
            board: Board::new(width, height),
            move_speed: MOVING_PERIOD,
        }
    }
    pub fn key_pressed (&mut self, key: Key) {
        let dir = match key {
            Key::Left => {
                self.board.clear_tetromino_from_board (&mut self.current_tetromino);
                self.current_tetromino.move_to_side(Direction::Left, self.width, self.height, &mut self.board.game_board);
                self.board.add_tetromino_to_board (&mut self.current_tetromino);
            },
            Key::Right => {
                self.board.clear_tetromino_from_board (&mut self.current_tetromino);
                self.current_tetromino.move_to_side(Direction::Right, self.width, self.height, &mut self.board.game_board);
                self.board.add_tetromino_to_board (&mut self.current_tetromino);
            },
            Key::A => {
                if self.current_tetromino.can_rotate(Direction::CounterClockwise, self.width, self.height, &mut self.board.game_board) {
                    self.board.clear_tetromino_from_board (&mut self.current_tetromino);
                    self.current_tetromino.rotate (Direction::CounterClockwise);
                    println! ("Attempting to rotate");
                    self.board.add_tetromino_to_board (&mut self.current_tetromino);
                }
            },
            Key::D => {
                if self.current_tetromino.can_rotate(Direction::Clockwise, self.width, self.height, &mut self.board.game_board) {
                    println! ("Attempting to rotate");
                    self.board.clear_tetromino_from_board (&mut self.current_tetromino);
                    self.current_tetromino.rotate (Direction::Clockwise);
                    self.board.add_tetromino_to_board (&mut self.current_tetromino);
                }
            }
            Key::S => {
                self.move_speed = MOVING_PERIOD / 5.0;
            }
            Key::W => {
                self.move_speed = MOVING_PERIOD;
            }
            _ => {

            }
        };

        //If key left
        // current_tetromino.move_to_side (Tetromino::TranslationDirection::Left);
        //If key right
        // current_tetromino.move_to_side (Tetromino::TranslationDirection::Right);
    }
    pub fn draw(&mut self, con: &Context, g: &mut G2d) {
        self.board.draw (con, g);

        //draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        //draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        //draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        //draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        //self.current_tetromino.draw (con, g);

    }
    pub fn update(&mut self, delta_time: f64) {
        self.elapsed_time += delta_time;
        if self.game_over {
            return;
        }
        if !self.board.piece_can_move {
            self.current_tetromino = Tetromino::new(4, 0);

            if self.board.game_board[3] != [0.0, 0.0, 0.0, 0.0] {
                self.game_over = true;
            }
            else { 
                let mut x = 0;
                let mut y = 0;
                

                while y < self.board.height {
                    x = 0;
                    while x < self.board.width {
                        let block = self.board.game_board[(x + (self.board.width * y)) as usize];
                        if block == [0.0, 0.0, 0.0, 0.0] {
                            break;
                        }
                        x += 1;
                        if x == self.width {
                            self.board.line_clear (y);
                        }
                    }
                    y += 1;
                }
            }
        }

        if self.elapsed_time > self.move_speed {
            self.board.update (delta_time, &mut self.current_tetromino);
            self.elapsed_time = 0.0;
        }
        
            /*current_tetromino = Tetromino {

            }*/
            // If current_tetromino start position is filled, game over!
    }

    

    fn restart (&mut self) {
        
    }
}