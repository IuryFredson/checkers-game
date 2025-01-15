use crate::piece::Piece; // Import the Piece enum from piece.rs
// The main structure of the game, including the board and the current player
pub struct Game {
    pub board: [[Piece; 8]; 8], // 8x8 board
    pub current_player: Piece,  // The current player (white or black)
}

impl Game {
    // Initialize the game with the default piece setup
    pub fn new() -> Game {
        let mut board = [[Piece::Empty; 8]; 8]; // Empty board

        // Place white pieces on the first 3 rows
        for row in 0..3 {
            for col in 0..8 {
                if (row + col) % 2 == 1 { // Only place pieces on dark squares
                    board[row][col] = Piece::White;
                }
            }
        }

        // Place black pieces on the last 3 rows
        for row in 5..8 {
            for col in 0..8 {
                if (row + col) % 2 == 1 { // Only place pieces on dark squares
                    board[row][col] = Piece::Black;
                }
            }
        }

        Game {
            board, // Initial board setup
            current_player: Piece::White, // White starts the game
        }
    }

    // Prints the current board to the console
    pub fn print_board(&self) {
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

    // Checks if a move is valid
    pub fn is_valid_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;
        let piece = self.board[from_row][from_col]; // The piece we're moving

        // Make sure the destination is within bounds and is empty
        if !self.is_within_bounds(to) || self.board[to_row][to_col] != Piece::Empty {
            return false;
        }

        let row_diff = (to_row as i32 - from_row as i32).abs();
        let col_diff = (to_col as i32 - from_col as i32).abs();

        // 1-square move (normal move) or 2-square move (capture)
        if row_diff == 1 && col_diff == 1 {
            match piece {
                Piece::White => to_row > from_row,  // White moves forward
                Piece::Black => to_row < from_row,  // Black moves backward
                Piece::WhiteKing | Piece::BlackKing => true, // Kings can move in any direction
                _ => false,
            }
        } else if row_diff == 2 && col_diff == 2 {
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            let captured = self.board[mid_row][mid_col]; // The captured piece

            match piece {
                Piece::White | Piece::WhiteKing => captured == Piece::Black || captured == Piece::BlackKing,
                Piece::Black | Piece::BlackKing => captured == Piece::White || captured == Piece::WhiteKing,
                _ => false, // Not a valid piece to capture
            }
        } else {
            false // Invalid move
        }
    }

    // Checks if the piece at the given position belongs to the current player
    pub fn is_current_player_piece(&self, pos: (usize, usize)) -> bool {
        let (row, col) = pos;
        match self.board[row][col] {
            Piece::White | Piece::WhiteKing if self.current_player == Piece::White => true,
            Piece::Black | Piece::BlackKing if self.current_player == Piece::Black => true,
            _ => false,
        }
    }

    // Checks if the game is over (winner or draw due to no moves left)
    pub fn is_game_over(&self) -> Option<String> {
        let mut white_pieces = 0;
        let mut black_pieces = 0;
        let mut has_valid_moves = false;

        // Count remaining pieces and check if the current player has any valid moves
        for row in 0..8 {
            for col in 0..8 {
                match self.board[row][col] {
                    Piece::White | Piece::WhiteKing => white_pieces += 1,
                    Piece::Black | Piece::BlackKing => black_pieces += 1,
                    _ => {}
                }

                if self.is_current_player_piece((row, col)) {
                    if self.check_valid_moves_for_piece((row, col)) {
                        has_valid_moves = true;
                    }
                }
            }
        }

        // Check for win or draw conditions
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

    // Checks if a piece at a position has valid moves
    pub fn check_valid_moves_for_piece(&self, (row, col): (usize, usize)) -> bool {
        for to_row in 0..8 {
            for to_col in 0..8 {
                if self.is_valid_move((row, col), (to_row, to_col)) {
                    return true;
                }
            }
        }
        false
    }

    // Makes a move from one position to another
    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        let (from_row, from_col) = from;

        // Make sure the piece belongs to the current player
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

        // Perform the move
        self.board[to_row][to_col] = piece;
        self.board[from_row][from_col] = Piece::Empty;

        // If it's a capture, remove the captured piece
        if (to_row as i32 - from_row as i32).abs() == 2 {
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            self.board[mid_row][mid_col] = Piece::Empty;
        }

        // Promote to king if the piece reaches the opposite side
        if (piece == Piece::White && to_row == 7) || (piece == Piece::Black && to_row == 0) {
            self.board[to_row][to_col] = if piece == Piece::White {
                Piece::WhiteKing
            } else {
                Piece::BlackKing
            };
        }

        // Switch the player
        self.current_player = if self.current_player == Piece::White {
            Piece::Black
        } else {
            Piece::White
        };

        true
    }

    // Checks if the position is within the bounds of the board
    pub fn is_within_bounds(&self, pos: (usize, usize)) -> bool {
        let (row, col) = pos;
        row < 8 && col < 8
    }

    // Converts player input into move coordinates
    pub fn parse_move(input: &str) -> Option<((usize, usize), (usize, usize))> {
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