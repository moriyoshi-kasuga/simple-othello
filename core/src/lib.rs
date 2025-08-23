#[derive(Debug, enum_table::Enumable, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Hash)]
pub enum OthelloColor {
    Black,
    White,
}

pub struct OthelloBoard {
    pub black: u64,
    pub white: u64,
    pub turn: u8,
}

macro_rules! line {
    ($data:expr, $start:expr,<<,$n:expr) => {{
        let mut result = $data & ($start << $n);
        result |= $data & (result << $n);
        result |= $data & (result << $n);
        result |= $data & (result << $n);
        result |= $data & (result << $n);
        result |= $data & (result << $n);
        result
    }};
    ($data:expr, $start:expr,>>,$n:expr) => {{
        let mut result = $data & ($start >> $n);
        result |= $data & (result >> $n);
        result |= $data & (result >> $n);
        result |= $data & (result >> $n);
        result |= $data & (result >> $n);
        result |= $data & (result >> $n);
        result
    }};
}

impl OthelloBoard {
    const LR_EDGE_MASK: u64 = 0x7e7e7e7e7e7e7e7e;
    const TB_EDGE_MASK: u64 = 0x00FFFFFFFFFFFF00;
    const LTRB_EDGE_MASK: u64 = 0x007e7e7e7e7e7e00;
    const SHIFT_MASK_LIST: [(u32, u64); 4] = [
        (1, Self::LR_EDGE_MASK),
        (8, Self::TB_EDGE_MASK),
        (7, Self::LTRB_EDGE_MASK),
        (9, Self::LTRB_EDGE_MASK),
    ];

    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        OthelloBoard {
            black: 0x0000000810000000,
            white: 0x0000001008000000,
            turn: 0,
        }
    }

    #[inline]
    pub const fn get_turn(&self) -> OthelloColor {
        if self.turn % 2 == 0 {
            OthelloColor::Black
        } else {
            OthelloColor::White
        }
    }

    #[inline]
    pub const fn get_current_player_and_opponent(&self) -> (u64, u64) {
        match self.get_turn() {
            OthelloColor::Black => (self.black, self.white),
            OthelloColor::White => (self.white, self.black),
        }
    }

    pub fn legal_moves(&self) -> u64 {
        let (player, opponent) = self.get_current_player_and_opponent();
        let empty_squares = !(player | opponent);

        let mut legal = 0;
        for (shift, mask) in Self::SHIFT_MASK_LIST {
            // Moves in the "positive" direction (e.g., right, down)
            let mut line = line!(opponent & mask, player, >>, shift);
            legal |= line >> shift;

            // Moves in the "negative" direction (e.g., left, up)
            line = line!(opponent & mask, player, <<, shift);
            legal |= line << shift;
        }

        legal & empty_squares
    }

    #[inline]
    pub fn can_place(&self, pos: u64) -> bool {
        pos.count_ones() == 1 && (self.legal_moves() & pos) != 0
    }

    #[inline]
    pub fn can_place_pos(&self, x: u8, y: u8) -> bool {
        if x >= 8 || y >= 8 {
            return false;
        }
        let pos = 1u64 << (y * 8 + x);
        self.can_place(pos)
    }

    pub fn place_and_return_inversions(&mut self, pos: u64) -> u64 {
        let turn = self.get_turn();
        let player_before = match turn {
            OthelloColor::Black => self.black,
            OthelloColor::White => self.white,
        };
        if !self.place(pos) {
            return 0;
        }
        let player_after = match turn {
            OthelloColor::Black => self.black,
            OthelloColor::White => self.white,
        };
        player_before ^ player_after ^ pos
    }

    pub fn place(&mut self, pos: u64) -> bool {
        if !self.can_place(pos) {
            return false;
        }

        let (mut player, mut opponent) = self.get_current_player_and_opponent();

        let mut to_flip = 0;
        for (shift, mask) in Self::SHIFT_MASK_LIST {
            // Flips in the "positive" direction
            let line1 = line!(opponent & mask, pos, >>, shift);
            if ((line1 >> shift) & player) != 0 {
                to_flip |= line1;
            }

            // Flips in the "negative" direction
            let line2 = line!(opponent & mask, pos, <<, shift);
            if ((line2 << shift) & player) != 0 {
                to_flip |= line2;
            }
        }

        player ^= to_flip | pos;
        opponent ^= to_flip;

        match self.get_turn() {
            OthelloColor::Black => {
                self.black = player;
                self.white = opponent;
            }
            OthelloColor::White => {
                self.white = player;
                self.black = opponent;
            }
        }
        self.turn += 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_moves_and_place() {
        let mut board = OthelloBoard::new();
        // Initial legal moves for Black.
        // c4(26), d3(19), f5(37), e6(44)
        assert_eq!(
            board.legal_moves(),
            (1 << 19) | (1 << 26) | (1 << 37) | (1 << 44)
        );

        // Place at f5(37), which is a legal move.
        assert!(board.place(1 << 37));
        // Now it's White's turn.
        assert_eq!(board.get_turn(), OthelloColor::White);

        // After Black's move, e5(36) is flipped.
        // Black: d5(35), e4(28), f5(37), e5(36)
        // White: d4(27)
        assert_eq!(board.black, (1 << 35) | (1 << 28) | (1 << 37) | (1 << 36));
        assert_eq!(board.white, 1 << 27);

        // Legal moves for White should be f4(29), d6(43), f6(45).
        assert_eq!(board.legal_moves(), (1 << 29) | (1 << 43) | (1 << 45));

        // Try to place at an illegal position.
        assert!(!board.place(1 << 0));
        // Board state should not change.
        assert_eq!(board.get_turn(), OthelloColor::White);
        assert_eq!(board.black, (1 << 35) | (1 << 28) | (1 << 37) | (1 << 36));
        assert_eq!(board.white, 1 << 27);
    }

    #[test]
    fn test_flip_horizontal() {
        let mut board = OthelloBoard {
            black: 1 << 0,                         // a1
            white: (1 << 1) | (1 << 2) | (1 << 3), // b1, c1, d1
            turn: 0,                               // Black's turn
        };
        // Legal move for black should be e1 (1<<4)
        assert_eq!(board.legal_moves(), 1 << 4);
        assert!(board.place(1 << 4));

        // Check board state: all pieces from a1 to e1 should be black
        let expected_black = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4);
        assert_eq!(board.black, expected_black);
        assert_eq!(board.white, 0);
        assert_eq!(board.get_turn(), OthelloColor::White);
    }

    #[test]
    fn test_flip_vertical() {
        let mut board = OthelloBoard {
            black: 1 << 0,                           // a1
            white: (1 << 8) | (1 << 16) | (1 << 24), // a2, a3, a4
            turn: 0,                                 // Black's turn
        };
        // Legal move for black should be a5 (1<<32)
        assert_eq!(board.legal_moves(), 1 << 32);
        assert!(board.place(1 << 32));

        // Check board state
        let expected_black = (1 << 0) | (1 << 8) | (1 << 16) | (1 << 24) | (1 << 32);
        assert_eq!(board.black, expected_black);
        assert_eq!(board.white, 0);
    }

    #[test]
    fn test_flip_diagonal_a1_h8() {
        let mut board = OthelloBoard {
            black: 1 << 0,                           // a1
            white: (1 << 9) | (1 << 18) | (1 << 27), // b2, c3, d4
            turn: 0,                                 // Black's turn
        };
        // Legal move for black should be e5 (1<<36)
        assert_eq!(board.legal_moves(), 1 << 36);
        assert!(board.place(1 << 36));

        // Check board state
        let expected_black = (1 << 0) | (1 << 9) | (1 << 18) | (1 << 27) | (1 << 36);
        assert_eq!(board.black, expected_black);
        assert_eq!(board.white, 0);
    }

    #[test]
    fn test_flip_diagonal_h1_a8() {
        let mut board = OthelloBoard {
            black: 1 << 7,                            // h1
            white: (1 << 14) | (1 << 21) | (1 << 28), // g2, f3, e4
            turn: 0,                                  // Black's turn
        };
        // Legal move for black should be d5 (1<<35)
        assert_eq!(board.legal_moves(), 1 << 35);
        assert!(board.place(1 << 35));

        // Check board state
        let expected_black = (1 << 7) | (1 << 14) | (1 << 21) | (1 << 28) | (1 << 35);
        assert_eq!(board.black, expected_black);
        assert_eq!(board.white, 0);
    }

    #[test]
    fn test_place_and_return_inversions() {
        let mut board = OthelloBoard::new();

        // 1. Test a legal move and check the returned flipped mask.
        // Placing at f5(37) should flip e5(36).
        let flipped = board.place_and_return_inversions(1 << 37);
        assert_eq!(flipped, 1 << 36);

        // Verify the board state is updated correctly.
        assert_eq!(board.get_turn(), OthelloColor::White);
        let expected_black = (1 << 35) | (1 << 28) | (1 << 37) | (1 << 36); // d5, e4, f5, e5
        let expected_white = 1 << 27; // d4
        assert_eq!(board.black, expected_black);
        assert_eq!(board.white, expected_white);

        // 2. Test an illegal move.
        // The board state is now after the first move.
        // Placing at a1(0) is illegal for White.
        let flipped_illegal = board.place_and_return_inversions(1 << 0);
        assert_eq!(flipped_illegal, 0);

        // Verify the board state has not changed.
        assert_eq!(board.get_turn(), OthelloColor::White);
        assert_eq!(board.black, expected_black);
        assert_eq!(board.white, expected_white);

        // 3. Test another legal move for White.
        // Placing at f4(29) should flip e4(28).
        let flipped2 = board.place_and_return_inversions(1 << 29);
        assert_eq!(flipped2, 1 << 28);
        assert_eq!(board.get_turn(), OthelloColor::Black);

        let expected_black2 = expected_black ^ (1 << 28); // d5, f5, e5
        let expected_white2 = expected_white | (1 << 29) | (1 << 28); // d4, f4, e4
        assert_eq!(board.black, expected_black2);
        assert_eq!(board.white, expected_white2);
    }
}
