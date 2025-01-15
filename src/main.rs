use std::io::{self, Write};

// Enum to represent different types of pieces
#[derive(Clone, Copy, PartialEq)]
enum Piece {
    Empty,         // Represents an empty square
    White,         // White piece
    Black,         // Black piece
    WhiteKing,     // White King piece
    BlackKing,     // Black King piece
}

// Game structure to hold board state and current player
struct Game {
    board: [[Piece; 8]; 8],  // 8x8 game board
    current_player: Piece,   // Current player (White or Black)
}

impl Game {
    // Initialize a new game with starting pieces
    fn new() -> Game {
        let mut board = [[Piece::Empty; 8]; 8];  // Initialize an empty board

        // Place white pieces in the initial positions (first three rows)
        for row in 0..3 {
            for col in 0..8 {
                if (row + col) % 2 == 1 {  // Place only on dark squares
                    board[row][col] = Piece::White;
                }
            }
        }

        // Place black pieces in the initial positions (last three rows)
        for row in 5..8 {
            for col in 0..8 {
                if (row + col) % 2 == 1 {  // Place only on dark squares
                    board[row][col] = Piece::Black;
                }
            }
        }

        // Return a new game instance with the board and white player to start
        Game {
            board,
            current_player: Piece::White,
        }
    }

    // Function to print the current board state
    fn print_board(&self) {
        println!("   0 1 2 3 4 5 6 7");
        println!("  -----------------");
        for (i, row) in self.board.iter().enumerate() {
            print!("{} |", i);  // Print row number
            for piece in row {
                let symbol = match piece {
                    Piece::Empty => ".",       // Empty square
                    Piece::White => "w",       // White piece
                    Piece::Black => "b",       // Black piece
                    Piece::WhiteKing => "W",   // White King
                    Piece::BlackKing => "B",   // Black King
                };
                print!("{}|", symbol);  // Print piece symbol
            }
            println!();
            println!("  -----------------");
        }
    }

    // Function to check if a move is valid
    fn is_valid_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;
        let piece = self.board[from_row][from_col];

        // Check if the destination is within bounds and empty
        if !self.is_within_bounds(to) || self.board[to_row][to_col] != Piece::Empty {
            return false;
        }

        // Calculate the row and column difference
        let row_diff = (to_row as i32 - from_row as i32).abs();
        let col_diff = (to_col as i32 - from_col as i32).abs();

        // Valid move: Simple move (1 step diagonal) or jump (2 steps with capture)
        if row_diff == 1 && col_diff == 1 {
            match piece {
                Piece::White => to_row > from_row,  // White moves downward
                Piece::Black => to_row < from_row,  // Black moves upward
                Piece::WhiteKing | Piece::BlackKing => true,  // Kings can move both ways
                _ => false,
            }
        } else if row_diff == 2 && col_diff == 2 {
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            let captured = self.board[mid_row][mid_col];

            // Check if the captured piece is the opponent's piece
            match piece {
                Piece::White | Piece::WhiteKing => captured == Piece::Black || captured == Piece::BlackKing,
                Piece::Black | Piece::BlackKing => captured == Piece::White || captured == Piece::WhiteKing,
                _ => false,
            }
        } else {
            false
        }
    }

    // Check if the piece at the position belongs to the current player
    fn is_current_player_piece(&self, pos: (usize, usize)) -> bool {
        let (row, col) = pos;
        match self.board[row][col] {
            Piece::White | Piece::WhiteKing if self.current_player == Piece::White => true,
            Piece::Black | Piece::BlackKing if self.current_player == Piece::Black => true,
            _ => false,
        }
    }

    // Check if the game is over and return the result (winner or draw)
    fn is_game_over(&self) -> Option<String> {
        let mut white_pieces = 0;
        let mut black_pieces = 0;
        let mut has_valid_moves = false;

        // Count remaining pieces and check if the current player has valid moves
        for row in 0..8 {
            for col in 0..8 {
                match self.board[row][col] {
                    Piece::White | Piece::WhiteKing => white_pieces += 1,
                    Piece::Black | Piece::BlackKing => black_pieces += 1,
                    _ => {}
                }

                // Check if current player has valid moves from this piece
                if self.is_current_player_piece((row, col)) {
                    if self.check_valid_moves_for_piece((row, col)) {
                        has_valid_moves = true;
                    }
                }
            }
        }

        // Determine game outcome based on piece count and valid moves
        if white_pieces == 0 {
            Some(String::from("Black wins!"))
        } else if black_pieces == 0 {
            Some(String::from("White wins!"))
        } else if !has_valid_moves {
            Some(format!("{} wins due to no valid moves!", 
                if self.current_player == Piece::White { "Black" } else { "White" }))
        } else {
            None
        }
    }

    // Check if a piece has any valid moves
    fn check_valid_moves_for_piece(&self, (row, col): (usize, usize)) -> bool {
        for to_row in 0..8 {
            for to_col in 0..8 {
                if self.is_valid_move((row, col), (to_row, to_col)) {
                    return true;
                }
            }
        }
        false
    }

    // Function to make a move and update the board
    fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        let (from_row, from_col) = from;

        // Check if the piece belongs to the current player
        if !self.is_current_player_piece(from) {
            println!("You can only move your own pieces!");
            return false;
        }

        // Check if the move is valid
        if !self.is_valid_move(from, to) {
            println!("Invalid move!");
            return false;
        }

        let (to_row, to_col) = to;
        let piece = self.board[from_row][from_col];

        // Move the piece to the new position
        self.board[to_row][to_col] = piece;
        self.board[from_row][from_col] = Piece::Empty;

        // Remove the captured piece if it's a jump move (2 steps)
        if (to_row as i32 - from_row as i32).abs() == 2 {
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            self.board[mid_row][mid_col] = Piece::Empty;
        }

        // Promote to King if a piece reaches the opponent's back row
        if (piece == Piece::White && to_row == 7) || (piece == Piece::Black && to_row == 0) {
            self.board[to_row][to_col] = if piece == Piece::White {
                Piece::WhiteKing
            } else {
                Piece::BlackKing
            };
        }

        // Switch the current player
        self.current_player = if self.current_player == Piece::White {
            Piece::Black
        } else {
            Piece::White
        };

        true
    }

    // Check if a position is within the bounds of the board
    fn is_within_bounds(&self, pos: (usize, usize)) -> bool {
        let (row, col) = pos;
        row < 8 && col < 8
    }

    // Parse user input for a move in the format: "from_row from_col to_row to_col"
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
    let mut game = Game::new();  // Initialize the game

    loop {
        game.print_board();  // Print the current board state
        println!("Current player: {}", if game.current_player == Piece::White { "White" } else { "Black" });
        println!("Enter move (from_row from_col to_row to_col) or 'q' to quit:");

        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "q" {  // Exit the game if the user enters 'q'
            break;
        }

        // Parse the input move and execute it
        if let Some((from, to)) = Game::parse_move(input) {
            if game.make_move(from, to) {
                println!("Move successful!");
            } else {
                println!("Invalid move! Try again.");
            }
        } else {
            println!("Invalid input! Use format: row col row col");
        }

        // Check if the game is over and print the result
        if let Some(result) = game.is_game_over() {
            println!("{}", result);
            break;
        }
    }
}