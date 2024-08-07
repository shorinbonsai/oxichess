use std::fmt;

type Bitboard = u64;

#[derive(Copy, Clone, Debug)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug)]
enum Color {
    White,
    Black,
}

struct ChessBoard {
    pieces: [[Bitboard; 6]; 2],
    occupancy: [Bitboard; 2],
    all_pieces: Bitboard,
    side_to_move: Color,
    castling_rights: u8,
    en_passant: Option<u8>,
    halfmove_clock: u8,
    fullmove_number: u16,
}

impl ChessBoard {
    fn new() -> Self {
        let mut board = ChessBoard {
            pieces: [[0; 6]; 2],
            occupancy: [0; 2],
            all_pieces: 0,
            side_to_move: Color::White,
            castling_rights: 0b1111, // KQkq
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        };
        board.set_starting_position();
        board
    }

    fn set_starting_position(&mut self) {
        // White pieces
        self.pieces[Color::Black as usize][PieceType::Pawn as usize] = 0x00FF000000000000;
        self.pieces[Color::Black as usize][PieceType::Rook as usize] = 0x8100000000000000;
        self.pieces[Color::Black as usize][PieceType::Knight as usize] = 0x4200000000000000;
        self.pieces[Color::Black as usize][PieceType::Bishop as usize] = 0x2400000000000000;
        self.pieces[Color::Black as usize][PieceType::Queen as usize] = 0x0800000000000000;
        self.pieces[Color::Black as usize][PieceType::King as usize] = 0x1000000000000000;

        // Black pieces
        self.pieces[Color::White as usize][PieceType::Pawn as usize] = 0x000000000000FF00;
        self.pieces[Color::White as usize][PieceType::Rook as usize] = 0x0000000000000081;
        self.pieces[Color::White as usize][PieceType::Knight as usize] = 0x0000000000000042;
        self.pieces[Color::White as usize][PieceType::Bishop as usize] = 0x0000000000000024;
        self.pieces[Color::White as usize][PieceType::Queen as usize] = 0x0000000000000008;
        self.pieces[Color::White as usize][PieceType::King as usize] = 0x0000000000000010;

        self.update_occupancy();
    }

    fn update_occupancy(&mut self) {
        self.occupancy = [0; 2];
        self.all_pieces = 0;

        for color in 0..2 {
            for piece_bb in self.pieces[color].iter() {
                self.occupancy[color] |= piece_bb;
                self.all_pieces |= piece_bb;
            }
        }
    }

    // Add more methods for move generation, making moves, etc.
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                let mask = 1u64 << square;
                let mut piece_char = '.';

                for color in 0..2 {
                    for (piece_type, &bitboard) in self.pieces[color].iter().enumerate() {
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

                write!(f, "{} ", piece_char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let board = ChessBoard::new();
    println!("Initial chess board:\n{}", board);
}
