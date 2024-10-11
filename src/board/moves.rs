
use crate::board::board::*;

// File masks (vertical)
const FILE_A: u64 = 0x0101010101010101;
const FILE_B: u64 = 0x0202020202020202;
const FILE_C: u64 = 0x0404040404040404;
const FILE_D: u64 = 0x0808080808080808;
const FILE_E: u64 = 0x1010101010101010;
const FILE_F: u64 = 0x2020202020202020;
const FILE_G: u64 = 0x4040404040404040;
const FILE_H: u64 = 0x8080808080808080;

// Rank masks (horizontal)
const RANK_1: u64 = 0x00000000000000FF;
const RANK_2: u64 = 0x000000000000FF00;
const RANK_3: u64 = 0x0000000000FF0000;
const RANK_4: u64 = 0x00000000FF000000;
const RANK_5: u64 = 0x000000FF00000000;
const RANK_6: u64 = 0x0000FF0000000000;
const RANK_7: u64 = 0x00FF000000000000;
const RANK_8: u64 = 0xFF00000000000000;

// Not file masks (useful for preventing wrapping)
const NOT_FILE_A: u64 = !FILE_A;
const NOT_FILE_H: u64 = !FILE_H;
const NOT_FILE_AB: u64 = !(FILE_A | FILE_B);
const NOT_FILE_GH: u64 = !(FILE_G | FILE_H);

pub struct MoveGen {
    all_pieces: Bitboard,
    white_pieces: Bitboard,
    black_pieces: Bitboard,
}

impl MoveGen {
    // Generate knight moves for a single knight
    fn knight_moves_from_square(&self, square: u32, side_pieces: Bitboard) -> Bitboard {
        let knight_bb = 1u64 << square;
        
        // Knight move patterns: 2 squares in one direction, 1 square perpendicular
        let moves = (knight_bb << 17) | (knight_bb << 15) |
                   (knight_bb << 10) | (knight_bb << 6)  |
                   (knight_bb >> 17) | (knight_bb >> 15) |
                   (knight_bb >> 10) | (knight_bb >> 6);

        // Handle file wrapping (when knight is on a/h files)
        let no_wrap = moves & !(
            if knight_bb & FILE_A != 0 { FILE_G | FILE_H } 
            else if knight_bb & FILE_H != 0 { FILE_A | FILE_B }
            else { 0 }
        );

        // Remove moves to squares occupied by friendly pieces
        no_wrap & !side_pieces
    }

    // Generate bishop moves (sliding piece)
    fn bishop_moves_from_square(&self, square: u32) -> Bitboard {
        let bishop_bb = 1u64 << square;
        let mut moves = 0u64;

        // Generate moves in all four diagonal directions
        for &direction in &[7, 9, -7, -9] {
            let mut possible = bishop_bb;
            loop {
                possible = self.shift_safe(possible, direction);
                if possible == 0 { break; }
                
                moves |= possible;
                
                // Stop if we hit a piece
                if possible & self.all_pieces != 0 { break; }
            }
        }

        // Remove moves to squares occupied by friendly pieces
        moves & !self.white_pieces
    }

    // Generate rook moves using "magic bitboards" approach
    fn rook_moves_from_square(&self, square: u32) -> Bitboard {
        let rook_bb = 1u64 << square;
        let mut moves = 0u64;

        // Pre-calculated attacks for each direction
        const ROOK_MASKS: [u64; 4] = [
            0x0101010101010101, // North
            0x00000000000000FF, // East
            0x8080808080808080, // South
            0xFF00000000000000, // West
        ];

        for direction in 0..4 {
            let mask = ROOK_MASKS[direction];
            let blockers = self.all_pieces & mask;
            
            // Use magic multiplication to get attacks
            // (simplified version - real implementations use pre-calculated lookup tables)
            moves |= self.get_rook_attacks(square, blockers, direction);
        }

        // Remove moves to squares occupied by friendly pieces
        moves & !self.white_pieces
    }

    // Helper function for safe bit shifting (prevents wrapping)
    fn shift_safe(&self, bb: Bitboard, shift: i32) -> Bitboard {
        if shift > 0 {
            if shift >= 64 { return 0; }
            bb << shift
        } else {
            if -shift >= 64 { return 0; }
            bb >> -shift
        }
    }

    fn get_rook_attacks(&self, square: u32, blockers: Bitboard, direction: usize) -> Bitboard {
        let rook_bb = 1u64 << square;
        let mut attacks = 0u64;

        // Define shift amounts for each direction
        const SHIFT_AMOUNTS: [i32; 4] = [
            8,   // North
            1,   // East
            -8,  // South
            -1,  // West
        ];

        let shift = SHIFT_AMOUNTS[direction];
        let mut possible = rook_bb;

        loop {
            possible = self.shift_safe(possible, shift);
            if possible == 0 { break; }
            
            attacks |= possible;
            
            // Stop if we hit a blocker
            if possible & blockers != 0 { break; }
        }

        attacks
    }
}

// Supporting structures
#[derive(Copy, Clone)]
struct Move {
    from: u32,
    to: u32,
    piece: PieceType,
    capture: Option<PieceType>,
    promotion: Option<PieceType>,
}