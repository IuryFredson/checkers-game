// A simple implementation of a checkers game in Rust
use std::io::{self, Write};

#[derive(Clone, Copy, PartialEq)]
enum Piece {
    Empty,
    White,
    Black,
    WhiteKing,
    BlackKing,
}

struct Game {
    board: [[Piece; 8]; 8],
    current_player: Piece,
    selected_piece: Option<(usize, usize)>,
}

impl Game {
    // Initializes a new game with pieces set up in starting positions
    fn new() -> Game {
        let mut board = [[Piece::Empty; 8]; 8];
        
        // Place white pieces in initial positions
        for row in 0..3 {
            for col in 0..8 {
                if (row + col) % 2 == 1 {
                    board[row][col] = Piece::White;
                }
            }
        }
        
        // Place black pieces in initial positions
        for row in 5..8 {
            for col in 0..8 {
                if (row + col) % 2 == 1 {
                    board[row][col] = Piece::Black;
                }
            }
        }

        Game {
            board,
            current_player: Piece::White,
            selected_piece: None,
        }
    }

    // Prints the game board with row and column indices
    fn print_board(&self) {
        println!("   0 1 2 3 4 5 6 7");
        println!("  -----------------");
        for (i, row) in self.board.iter().enumerate() {
            print!("{} |", i);
            for piece in row {
                let symbol = match piece {
                    Piece::Empty => ".",
                    Piece::White => "w",
                    Piece::Black => "b",
                    Piece::WhiteKing => "W",
                    Piece::BlackKing => "B",
                };
                print!("{}|", symbol);
            }
            println!();
            println!("  -----------------");
        }
    }

    // Checks if a move is valid according to game rules
    fn is_valid_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        if !self.is_within_bounds(to) {
            return false;
        }

        let (from_row, from_col) = from;
        let (to_row, to_col) = to;
        let piece = self.board[from_row][from_col];

        // Check if destination is empty
        if self.board[to_row][to_col] != Piece::Empty {
            return false;
        }

        // Calculate move direction and distance
        let row_diff = (to_row as i32 - from_row as i32).abs();
        let col_diff = (to_col as i32 - from_col as i32).abs();

        // Regular move
        if row_diff == 1 && col_diff == 1 {
            match piece {
                Piece::White => to_row > from_row,
                Piece::Black => to_row < from_row,
                Piece::WhiteKing | Piece::BlackKing => true,
                Piece::Empty => false,
            }
        }
        // Capture move
        else if row_diff == 2 && col_diff == 2 {
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            let captured = self.board[mid_row][mid_col];

            match piece {
                Piece::White | Piece::WhiteKing => {
                    captured == Piece::Black || captured == Piece::BlackKing
                }
                Piece::Black | Piece::BlackKing => {
                    captured == Piece::White || captured == Piece::WhiteKing
                }
                Piece::Empty => false,
            }
        } else {
            false
        }
    }

    // Executes a move if valid, handles captures and king promotions
    fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        if !self.is_valid_move(from, to) {
            return false;
        }

        let (from_row, from_col) = from;
        let (to_row, to_col) = to;
        let piece = self.board[from_row][from_col];

        // Move the piece
        self.board[to_row][to_col] = piece;
        self.board[from_row][from_col] = Piece::Empty;

        // Handle capture
        if (to_row as i32 - from_row as i32).abs() == 2 {
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            self.board[mid_row][mid_col] = Piece::Empty;
        }

        // King promotion
        if (piece == Piece::White && to_row == 7) || (piece == Piece::Black && to_row == 0) {
            self.board[to_row][to_col] = if piece == Piece::White {
                Piece::WhiteKing
            } else {
                Piece::BlackKing
            };
        }

        // Switch player
        self.current_player = if self.current_player == Piece::White {
            Piece::Black
        } else {
            Piece::White
        };

        true
    }

    // Checks if a position is within the board boundaries
    fn is_within_bounds(&self, pos: (usize, usize)) -> bool {
        let (row, col) = pos;
        row < 8 && col < 8
    }

    // Parses user input into move coordinates
    fn parse_move(input: &str) -> Option<((usize, usize), (usize, usize))> {
        let coords: Vec<&str> = input.split_whitespace().collect();
        if coords.len() != 4 {
            return None;
        }

        let from_row = coords[0].parse().ok()?;
        let from_col = coords[1].parse().ok()?;
        let to_row = coords[2].parse().ok()?;
        let to_col = coords[3].parse().ok()?;

        Some(((from_row, from_col), (to_row, to_col)))
    }
}

fn main() {
    let mut game = Game::new();
    
    loop {
        game.print_board();
        println!("Current player: {}", if game.current_player == Piece::White { "White" } else { "Black" });
        println!("Enter move (from_row from_col to_row to_col) or 'q' to quit:");
        
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        if input == "q" {
            break;
        }

        if let Some((from, to)) = Game::parse_move(input) {
            if game.make_move(from, to) {
                println!("Move successful!");
            } else {
                println!("Invalid move! Try again.");
            }
        } else {
            println!("Invalid input! Use format: row col row col");
        }
    }
}
