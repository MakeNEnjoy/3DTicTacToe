
use itertools::Itertools;

use crate::{tic_array::tic::*, minmax::{GameState, Heuristic}};
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

#[derive(Clone, PartialEq)]
pub struct BitBoard {
    player1: u128,
    player2: u128,
    last_move: Option<u128>,
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
const WHOLE_BOARD_MASK: u128 = 0b111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111;

fn row_wins_single(counts: u128) -> u128 {
    let onestep = (counts << 1) & COL_MASK;
    let twostep = (counts << 2) & COL_MASK;
    counts & onestep & twostep
}

fn col_wins_single(counts: u128) -> u128 {
    let onestep = (counts << 9) & ROW_MASK;
    let twostep = (counts << 18) & ROW_MASK;
    counts & onestep & twostep
}

fn left_right_wins_single(counts: u128) -> u128 {
    let onestep = (counts << 10) & LEFT_RIGHT_MASK;
    let twostep = (counts << 20) & LEFT_RIGHT_MASK;

    counts & onestep & twostep
}

fn right_left_wins_single(counts: u128) -> u128 {
    let onestep = (counts << 8) & RIGHT_LEFT_MASK;
    let twostep = (counts << 16) & RIGHT_LEFT_MASK;
    counts & onestep & twostep
}

fn almost_wins_single(counts: u128) -> u128 {
    let row = ((counts << 1) & COL_MASK) & counts;
    let row_gap = ((((counts << 1) & COL_MASK) << 1) & COL_MASK) & counts;
    let col = ((counts << 9) & ROW_MASK) & counts;
    let col_gap = ((((counts << 9) & ROW_MASK) << 9) & ROW_MASK) & counts;
    let left_right = ((counts << 10) & LEFT_RIGHT_MASK) & counts;
    let left_right_gap = ((((counts << 10) & LEFT_RIGHT_MASK) << 10) & LEFT_RIGHT_MASK) & counts;
    let right_left = ((counts << 8) & RIGHT_LEFT_MASK) & counts;
    let right_left_gap = ((((counts << 8) & RIGHT_LEFT_MASK) << 8) & RIGHT_LEFT_MASK) & counts;
    let combine = row | col | left_right | right_left
                        | row_gap | col_gap | left_right_gap | right_left_gap;
    let accumulate_into_col = combine | combine << 1 | combine << 2;
    let accumulate = (accumulate_into_col | accumulate_into_col << 9 | accumulate_into_col << 18) & CORNER_MASK;
    accumulate
}

fn all_wins_single(counts: u128) -> u128 {
    let row = row_wins_single(counts);
    let col = col_wins_single(counts);
    let accumulate_left_right = left_right_wins_single(counts);
    let right_left = right_left_wins_single(counts);

    let accumulate_row = (row | (row << 9) | (row << 18)) & CORNER_MASK;

    let accumulate_col = (col | (col << 1) | (col << 2)) & CORNER_MASK;

    let accumulate_right_left = right_left << 2;
    let all_wins = accumulate_col | accumulate_row | accumulate_left_right | accumulate_right_left;
    all_wins
}

fn count_to_mask(counts: u128) -> u128 {
    let col_mask = counts | (counts >> 9) | (counts >> 18);
    col_mask | (col_mask >> 1) | (col_mask >> 2)
}

struct LegalBoards {
    not_allowed_moves: u128,
    original_state: BitBoard,
    who_turn: Player
}

impl Iterator for LegalBoards {
    type Item = BitBoard;

    fn next(&mut self) -> Option<Self::Item> {
        let ones = self.not_allowed_moves.trailing_ones();
        assert!(ones <= 81);
        if ones == 81 {
            return None;
        }
        let next_move: u128 = 1 << ones;
        let next_move_board: u128 = {
            let k: u128 = (3*((ones%(9*3))/9) + ones % 3).into();
            let board_idx = 3*9*(k/3) + (k%3)*3 + 20;
            1 << board_idx
        };
        self.not_allowed_moves = self.not_allowed_moves | next_move;
        assert!((self.original_state.player1 | self.original_state.player2) & next_move == 0);
        let next_board = match self.who_turn {
            Player::Player1 => BitBoard {
                player1: self.original_state.player1 | next_move,
                player2: self.original_state.player2,
                last_move: Some(next_move_board),
            },
            Player::Player2 => BitBoard {
                player1: self.original_state.player1,
                player2: self.original_state.player2 | next_move,
                last_move: Some(next_move_board),
            }
        };
        
        Some(next_board)
    }
}

impl BitBoard {
    pub fn new() -> Self {
        BitBoard { player1: 0, player2: 0, last_move: None }
    }

    fn row_wins(&self) -> BoardCounts {
        let player1 = row_wins_single(self.player1);
        let player2 = row_wins_single(self.player2);
        BoardCounts {player1, player2}
    }

    fn col_wins(&self) -> BoardCounts {
        let player1 = col_wins_single(self.player1);
        let player2 = col_wins_single(self.player2);
        BoardCounts {player1, player2}
    }

    fn left_right_wins(&self) -> BoardCounts {
        let player1 = left_right_wins_single(self.player1);
        let player2 = left_right_wins_single(self.player2);
        BoardCounts {player1, player2}
    }

    fn right_left_wins(&self) -> BoardCounts {
        let player1 = right_left_wins_single(self.player1);
        let player2 = right_left_wins_single(self.player2);
        BoardCounts {player1, player2}
    }

    fn all_wins(&self) -> BoardCounts {
        let player1 = all_wins_single(self.player1);
        let player2 = all_wins_single(self.player2);
        BoardCounts {player1, player2}
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

    fn who_win(&self, wins: BoardCounts) -> Option<Player> {
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
        row_wins_single(col_wins_single(combined))
    }

    fn not_allowed_moves(&self) -> u128 {
        let BoardCounts { player1: wins1, player2: wins2} = self.all_wins();
        let filled_tiles = self.player1 | self.player2;
        let combined_wins = wins1 | wins2;
        let last_move_mask = if let Some(m) = self.last_move {
            if m & (combined_wins | self.full_boards()) == 0 {
                !count_to_mask(m) & WHOLE_BOARD_MASK
            } else {
                0
            }
        } else {
            0
        };
        let wins_mask = count_to_mask(combined_wins);
        wins_mask | filled_tiles | last_move_mask
    }

    fn who_turn(&self) -> Player {
        match (self.player1 | self.player2).count_ones() % 2 {
            0 => Player::Player1,
            1 => Player::Player2,
            _ => panic!("Maths is broken!")
        }   
    }

    fn get_legal_boards(&self) -> impl Iterator<Item = Self> {
        let moves_mask = self.not_allowed_moves();
        let who_turn = self.who_turn();
        LegalBoards {
            original_state: self.clone(),
            not_allowed_moves: moves_mask,
            who_turn: who_turn,
        }
    }
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
            let n = 3*y + x*3*9 + 20;
            Some(1 << n)
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
            let n: usize = m.ilog2().try_into().unwrap();
            Some((0, 0,  n/(3*9) , (n % 9)/3))
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
        self.get_legal_boards().collect_vec()
    }
}

impl Heuristic<BitBoard> for AlmostWinHeuristic {
    type Score = i32;

    fn score(game_state: &BitBoard) -> Self::Score {
        let wins = game_state.all_wins();
        let is_won = game_state.who_win(wins.clone());
        match is_won {
            Some(Player::Player1) => return -1000,
            Some(Player::Player2) => return 1000,
            None => {}
        }
        let wins1 = wins.player1.count_ones();
        let almost_wins1 = almost_wins_single(game_state.player1);
        let almost_wins1_count = (almost_wins1 & !wins.player1).count_ones(); 
        let wins2 = wins.player2.count_ones();
        let almost_wins2 = almost_wins_single(game_state.player2);
        let almost_wins2_count = (almost_wins2 & !wins.player2).count_ones(); 

        (wins2 * 3 + almost_wins2_count - wins1 * 3 - almost_wins1_count).try_into().unwrap()
    }
}

pub struct WinHeuristic {}

impl Heuristic<BitBoard> for WinHeuristic {
    type Score = i32;
    fn score(game_state: &BitBoard) -> Self::Score {
        
        let wins = game_state.all_wins();
        let p1: i32 = wins.player1.count_ones().try_into().unwrap();
        let p2: i32 = wins.player2.count_ones().try_into().unwrap();
        p2 - p1
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::collections::HashSet;
    use std::fs;
    use std::path::PathBuf;

    use crate::tic_array::tic::*;
    use super::*;

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
        assert_eq!(board.board_winner(), bitmap.who_win(bitmap.all_wins()));
    }

    #[rstest]
    fn test_get_legal_moves(#[files("positions/*.txt")] file_path: PathBuf) {
        let board_str = fs::read_to_string(file_path)
            .expect("Failed to read board file");

        let board_result: BoardResult = board_str.as_str().into();
        let board = board_result.into_inner().unwrap();
        let bitmap: BitBoard = board.clone().into();
        let next_boards_bitmap = bitmap.get_legal_boards().collect_vec();
        let mut next_board_board1: Vec<Board> = next_boards_bitmap.into_iter().map(|x| x.into()).collect();
        let mut next_board_board2 = board.next_states();
        next_board_board1.sort();
        next_board_board2.sort();
        assert_eq!(next_board_board1, next_board_board2);
    }

    #[rstest]
    fn test_almost_wins(#[files("positions/*.txt")] file_path: PathBuf) {
        let board_str = fs::read_to_string(file_path)
            .expect("Failed to read board file");

        let board_result: BoardResult = board_str.as_str().into();
        let board = board_result.into_inner().unwrap();
        let bitmap: BitBoard = board.clone().into();
        println!("{:?}", D(bitmap.player1));
        println!("{:?}", D(almost_wins_single(bitmap.player1)));
        // assert!(false);
    }
}