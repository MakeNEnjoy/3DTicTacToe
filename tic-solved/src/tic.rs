use std::fmt;
use itertools::iproduct;

use crate::game::GameState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Player {
    Player1,
    Player2
}

impl Player {
    fn get_other_player(&self) -> Player {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Player1,
    Player2,
    Empty
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Player1 => write!(f, "X"),
            Tile::Player2 => write!(f, "O"),
            Tile::Empty => write!(f, "."),
        }
    }
}

impl From<Player> for Tile {
    fn from(player: Player) -> Self {
        match player {
            Player::Player1 => Tile::Player1,
            Player::Player2 => Tile::Player2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    cells: [[[[Tile; 3]; 3]; 3]; 3],
    player_to_move: Player,
    last_move: Option<(usize, usize, usize, usize)>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[[[Tile::Empty; 3]; 3]; 3]; 3],
            player_to_move: Player::Player1,
            last_move: None,
        }
    }
    pub fn is_single_board_winning(&self, x0: usize, x1: usize) -> bool {
        // Check horizontal lines
        for x2 in 0..3 {
            if self.cells[x0][x1][x2][0] == self.cells[x0][x1][x2][1] && self.cells[x0][x1][x2][1] == self.cells[x0][x1][x2][2] {
                if self.cells[x0][x1][x2][0] != Tile::Empty {
                    return true;
                }
            }
        }

        // Check vertical lines
        for x3 in 0..3 {
            if self.cells[x0][x1][0][x3] == self.cells[x0][x1][1][x3] && self.cells[x0][x1][1][x3] == self.cells[x0][x1][2][x3] {
                if self.cells[x0][x1][0][x3] != Tile::Empty {
                    return true;
                }
            }
        }

        // Check diagonal lines
        if self.cells[x0][x1][0][0] == self.cells[x0][x1][1][1] && self.cells[x0][x1][1][1] == self.cells[x0][x1][2][2] {
            if self.cells[x0][x1][0][0] != Tile::Empty {
                return true;
            }
        }
        if self.cells[x0][x1][0][2] == self.cells[x0][x1][1][1] && self.cells[x0][x1][1][1] == self.cells[x0][x1][2][0] {
            if self.cells[x0][x1][0][2] != Tile::Empty {
                return true;
            }
        }

        false
    }
    fn is_board_winning(&self) -> bool {
        // Check horizontal lines
        for x0 in 0..3 {
            for x1 in 0..3 {
                if self.is_single_board_winning(x0, x1) {
                    return true;
                }
            }
        }

        // Check vertical lines
        for x0 in 0..3 {
            for x2 in 0..3 {
                if self.is_single_board_winning(x0, x2) {
                    return true;
                }
            }
        }

        // Check diagonal lines
        if self.is_single_board_winning(0, 0) || self.is_single_board_winning(1, 1) || self.is_single_board_winning(2, 2) {
            return true;
        }
        if self.is_single_board_winning(0, 2) || self.is_single_board_winning(1, 1) || self.is_single_board_winning(2, 0) {
            return true;
        }

        false
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..3 {
            for k in 0..3 {
                for j in 0..3 {
                    for l in 0..3 {
                        let tile = self.cells[i][j][k][l];
                        write!(f, "{}", tile)?;
                    }
                    write!(f, " ")?;
                }
                write!(f, "\n")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

struct TicMove<'a> {
    move_to_make: (usize, usize, usize, usize),
    board: &'a Board,
}

impl<'a> TicMove<'a> {
    fn new((x1, x2, x3, x4): (usize, usize, usize, usize), board: &Board) -> Option<TicMove> {
        let tile = board.cells[x1][x2][x3][x4];
        let tic_move = TicMove {
            move_to_make: (x1, x2, x3, x4),
            board
        };
        if !tic_move.move_in_valid_board() || board.is_board_winning() {
            return None;
        }
        match tile {
            Tile::Empty => Some(tic_move),
            Tile::Player1 => None,
            Tile::Player2 => None,
        }
    }

    fn move_in_valid_board(&self) -> bool {
        let (x1,x2,_,_) = self.move_to_make;
        if let Some((_,_,y3,y4)) = self.board.last_move {
            x1 == y3 && x2 == y4
        } else {
            true
        }
    }

    fn iter_moves(board: &Board) -> impl Iterator<Item = TicMove> {
        iproduct!(0..3, 0..3, 0..3, 0..3)
        .filter_map(|pos| TicMove::new(pos, board))

    }

    fn do_move(&self) -> Board {
        let mut new_cells = self.board.cells.clone();
        let (x1, x2, x3, x4) = self.move_to_make;
        new_cells[x1][x2][x3][x4] = self.board.player_to_move.into();
        Board {
            cells: new_cells,
            player_to_move: self.board.player_to_move.get_other_player(),
            last_move: Some(self.move_to_make),
        }
    }
}

impl GameState for Board {
    type Implementation = Board;
    fn next_states(&self) -> Vec<Board> {
        let mut states: Vec<Board> = Vec::new();
        for m in TicMove::iter_moves(self).map(|m| m.do_move()) {
            states.push(m);
        }
        states
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_move() {
        let board = Board::new();
        let tic_move = TicMove::new( 
            (1,1,0,2),
            &board
        ).unwrap();
        let new_board = tic_move.do_move();
        println!("Old board \n{}", board);
        println!("New board \n{}", new_board);
    }

    #[test]
    fn test_move_nonempty_cell() {
        let board = Board::new();
        let tic_move = TicMove::new( 
            (1,1,0,2),
            &board
        ).unwrap();
        let new_board = tic_move.do_move();
        let tic_move = TicMove::new( 
            (1,1,0,2),
            &new_board
        );
        assert!(tic_move.is_none());
    }

    #[test]
    fn test_move_invalid_board() {
        let board = Board::new();
        let tic_move = TicMove::new( 
            (1,1,0,2),
            &board
        ).unwrap();
        let new_board = tic_move.do_move();
        let tic_move = TicMove::new( 
            (0,1,1,1),
            &new_board
        );
        assert!(tic_move.is_none());
    }


    #[test]
    fn test_do_multiple_moves() {
        let board = Board::new();
        let moves = board.next_states();
        let board2 = moves.get(11).unwrap();
        let moves = board2.next_states();
        let board3 = moves.get(4).unwrap();

        println!("Board \n{}", board3);
    }

    #[test]
    fn test_is_single_board_winning_row() {
        let mut board = Board::new();
        board.cells[0][0][0][0] = Tile::Player1;
        board.cells[0][0][0][1] = Tile::Player1;
        board.cells[0][0][0][2] = Tile::Player1;

        assert!(board.is_single_board_winning(0, 0));
    }

    #[test]
    fn test_is_single_board_winning_dig() {
        let mut board = Board::new();
        board.cells[1][1][0][2] = Tile::Player1;
        board.cells[1][1][1][1] = Tile::Player1;
        board.cells[1][1][2][0] = Tile::Player1;

        assert!(board.is_single_board_winning(1, 1));
    }

    #[test]
    fn test_is_single_board_winning_col() {
        let mut board = Board::new();
        board.cells[2][0][0][1] = Tile::Player2;
        board.cells[2][0][1][1] = Tile::Player2;
        board.cells[2][0][2][1] = Tile::Player2;
        println!("{}", board);
        assert!(board.is_single_board_winning(2, 0));
    }


    #[test]
    fn test_is_single_board_not_winning_col() {
        let mut board = Board::new();
        board.cells[2][0][0][1] = Tile::Player2;
        board.cells[2][0][1][1] = Tile::Player2;
        board.cells[2][0][2][1] = Tile::Player2;
        assert!(!board.is_single_board_winning(1, 1));
    }

    #[test]
    fn test_is_single_board_not_winning_col2() {
        let mut board = Board::new();
        board.cells[2][0][0][1] = Tile::Player2;
        board.cells[2][0][1][1] = Tile::Player2;
        board.cells[2][0][2][1] = Tile::Player1;
        assert!(!board.is_single_board_winning(2, 0));
    }


    #[test]
    fn test_is_board_winning() {
        let mut board = Board::new();
        board.cells[0][0][0][0] = Tile::Player1;
        board.cells[0][0][0][1] = Tile::Player1;
        board.cells[0][0][0][2] = Tile::Player1;
        assert!(board.is_single_board_winning(0, 0));

        board.cells[1][1][0][2] = Tile::Player1;
        board.cells[1][1][1][1] = Tile::Player1;
        board.cells[1][1][2][0] = Tile::Player1;
        assert!(board.is_single_board_winning(1, 1));

        board.cells[2][2][0][1] = Tile::Player1;
        board.cells[2][2][1][1] = Tile::Player1;
        board.cells[2][2][2][1] = Tile::Player1;
        assert!(board.is_single_board_winning(2, 2));

        println!("{}", board);
        assert!(board.is_board_winning());
    }
}