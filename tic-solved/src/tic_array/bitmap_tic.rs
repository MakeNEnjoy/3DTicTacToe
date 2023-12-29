
use crate::{tic_array::tic::*, minmax::GameState};
use std::fmt;

struct D(u128);

impl fmt::Debug for D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let board: Board = self.into();
        let bit_board = BitBoard { player1: self.0, player2: 0, last_move: None };
        let board: Board = bit_board.into();
        write!(f, "D(\n{})", board)
    }
}

#[derive(Clone)]
struct BoardCounts {
    player1: u128,
    player2: u128
}

impl fmt::Debug for BoardCounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BoardCounts {{\nplayer1: {:0>128b},\nplayer2: {:0>128b}\n}}", self.player1, self.player2)
    }
}

#[derive(Clone)]
struct BitBoard {
    player1: u128,
    player2: u128,
    last_move: Option<u8>,
}

impl fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BitBoard {{ \nplayer1: {:0>128b},\nplayer2: {:0>128b},\n last_move: {:?} }}", self.player1, self.player2, self.last_move)
    }
}

const COL_MASK: u128 = 0b110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110;
const ROW_MASK: u128 = 0b111_111_111_111_111_111_000_000_000_111_111_111_111_111_111_000_000_000_111_111_111_111_111_111_000_000_000;
const LEFT_RIGHT_MASK: u128 = 0b100_100_100_010_010_010_000_000_000_100_100_100_010_010_010_000_000_000_100_100_100_010_010_010_000_000_000;
const RIGHT_LEFT_MASK: u128 = 0b001_001_001_010_010_010_000_000_000_001_001_001_010_010_010_000_000_000_001_001_001_010_010_010_000_000_000;
const OVERALL_COL_MASK: u128 = 0b111_111_111_111_111_111_000_000_000_111_111_111_111_111_111_000_000_000_111_111_111_111_111_111_000_000_000;
const OVERALL_ROW_MASK: u128 = 0b110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110_110;
const OVERALL_LEFT_RIGHT_WIN: u128 = 0b100_000_000_000_000_000_000_000_000_000_100_000_000_000_000_000_000_000_000_000_100_000_000_000_000_000_000;
const OVERALL_RIGHT_LEFT_WIN: u128 = 0b000_000_100_000_000_000_000_000_000_000_100_000_000_000_000_000_000_000_100_000_000_000_000_000_000_000_000;
const CORNER_MASK: u128 = 0b100_100_100_000_000_000_000_000_000_100_100_100_000_000_000_000_000_000_100_100_100_000_000_000_000_000_000;

impl BitBoard {
    pub fn new() -> Self {
        BitBoard { player1: 0, player2: 0, last_move: None }
    }
    
    fn row_wins(&self) -> BoardCounts {
        let onestep = (self.player1 << 1) & COL_MASK;
        let twostep = (self.player1 << 2) & COL_MASK;
        let player1 = self.player1 & onestep & twostep;
        let onestep = (self.player2 << 1) & COL_MASK;
        let twostep = (self.player2 << 2) & COL_MASK;
        let player2 = self.player2 & onestep & twostep;
        BoardCounts {player1, player2}
    }

    fn col_wins(&self) -> BoardCounts {
        let onestep = (self.player1 << 9) & ROW_MASK;
        let twostep = (self.player1 << 18) & ROW_MASK;
        let player1 = self.player1 & onestep & twostep;
        let onestep = (self.player2 << 9) & ROW_MASK;
        let twostep = (self.player2 << 18) & ROW_MASK;
        let player2 = self.player2 & onestep & twostep;
        
        BoardCounts {player1, player2}
    }

    fn left_right_wins(&self) -> BoardCounts {
        let onestep = (self.player1 << 10) & LEFT_RIGHT_MASK;
        let twostep = (self.player1 << 20) & LEFT_RIGHT_MASK;
        let player1 = self.player1 & onestep & twostep;
        let onestep = (self.player2 << 10) & LEFT_RIGHT_MASK;
        let twostep = (self.player2 << 20) & LEFT_RIGHT_MASK;
        let player2 = self.player2 & onestep & twostep;
        BoardCounts {player1, player2}
    }

    fn right_left_wins(&self) -> BoardCounts {
        let onestep = (self.player1 << 8) & RIGHT_LEFT_MASK;
        let twostep = (self.player1 << 16) & RIGHT_LEFT_MASK;
        let player1 = self.player1 & onestep & twostep;
        let onestep = (self.player2 << 8) & RIGHT_LEFT_MASK;
        let twostep = (self.player2 << 16) & RIGHT_LEFT_MASK;
        let player2 = self.player2 & onestep & twostep;
        BoardCounts {player1, player2}
    }

    fn all_wins(&self) -> BoardCounts {
        let BoardCounts {player1: row1, player2: row2} = self.row_wins();
        let BoardCounts {player1: col1, player2: col2} = self.col_wins();
        let BoardCounts {player1: accumulate_left_right1, player2: accumulate_left_right2} = self.left_right_wins();
        let BoardCounts {player1: right_left1, player2: right_left2} = self.right_left_wins();
        
        let accumulate_row1 = (row1 | (row1 << 9) | (row1 << 18)) & CORNER_MASK;
        let accumulate_row2 = (row2 | (row2 << 9) | (row2 << 18)) & CORNER_MASK;

        let accumulate_col1 = (col1 | (col1 << 1) | (col1 << 2)) & CORNER_MASK;
        let accumulate_col2 = (col2 | (col2 << 1) | (col2 << 2)) & CORNER_MASK;

        let accumulate_right_left1 = right_left1 << 2;
        let accumulate_right_left2 = right_left2 << 2;
        let all_wins1 = accumulate_col1 | accumulate_row1 | accumulate_left_right1 | accumulate_right_left1;
        let all_wins2 = accumulate_col2 | accumulate_row2 | accumulate_left_right2 | accumulate_right_left2;
        BoardCounts { player1: all_wins1, player2: all_wins2 }
    }

    fn overall_col_win(&self, BoardCounts{player1, player2}: BoardCounts) -> Option<Player> {
        let onestep = (player1 << 27) & OVERALL_COL_MASK;
        let twostep = (player1 << 54) & OVERALL_COL_MASK;
        let player1 = player1 & onestep & twostep;
        let onestep = (player2 << 27) & OVERALL_COL_MASK;
        let twostep = (player2 << 54) & OVERALL_COL_MASK;
        let player2 = player2 & onestep & twostep;
        assert!(!((player1 > 0) && (player2 > 0)));
        if player1 > 0 {
            Some(Player::Player1)
        } else if player2 > 0 {
            Some(Player::Player2)
        } else {
            None
        }
    }

    fn overall_row_win(&self, BoardCounts{player1, player2}: BoardCounts) -> Option<Player> {
        let onestep = (player1 << 3) & OVERALL_ROW_MASK;
        let twostep = (player1 << 6) & OVERALL_ROW_MASK;
        let player1 = player1 & onestep & twostep;
        let onestep = (player2 << 3) & OVERALL_ROW_MASK;
        let twostep = (player2 << 6) & OVERALL_ROW_MASK;
        let player2 = player2 & onestep & twostep;
        assert!(!((player1 > 0) && (player2 > 0)));
        if player1 > 0 {
            Some(Player::Player1)
        } else if player2 > 0 {
            Some(Player::Player2)
        } else {
            None
        }
    }

    fn overall_left_right_win(&self, BoardCounts{player1, player2}: BoardCounts) -> Option<Player> {
        let player1_win = player1 & OVERALL_LEFT_RIGHT_WIN == OVERALL_LEFT_RIGHT_WIN;
        let player2_win = player2 & OVERALL_LEFT_RIGHT_WIN == OVERALL_LEFT_RIGHT_WIN;
        assert!(!(player1_win && player2_win));
        if player1_win {
            Some(Player::Player1)
        } else if player2_win {
            Some(Player::Player2)
        } else {
            None
        }
    }

    fn overall_right_left_win(&self, BoardCounts{player1, player2}: BoardCounts) -> Option<Player> {
        let player1_win = player1 & OVERALL_RIGHT_LEFT_WIN == OVERALL_RIGHT_LEFT_WIN;
        let player2_win = player2 & OVERALL_RIGHT_LEFT_WIN == OVERALL_RIGHT_LEFT_WIN;
        assert!(!(player1_win && player2_win));
        if player1_win {
            Some(Player::Player1)
        } else if player2_win {
            Some(Player::Player2)
        } else {
            None
        }
    }

    pub fn who_win(&self) -> Option<Player> {
        let wins = self.all_wins();
        let row = self.overall_row_win(wins.clone());
        let col = self.overall_col_win(wins.clone());
        let left_right = self.overall_left_right_win(wins.clone());
        let right_left = self.overall_right_left_win(wins);
        let mut cumalative_win = None;
        if row.is_some() {
            cumalative_win = row;
        }
        if col.is_some() {
            assert!(cumalative_win.is_none() || col == cumalative_win);
            cumalative_win = col;
        }
        if left_right.is_some() {
            assert!(cumalative_win.is_none() || left_right == cumalative_win);
            cumalative_win = left_right;
        }
        if right_left.is_some() {
            assert!(cumalative_win.is_none() || right_left == cumalative_win);
            cumalative_win = right_left;
        }
        cumalative_win
    }

    fn full_boards(&self) -> u128 {
        let combined = self.player1 | self.player2;
        todo!()    
    }

    fn not_allowed_moves(&self) -> BoardCounts {
        let BoardCounts {player1: row1, player2: row2} = self.row_wins();
        let BoardCounts {player1: col1, player2: col2} = self.col_wins();
        let BoardCounts {player1: accumulate_left_right1, player2: accumulate_left_right2} = self.left_right_wins();
        let BoardCounts {player1: right_left1, player2: right_left2} = self.right_left_wins();
        
        let accumulate_row1 = (row1 | (row1 << 9) | (row1 << 18)) & CORNER_MASK;
        let accumulate_row2 = (row2 | (row2 << 9) | (row2 << 18)) & CORNER_MASK;

        let accumulate_col1 = (col1 | (col1 << 1) | (col1 << 2)) & CORNER_MASK;
        let accumulate_col2 = (col2 | (col2 << 1) | (col2 << 2)) & CORNER_MASK;

        let accumulate_right_left1 = right_left1 << 2;
        let accumulate_right_left2 = right_left2 << 2;
        let all_wins1 = accumulate_col1 | accumulate_row1 | accumulate_left_right1 | accumulate_right_left1;
        let all_wins2 = accumulate_col2 | accumulate_row2 | accumulate_left_right2 | accumulate_right_left2;
        BoardCounts { player1: all_wins1, player2: all_wins2 }  
    }

    // fn get_legal_boards(&self) -> impl Iterator<Item = Self> {
    //     todo!()
    // }
}

impl From<Board> for BitBoard {
    fn from(board: Board) -> Self {
        let (cells, last_to_move) = board.to_arr();
        let mut player1 = 0;
        let mut player2 = 0;
        let mut marker: u128 = 1;
        for i in 0..3 {
            for k in 0..3 {
                for j in 0..3 {
                    for l in 0..3 {
                        if cells[i][j][k][l] == 1 {
                            player1 += marker;
                        } else if cells[i][j][k][l] == 2 {
                            player2 += marker;
                        }
                        marker = marker << 1;
                    }
                }
            }
        }
        let last_move = if let Some((_, _, x, y)) = last_to_move {
            Some((3*x + y).try_into().unwrap())
        } else {
            None
        };
        BitBoard {player1, player2, last_move}
    }
}

impl Into<Board> for BitBoard {
    fn into(self) -> Board {
        let mut cells = [[[[0; 3]; 3]; 3]; 3];
        let mut marker: u128 = 1;
        for i in 0..3 {
            for k in 0..3 {
                for j in 0..3 {
                    for l in 0..3 {
                        cells[i][j][k][l] = if marker & self.player1 > 0 {
                            1
                        } else if marker & self.player2 > 0 {
                            2 
                        } else {
                            0
                        };
                        marker = marker << 1;
                    }
                }
            }
        }
        let last_move: Option<(usize, usize, usize, usize)> = if let Some(m) = self.last_move {
            Some((0, 0, ( m/3 ).into(), (m % 3).into()))
        } else {
            None
        };
        Board::create_board(
            cells,
            last_move,
        ).unwrap()
    }
}

impl GameState for BitBoard {
    fn next_states(&self) -> Vec<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::fs;
    use std::path::PathBuf;
    use crate::tic_array::tic::*;

    use super::BitBoard;

    #[rstest]
    fn test_bitmap_conversion(#[files("positions/*.txt")] file_path: PathBuf) {
        let board_str = fs::read_to_string(file_path)
            .expect("Failed to read board file");

        let board_result: BoardResult = board_str.as_str().into();
        let board = board_result.into_inner().unwrap();
        let bitmap: BitBoard = board.clone().into();
        let board_copy: Board = bitmap.clone().into();
        assert_eq!(board, board_copy);
    }

    #[rstest]
    fn test_who_win(#[files("positions/gameover/*.txt")] file_path: PathBuf) {
        let board_str = fs::read_to_string(file_path)
            .expect("Failed to read board file");

        let board_result: BoardResult = board_str.as_str().into();
        let board = board_result.into_inner().unwrap();
        let bitmap: BitBoard = board.clone().into();
        assert_eq!(board.board_winner(), bitmap.who_win());
    }
}