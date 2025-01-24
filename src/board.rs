// src/board.rs
// Imports types used in the board implementation
use crate::types::{Piece, Player, Move};

// Represents the game board for a checkers game
pub struct Board {
    // 2D array representing the 8x8 game board
    pieces: [[Piece; 8]; 8],
    // Tracks the current player's turn
    current_player: Player,
}

impl Board {
    // Returns a reference to the current board state
    pub fn get_board(&self) -> &[[Piece; 8]; 8] {
        &self.pieces
    }

    // Creates a new board with initial piece positions
    pub fn new() -> Self {
        let mut pieces = [[Piece::Empty; 8]; 8];
        
        // Initialize board with checkers pieces
        // Black pieces start at the top (rows 0-2)
        // White pieces start at the bottom (rows 5-7)
        // Pieces are placed only on dark squares (where row + column is odd)
        for i in 0..8 {
            for j in 0..8 {
                if (i + j) % 2 == 1 {
                    if i < 3 {
                        pieces[i][j] = Piece::Regular(Player::Black);
                    } else if i > 4 {
                        pieces[i][j] = Piece::Regular(Player::White);
                    }
                }
            }
        }

        Board {
            pieces,
            current_player: Player::White, // White starts first
        }
    }

    // Retrieves a piece at a specific board coordinate
    pub fn get_piece(&self, x: usize, y: usize) -> Piece {
        self.pieces[x][y]
    }

    // Returns the current player
    pub fn get_current_player(&self) -> Player {
        self.current_player
    }

    // Executes a move on the board, returns true if successful
    pub fn make_move(&mut self, move_: &Move) -> bool {
        // Validate the move before execution
        if !self.is_valid_move(move_) {
            return false;
        }

        // Remove piece from original position
        let piece = self.pieces[move_.from_x][move_.from_y];
        self.pieces[move_.from_x][move_.from_y] = Piece::Empty;

        // Remove captured pieces
        for (x, y) in &move_.captures {
            self.pieces[*x][*y] = Piece::Empty;
        }

        // Check for piece promotion to king
        let new_piece = match piece {
            Piece::Regular(player) => {
                // Promote to king if it reaches the opposite end of the board
                if (player == Player::White && move_.to_x == 0) ||
                   (player == Player::Black && move_.to_x == 7) {
                    Piece::King(player)
                } else {
                    piece
                }
            }
            _ => piece,
        };

        // Place the piece (or king) in the new position
        self.pieces[move_.to_x][move_.to_y] = new_piece;

        // Switch players if no more mandatory captures are available
        if self.get_mandatory_captures().is_empty() {
            self.current_player = match self.current_player {
                Player::White => Player::Black,
                Player::Black => Player::White,
            };
        }

        true
    }

    // Finds all mandatory capture moves for the current player
    pub fn get_mandatory_captures(&self) -> Vec<Move> {
        let mut captures = Vec::new();
        
        // Iterate through all board positions
        for i in 0..8 {
            for j in 0..8 {
                // Check if the piece belongs to the current player
                if let Some(player) = match self.pieces[i][j] {
                    Piece::Regular(p) | Piece::King(p) => Some(p),
                    _ => None,
                } {
                    if player == self.current_player {
                        // Get possible moves and filter out only capture moves
                        let moves = self.get_possible_moves(i, j);
                        captures.extend(moves.into_iter().filter(|m| !m.captures.is_empty()));
                    }
                }
            }
        }
        
        captures
    }

    // Calculates all possible moves for a piece at a given position
    pub fn get_possible_moves(&self, x: usize, y: usize) -> Vec<Move> {
        let mut moves = Vec::new();
        
        // Determine the piece and its owner
        let piece = self.pieces[x][y];
        let player = match piece {
            Piece::Regular(p) | Piece::King(p) => Some(p),
            Piece::Empty => None,
        };
        
        // Validate piece ownership and current player
        if player.is_none() || player.unwrap() != self.current_player {
            return moves;
        }
    
        // Define movement directions based on piece type
        let directions = match (piece, player.unwrap()) {
            (Piece::Regular(Player::White), _) => vec![(-1, -1), (-1, 1)], // White moves up diagonally
            (Piece::Regular(Player::Black), _) => vec![(1, -1), (1, 1)],   // Black moves down diagonally
            (Piece::King(_), _) => vec![(-1, -1), (-1, 1), (1, -1), (1, 1)], // Kings move in all directions
            _ => vec![],
        };
    
        // First, check for capture moves
        for (dx, dy) in &directions {
            self.check_capture_moves(x, y, *dx, *dy, &mut moves, &mut Vec::new());
        }
    
        // If no captures, check simple moves
        if moves.is_empty() {
            for (dx, dy) in directions {
                // Calculate new position with safe coordinate checking
                let new_x = match (x as i32).checked_add(dx) {
                    Some(nx) if nx >= 0 && nx < 8 => nx as usize,
                    _ => continue,
                };
                
                let new_y = match (y as i32).checked_add(dy) {
                    Some(ny) if ny >= 0 && ny < 8 => ny as usize,
                    _ => continue,
                };
    
                // Allow move if destination is empty
                if self.pieces[new_x][new_y] == Piece::Empty {
                    moves.push(Move {
                        from_x: x,
                        from_y: y,
                        to_x: new_x,
                        to_y: new_y,
                        captures: vec![],
                    });
                }
            }
        }
    
        moves
    }
    
    // Recursively checks and generates multi-jump capture moves
    fn check_capture_moves(&self, x: usize, y: usize, dx: i32, dy: i32, moves: &mut Vec<Move>, captured: &mut Vec<(usize, usize)>) {
        // Calculate target position after a jump
        let new_x = match (x as i32).checked_add(dx * 2) {
            Some(nx) if nx >= 0 && nx < 8 => nx as usize,
            _ => return,
        };
        
        let new_y = match (y as i32).checked_add(dy * 2) {
            Some(ny) if ny >= 0 && ny < 8 => ny as usize,
            _ => return,
        };
        
        // Calculate the position of the piece to be captured
        let capture_x = (x as i32 + dx) as usize;
        let capture_y = (y as i32 + dy) as usize;
        
        // Get current player
        let current_player = match self.pieces[x][y] {
            Piece::Regular(p) | Piece::King(p) => p,
            _ => return,
        };
    
        // Check if capture is possible
        if let Some(captured_player) = match self.pieces[capture_x][capture_y] {
            Piece::Regular(p) | Piece::King(p) => Some(p),
            Piece::Empty => None,
        } {
            // Validate capture: different players, empty destination, not previously captured
            if captured_player != current_player 
                && self.pieces[new_x][new_y] == Piece::Empty 
                && !captured.contains(&(capture_x, capture_y)) {
                
                let mut new_captured = captured.clone();
                new_captured.push((capture_x, capture_y));
                
                // Add capture move
                moves.push(Move {
                    from_x: x,
                    from_y: y,
                    to_x: new_x,
                    to_y: new_y,
                    captures: new_captured.clone(),
                });
    
                // Recursively check for additional captures from new position
                for (next_dx, next_dy) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
                    self.check_capture_moves(new_x, new_y, next_dx, next_dy, moves, &mut new_captured);
                }
            }
        }
    }

    // Checks if the game is over by verifying if any moves are possible
    pub fn is_game_over(&self) -> bool {
        // Iterate through all board positions
        for i in 0..8 {
            for j in 0..8 {
                // Find current player's pieces
                if let Some(player) = match self.pieces[i][j] {
                    Piece::Regular(p) | Piece::King(p) => Some(p),
                    _ => None,
                } {
                    // If any piece has possible moves, game continues
                    if player == self.current_player && !self.get_possible_moves(i, j).is_empty() {
                        return false;
                    }
                }
            }
        }
        true
    }

    // Validates if a move is legal according to current board state
    fn is_valid_move(&self, move_: &Move) -> bool {
        // Check if the move exists in possible moves for the piece
        self.get_possible_moves(move_.from_x, move_.from_y)
            .iter()
            .any(|m| m.to_x == move_.to_x && m.to_y == move_.to_y)
    }
}