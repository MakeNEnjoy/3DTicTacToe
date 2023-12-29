use std::fmt;
use itertools::iproduct;

use crate::minmax::{GameState, Heuristic};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
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

impl From<Tile> for Option<Player> {
    fn from(tile: Tile) -> Self {
        return match tile {
            Tile::Player1 => Some(Player::Player1),
            Tile::Player2 => Some(Player::Player2),
            Tile::Empty => None
        };
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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

    pub fn create_board(
        cells: [[[[u8; 3]; 3]; 3]; 3],
        last_move: Option<(usize, usize, usize, usize)>
    ) -> Result<Self, String> {
        let mut tile_cells = [[[[Tile::Empty; 3]; 3]; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        tile_cells[i][j][k][l] = match cells[i][j][k][l] {
                            0 => Tile::Empty,
                            1 => Tile::Player1,
                            2 => Tile::Player2,
                            _ => return Err("Cells parameter".to_string())
                        };
                    }
                    
                }
            }
        }

        let mut board = Board {cells:tile_cells, player_to_move: Player::Player1, last_move };
        board.player_to_move = board.who_turn();
        Ok(board)
    }

    pub fn to_arr(&self) -> ([[[[u8; 3]; 3]; 3]; 3], Option<(usize, usize, usize, usize)>) {
        let mut cells = [[[[0; 3]; 3]; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        cells[i][j][k][l] = match self.cells[i][j][k][l] {
                            Tile::Empty => 0,
                            Tile::Player1 => 1,
                            Tile::Player2 => 2,
                        };
                    }
                }
            }
        }
        
        (cells, self.last_move)
    }

    fn number_almost_wins(&self, x0: usize, x1: usize, player: Player) -> usize {
        let mut count = 0;
        // Check horizontal lines
        for x2 in 0..3 {
            if self.cells[x0][x1][x2][0] == self.cells[x0][x1][x2][1] {
                if self.cells[x0][x1][x2][0] == player.into() && self.cells[x0][x1][x2][2] == Tile::Empty{
                    count += 1;
                }
            }
            if self.cells[x0][x1][x2][1] == self.cells[x0][x1][x2][2] {
                if self.cells[x0][x1][x2][1] == player.into()  && self.cells[x0][x1][x2][0] == Tile::Empty {
                    count += 1;
                }
            }
            if self.cells[x0][x1][x2][0] == self.cells[x0][x1][x2][2] {
                if self.cells[x0][x1][x2][0] == player.into() && self.cells[x0][x1][x2][1] == Tile::Empty {
                    count += 1;
                }
            }
        }
        // Check vertical lines
        for x3 in 0..2 {
            if self.cells[x0][x1][0][x3] == self.cells[x0][x1][1][x3] {
                if self.cells[x0][x1][0][x3] == player.into() && self.cells[x0][x1][2][x3] == Tile::Empty{
                    count += 1;
                }
            }
            if self.cells[x0][x1][1][x3] == self.cells[x0][x1][2][x3] && self.cells[x0][x1][0][x3] == Tile::Empty {
                if self.cells[x0][x1][1][x3] == player.into() {
                    count += 1;
                }
            }
            if self.cells[x0][x1][0][x3] == self.cells[x0][x1][2][x3] {
                if self.cells[x0][x1][0][x3] == player.into() && self.cells[x0][x1][1][x3] == Tile::Empty {
                    count += 1;
                }
            }
        }

        // Check diagonal lines
        if self.cells[x0][x1][0][0] == self.cells[x0][x1][1][1] {
            if self.cells[x0][x1][0][0] == player.into() && self.cells[x0][x1][2][2] == Tile::Empty {
                count += 1;
            }
        }
        if self.cells[x0][x1][1][1] == self.cells[x0][x1][2][2] {
            if self.cells[x0][x1][1][1] == player.into() && self.cells[x0][x1][0][0] == Tile::Empty {
                count += 1;
            }
        }
        if self.cells[x0][x1][0][0] == self.cells[x0][x1][2][2] { // Check that middle is empty
            if self.cells[x0][x1][0][0] == player.into() && self.cells[x0][x1][1][1] == Tile::Empty {
                count += 1;
            }
        }
        if self.cells[x0][x1][1][1] == self.cells[x0][x1][2][0] {
            if self.cells[x0][x1][1][1] == player.into() && self.cells[x0][x1][0][2] == Tile::Empty {
                count += 1;
            }
        }
        if self.cells[x0][x1][0][2] == self.cells[x0][x1][1][1] {
            if self.cells[x0][x1][0][2] == player.into() && self.cells[x0][x1][2][0] == Tile::Empty {
                count += 1;
            }
        }
        if self.cells[x0][x1][2][0] == self.cells[x0][x1][0][2] {
            if self.cells[x0][x1][2][0] == player.into() && self.cells[x0][x1][1][1] == Tile::Empty {
                count += 1;
            }
        }

        count
    }

    fn single_board_winner(&self, x0: usize, x1: usize) -> Option<Player> {
        // Check horizontal lines
        for x2 in 0..3 {
            if self.cells[x0][x1][x2][0] == self.cells[x0][x1][x2][1] && self.cells[x0][x1][x2][1] == self.cells[x0][x1][x2][2] {
                if self.cells[x0][x1][x2][0] != Tile::Empty {
                    return self.cells[x0][x1][x2][0].into();
                }
            }
        }

        // Check vertical lines
        for x3 in 0..3 {
            if self.cells[x0][x1][0][x3] == self.cells[x0][x1][1][x3] && self.cells[x0][x1][1][x3] == self.cells[x0][x1][2][x3] {
                if self.cells[x0][x1][0][x3] != Tile::Empty {
                    return self.cells[x0][x1][0][x3].into();
                }
            }
        }

        // Check diagonal lines
        if self.cells[x0][x1][0][0] == self.cells[x0][x1][1][1] && self.cells[x0][x1][1][1] == self.cells[x0][x1][2][2] {
            if self.cells[x0][x1][0][0] != Tile::Empty {
                return self.cells[x0][x1][0][0].into();
            }
        }
        if self.cells[x0][x1][0][2] == self.cells[x0][x1][1][1] && self.cells[x0][x1][1][1] == self.cells[x0][x1][2][0] {
            if self.cells[x0][x1][0][2] != Tile::Empty {
                return self.cells[x0][x1][0][2].into();
            }
        }

        None
    }

    fn single_board_tie(&self, x0: usize, x1: usize) -> bool {
        self.cells[x0][x1].iter().all(|row| row.iter().all(|&cell| cell != Tile::Empty))
    }

    pub fn board_winner(&self) -> Option<Player> {
        // Check horizontal lines
        let horizontal_win = (0..3).find_map(|x0| {
            let first = self.single_board_winner(x0, 0)?;
            if (1..3).all(|x1| self.single_board_winner(x0, x1) == Some(first)) {
                Some(first)
            } else {
                None
            }
        });
    
        if horizontal_win.is_some() {
            return horizontal_win;
        }
    
        // Check vertical lines
        let vertical_win = (0..3).find_map(|x0| {
            let first = self.single_board_winner(0, x0)?;
            if (1..3).all(|x1| self.single_board_winner(x1, x0) == Some(first)) {
                Some(first)
            } else {
                None
            }
        });
    
        if vertical_win.is_some() {
            return vertical_win;
        }
    
        // Check diagonals
        let diagonal_win = {
            let first = self.single_board_winner(0, 0)?;
            if (1..3).all(|x| self.single_board_winner(x, x) == Some(first)) {
                Some(first)
            } else {
                let first = self.single_board_winner(0, 2)?;
                if (1..3).all(|x| self.single_board_winner(x, 2 - x) == Some(first)) {
                    Some(first)
                } else {
                    None
                }
            }
        };
    
        diagonal_win
    }
    pub fn who_turn(&self) -> Player {
        let mut player1_count = 0;
        let mut player2_count = 0;

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        match self.cells[i][j][k][l] {
                            Tile::Player1 => player1_count += 1,
                            Tile::Player2 => player2_count += 1,
                            Tile::Empty => {}
                        }
                    }
                }
            }
        }

        if player1_count <= player2_count {
            Player::Player1
        } else {
            Player::Player2
        }
    }
    
    pub fn get_legal_boards(&self) -> Vec<Board> {
        let mut states: Vec<Board> = Vec::new();
        for m in TicMove::iter_moves(self).map(|m| m.do_move()) {
            states.push(m);
        }
        states
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..3 {
            for k in 0..3 {
                for j in 0..3 {
                    for l in 0..3 {
                        let tile = self.cells[i][j][k][l];
                        if let Some((_, _, x, y)) = self.last_move {
                            if i == x && j == y {
                                if tile.to_string() == "." {
                                    write!(f, "`")?;
                                } else {
                                    write!(f, "{}", tile.to_string().to_uppercase())?;
                                }
                            } else {
                                write!(f, "{}", tile.to_string().to_lowercase())?;
                            }
                        } else {
                            write!(f, "{}", tile.to_string().to_lowercase())?;
                        }
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

pub struct BoardResult(Result<Board, String>);

impl BoardResult {
    pub fn into_inner(self) -> Result<Board, String> {
        self.0
    }
}

impl From<&str> for BoardResult {
    fn from(s: &str) -> Self {
        let mut cells = [[[[Tile::Empty; 3]; 3]; 3]; 3];
        let mut last_move = None;

        let chars: Vec<char> = s.chars().filter(|&c| !c.is_whitespace()).collect();
        let mut index = 0;
        let mut x_count = 0;
        let mut o_count = 0;

        if chars.len() != 81 {
            return BoardResult(Err("Invalid board string: wrong number of characters".to_string()));
        }

        for i in 0..3 {
            for k in 0..3 {
                for j in 0..3 {
                    for l in 0..3 {
                        let c = {
                            if chars[index].is_uppercase() || chars[index] == '`' {
                                if let Some((_, _, x, y)) = last_move {
                                    if !(i == x && j == y) {
                                        return BoardResult(Err("Last move is unclear".to_string()))
                                    }
                                } else {
                                    last_move = Some((0, 0, i, j));
                                }
                                if chars[index].is_uppercase() {
                                    chars[index].to_ascii_lowercase()
                                } else {
                                    '.'
                                }

                            } else {
                                chars[index]
                            }
                        };
                        match c {
                            'x' => {
                                cells[i][j][k][l] = Tile::Player1;
                                x_count += 1;
                            },
                            'o' => {
                                cells[i][j][k][l] = Tile::Player2;
                                o_count += 1;
                            },
                            '.' => cells[i][j][k][l] = Tile::Empty,
                            _ => return BoardResult(Err("Invalid board string: unknown character".to_string())),
                        }
                        index += 1;
                    }
                }
            }
        }

        if x_count < o_count || x_count > o_count + 1 {
            return BoardResult(Err("Invalid board string: impossible number of x's or o's".to_string()));
        }

        let player_to_move = if x_count == o_count { Player::Player1 } else { Player::Player2 };

        BoardResult(Ok(Board {
            cells,
            player_to_move,
            last_move,
        }))
    }
}

pub fn print_on_board<T>(board: &Board, data: Vec<T>)
where T: Copy + fmt::Display, {
    let max_length = data.iter()
        .map(|item| item.to_string().len())
        .max()
        .unwrap();
    for i in 0..3 {
        for k in 0..3 {
            for j in 0..3 {
                for l in 0..3 {
                    let tile = board.cells[i][j][k][l];
                    let mut move_num = None;
                    for (index, m) in TicMove::iter_moves(board).enumerate() {
                        if m.move_to_make == (i,j,k,l) {
                            move_num = Some(data[index]);
                        }
                    }
                    let length = {
                        if let Some(num) = move_num {
                            print!(" {}", num);
                            num.to_string().len()
                        } else {
                            print!(" {}", tile);
                            1
                        }
                    };
                    for _ in 0..max_length-length {
                        print!(" ");
                    }
                }
                print!(" ");
            }
            print!("\n");
        }
        print!("\n");
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
        if !tic_move.move_in_valid_board() || board.board_winner().is_some() {
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
        if self.board.single_board_winner(x1, x2).is_some() {
            return false;
        }

        if let Some((_,_,y3,y4)) = self.board.last_move {
            (x1 == y3 && x2 == y4) || self.board.single_board_winner(y3, y4).is_some() || self.board.single_board_tie(y3, y4)
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
    fn next_states(&self) -> Vec<Board> {
        self.get_legal_boards()
    }
}

pub struct AlmostWinHeuristic {}

impl Heuristic<Board> for AlmostWinHeuristic {
    type Score = i32;
    fn score(board: &Board) -> Self::Score {
        if board.board_winner() == Some(Player::Player1) {
            return -1000;
        }
        if board.board_winner() == Some(Player::Player2) {
            return 1000;
        }
        if board.get_legal_boards().len() == 0 {
            return 0;
        }

        let mut count = 0;
        for x0 in 0..3 {
            for x1 in 0..3 {
                match board.single_board_winner(x0, x1) {
                    Some(player) => match player {
                        Player::Player1 => {
                            count -= 3;
                        }
                        Player::Player2 => {
                            count += 3;
                        }
                    },
                    None => {
                        if board.number_almost_wins(x0, x1, Player::Player2) > 0 {
                            count += 1;
                        }
                        if board.number_almost_wins(x0, x1, Player::Player1) > 0 {
                            count -= 1;
                        }
                    },
                }
                
            }
        }

        count.try_into().unwrap()
    }
}

#[cfg(test)]
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
    fn test_winning_game() {
        let moves_to_play = vec![40, 3, 4, 0, 4, 3, 0, 0, 6, 0, 5, 4, 5, 2, 0, 3, 7, 2, 6, 4];
        let mut board = Board::new();
        for m in moves_to_play {
            assert!(board.board_winner().is_none());
            let moves = board.next_states();
            board = moves.get(m).unwrap().to_owned();
        }
        println!("Board \n{}", board);
        assert!(board.board_winner() == Some(Player::Player2));
    }

    #[test]
    fn test_is_single_board_winning_row() {
        let mut board = Board::new();
        board.cells[0][0][0][0] = Tile::Player1;
        board.cells[0][0][0][1] = Tile::Player1;
        board.cells[0][0][0][2] = Tile::Player1;
        board.player_to_move = Player::Player2;

        assert_eq!(board.single_board_winner(0, 0), Some(Player::Player1));
    }

    #[test]
    fn test_is_single_board_winning_dig() {
        let mut board = Board::new();
        board.cells[1][1][0][2] = Tile::Player1;
        board.cells[1][1][1][1] = Tile::Player1;
        board.cells[1][1][2][0] = Tile::Player1;
        board.player_to_move = Player::Player2;

        assert_eq!(board.single_board_winner(1, 1), Some(Player::Player1));
    }

    #[test]
    fn test_is_single_board_winning_col() {
        let mut board = Board::new();
        board.cells[2][0][0][1] = Tile::Player2;
        board.cells[2][0][1][1] = Tile::Player2;
        board.cells[2][0][2][1] = Tile::Player2;
        println!("{}", board);
        assert_eq!(board.single_board_winner(2, 0), Some(Player::Player2));
    }


    #[test]
    fn test_is_single_board_not_winning_col() {
        let mut board = Board::new();
        board.cells[2][0][0][1] = Tile::Player2;
        board.cells[2][0][1][1] = Tile::Player2;
        board.cells[2][0][2][1] = Tile::Player2;
        assert_eq!(board.single_board_winner(1, 1), None);
    }

    #[test]
    fn test_is_single_board_not_winning_col2() {
        let mut board = Board::new();
        board.cells[2][0][0][1] = Tile::Player2;
        board.cells[2][0][1][1] = Tile::Player2;
        board.cells[2][0][2][1] = Tile::Player1;
        board.player_to_move = Player::Player2;
        assert_eq!(board.single_board_winner(2, 0), None);
    }


    #[test]
    fn test_is_board_winning() {

        let mut board = Board::new();
        board.player_to_move = Player::Player2;
        board.cells[0][0][0][0] = Tile::Player1;
        board.cells[0][0][0][1] = Tile::Player1;
        board.cells[0][0][0][2] = Tile::Player1;
        assert_eq!(board.single_board_winner(0, 0), Some(Player::Player1));

        board.cells[1][1][0][2] = Tile::Player1;
        board.cells[1][1][1][1] = Tile::Player1;
        board.cells[1][1][2][0] = Tile::Player1;
        assert_eq!(board.single_board_winner(1, 1), Some(Player::Player1));

        board.cells[2][2][0][1] = Tile::Player1;
        board.cells[2][2][1][1] = Tile::Player1;
        board.cells[2][2][2][1] = Tile::Player1;
        assert_eq!(board.single_board_winner(2, 2), Some(Player::Player1));

        println!("{}", board);
        assert!(board.board_winner() == Some(Player::Player1));
        assert_eq!(AlmostWinHeuristic::score(&board), -1000);
    }

    #[test]
    fn test_2_in_a_col() {
        let mut board = Board::new();
        board.cells[2][0][0][1] = Tile::Player2;
        board.cells[2][0][1][1] = Tile::Player2;
        board.player_to_move = Player::Player1;
        println!("{}", board);
        assert_eq!(board.number_almost_wins(2, 0, Player::Player2), 1);
    }

    #[test]
    fn test_2_in_a_col_and_dig() {
        let mut board = Board::new();
        board.player_to_move = Player::Player1;
        board.cells[2][0][0][1] = Tile::Player2;
        board.cells[2][0][1][1] = Tile::Player2;

        board.cells[1][1][0][2] = Tile::Player2;
        board.cells[1][1][1][1] = Tile::Player2;

        println!("{}", board);

        assert_eq!(board.number_almost_wins(2, 0, Player::Player2) + board.number_almost_wins(1, 1, Player::Player2), 2);
        assert_eq!(AlmostWinHeuristic::score(&board), 2);
    }

    #[test]
    fn test_score() {
        let mut board = Board::new();
        board.cells[2][0][0][1] = Tile::Player2;
        board.cells[2][0][1][1] = Tile::Player2;
        board.cells[2][0][2][1] = Tile::Player2;
        board.cells[1][0][0][1] = Tile::Player2;
        board.cells[1][0][1][1] = Tile::Player2;
        board.cells[2][1][0][1] = Tile::Player1;
        board.cells[2][1][1][1] = Tile::Player1;
        board.cells[2][1][2][1] = Tile::Player1;
        board.cells[0][0][0][0] = Tile::Player1;
        board.cells[0][0][1][1] = Tile::Player1;
        board.cells[0][2][2][2] = Tile::Player1;
        board.cells[0][2][2][0] = Tile::Player1;
        board.cells[1][2][2][2] = Tile::Player1;
        board.cells[1][2][0][0] = Tile::Player1;
        board.cells[1][1][0][1] = Tile::Player1;
        board.cells[1][1][2][1] = Tile::Player1;
        println!("{}", board);
        assert_eq!(AlmostWinHeuristic::score(&board), -3)
    }

    #[test]
    fn test_score_symmetry() {
        let empty_board = Board::new();
        let moves = empty_board.next_states();
        let board1 = moves.first().unwrap().to_owned();
        let board2 = moves.last().unwrap().to_owned();
        assert_eq!(AlmostWinHeuristic::score(&board1), AlmostWinHeuristic::score(&board2));
    }

    #[test]
    fn test_almost_wins_dummy() {
        let empty_board = Board::new();
        let moves = empty_board.next_states();
        let dummy_board = moves.first().unwrap().to_owned();
        assert_eq!(dummy_board.number_almost_wins(0, 0, Player::Player1), 0);
        assert_eq!(dummy_board.number_almost_wins(0, 0, Player::Player2), 0);
    }

    #[test]
    fn test_score_simple() {
        let board = Board::new();
        let moves = board.next_states();
        let board = moves.get(0).unwrap().to_owned();
        let moves = board.next_states();
        let board = moves.get(3).unwrap().to_owned();
        let moves = board.next_states();
        let board = moves.get(0).unwrap().to_owned();
        let moves = board.next_states();
        let board = moves.get(0).unwrap().to_owned();
        println!("{}", &board);
        assert!(AlmostWinHeuristic::score(&board) > 0);
    }

}