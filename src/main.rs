use std::fmt;
mod board;
use crate::board::board::*;

use raylib::prelude::*;
const SQUARE_SIZE: i32 = 64;

fn draw_game(rl: &mut RaylibDrawHandle, thread: &RaylibThread, game: &ChessBoard) {
    // rl.begin_drawing(&thread);

    // rl.clear_background(raylib::color::Color::RAYWHITE);

    // Draw the chessboard
    for row in 0..8 {
        for col in 0..8 {
            let square = row * 8 + col;
            let x = col * SQUARE_SIZE;
            let y = row * SQUARE_SIZE;
            rl.draw_rectangle(
                x,
                y,
                SQUARE_SIZE,
                SQUARE_SIZE,
                if (row + col) % 2 == 0 {
                    raylib::color::Color::WHITE
                } else {
                    raylib::color::Color::GRAY
                },
            );
        }
    }
    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = rank * 8 + file;
            let mask = 1u64 << square;
            let mut piece_char = '.';
            let x = file * SQUARE_SIZE;
            let y = rank * SQUARE_SIZE;
            // Draw the piece letter
            for color in 0..2 {
                for (piece_type, &bitboard) in game.pieces[color].iter().enumerate() {
                    if bitboard & mask != 0 {
                        piece_char = match (color, piece_type) {
                            (0, 0) => 'P',
                            (0, 1) => 'N',
                            (0, 2) => 'B',
                            (0, 3) => 'R',
                            (0, 4) => 'Q',
                            (0, 5) => 'K',
                            (1, 0) => 'p',
                            (1, 1) => 'n',
                            (1, 2) => 'b',
                            (1, 3) => 'r',
                            (1, 4) => 'q',
                            (1, 5) => 'k',
                            _ => unreachable!(),
                        };
                        break;
                    }
                }
                if piece_char != '.' {
                    break;
                }
            }
            if piece_char != '.' {
                let font_size = SQUARE_SIZE as f32 * 0.8;
                let x_offset = (SQUARE_SIZE as f32
                    - rl.measure_text(&piece_char.to_string(), font_size as i32) as f32)
                    / 2.0;
                let y_offset = (SQUARE_SIZE as f32 - font_size) / 2.0;
                rl.draw_text(
                    &piece_char.to_string(),
                    (x as f32 + x_offset) as i32,
                    (y as f32 + y_offset) as i32,
                    font_size as i32,
                    raylib::color::Color::BLACK,
                );
            }
        }
    }
}

fn main() {
    let mut board = ChessBoard::new();
    println!("Initial chess board:\n{}", board);

    let (mut rl, thread) = raylib::init()
        .size(8 * SQUARE_SIZE, 8 * SQUARE_SIZE)
        .title("Chess")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // board.draw(&mut d);
        draw_game(&mut d, &thread, &board);
        // game.update(&mut d);
    }
}
