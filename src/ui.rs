// src/ui.rs
// User interface module for displaying game state and getting input
use crate::types::{Piece, Player};
use std::io::{self, Write};

// Manages game visualization and user interaction
pub struct GameUI;

impl GameUI {
    // Initializes a new GameUI instance
    pub fn new() -> Self {
        GameUI
    }

    // Renders the game board to the console
    // Uses Unicode characters to represent different piece types
    pub fn print_board(&self, board: &[[Piece; 8]; 8]) {
        // Print column numbers
        println!("\n  0 1 2 3 4 5 6 7");
        
        // Iterate through board rows
        for i in 0..8 {
            // Print row number
            print!("{} ", i);
            
            // Render each square
            for j in 0..8 {
                match board[i][j] {
                    Piece::Empty => print!("· "),      // Empty square
                    Piece::Regular(Player::White) => print!("○ "), // White regular piece
                    Piece::Regular(Player::Black) => print!("● "), // Black regular piece
                    Piece::King(Player::White) => print!("□ "),   // White king piece
                    Piece::King(Player::Black) => print!("■ "),   // Black king piece
                }
            }
            println!();
        }
        println!();
    }

    // Handles user input for piece movement
    // Returns coordinates of move or None if input is invalid
    pub fn get_move_input(&self) -> Option<(usize, usize, usize, usize)> {
        // Prompt user for move input
        println!("Digite o movimento (ex: 2,1,3,2):");
        
        // Read input from console
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            // Parse input into coordinate array
            let coords: Vec<usize> = input
                .trim()
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            
            // Validate input (must be 4 coordinates)
            if coords.len() == 4 {
                return Some((coords[0], coords[1], coords[2], coords[3]));
            }
        }
        
        // Return None if input is invalid
        None
    }
}