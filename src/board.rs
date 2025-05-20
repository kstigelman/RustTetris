use std::thread::current;

use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::{draw_rectangle, draw_block, to_coord_u32};
use crate::tetromino::{Tetromino};

const PIECE_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
pub struct Board {
    pub width: i32,
    pub height: i32,
    pub game_board: Vec<Color>,
    pub piece_can_move: bool,
}

impl Board {

    pub fn new (width: i32, height: i32) -> Board {
        let w: u32 = to_coord_u32(width);
        let h: u32 = to_coord_u32(height);
        let mut sz: usize =  (w * h) as usize;

        Board {
            width,
            height,
            game_board: vec![[0.0, 0.0, 0.0, 0.0]; sz],
            piece_can_move: true,
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {

        let mut x = 0;
        let mut y = 0;
        while y < self.height {
            x = 0;
            while x < self.width {
                let index =(self.width * y) + x;
                draw_block(self.game_board[index as usize], x, y, con, g);
                //draw_block([1.0, 0.0, 0.0, 1.0], x, y, con, g);
                //println! ("Width: {}, Height: {}, Y: {}", self.width, self.height, y);
                //draw_block([self.game_board[index as usize]], x, y, con, g);
                x += 1;
            }
            y += 1;
        }
    }
    pub fn update(&mut self, delta_time: f64, tetromino: &mut Tetromino) {
        self.clear_tetromino_from_board(tetromino);
        self.piece_can_move = tetromino.move_down(self.width, self.height, &mut self.game_board);
        //println! ("{} {}", tetromino.x, tetromino.y);
        if !self.piece_can_move {
            // Checks all rows for a line clear
            
        }

            
            /**/
        
        //tetromino.move_down(self.width, self.height, &mut self.game_board);
        self.add_tetromino_to_board(tetromino);

        
    }

    pub fn clear_tetromino_from_board (&mut self, tetromino: &mut Tetromino) {
        for block in tetromino.blocks {
            
            let index: i32 = ((block.y + tetromino.y) * self.width) + (tetromino.x + block.x);
            if index < 0 {
                if tetromino.x > 0 {
                    continue;
                }
                break;
            }
            //self.game_board
            self.game_board [index as usize] = [0.0, 0.0, 0.0, 0.0];
        }
    }
    pub fn add_tetromino_to_board (&mut self, tetromino: &mut Tetromino) {
        for block in tetromino.blocks {
            let index: i32 = ((block.y + tetromino.y) * self.width) + (tetromino.x + block.x);
            //println! ("BX: {} BY: {} TX: {} TY {}", block.x, block.y, tetromino.x, tr)
            //self.game_board
            self.game_board [index as usize] = tetromino.color;
        }
    }
    
    pub fn line_clear (&mut self, first_row: i32) {
        let mut prev_row = first_row - 1;
        let mut current_row = first_row;
        let mut col = 0;
        while current_row >= 0 {
            col = 0;
            while col < self.width {
                if current_row == 0 {
                    self.game_board[col as usize] = [0.0, 0.0, 0.0, 0.0];
                }
                else {
                    let current_index = (current_row * self.width) + col;
                    let prev_index = (prev_row * self.width) + col;
                    self.game_board[current_index as usize] = self.game_board[prev_index as usize];
                }
                col += 1;
            }
            current_row = current_row - 1;
            prev_row = prev_row - 1;
        }
        //remove all rows that are full
        //shift all rows ABOVE these rows down!
    }
}