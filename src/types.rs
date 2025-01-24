// src/types.rs
// Shared type definitions for the checkers game

// Represents a game piece with possible states
#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    Empty,           // No piece on this square
    Regular(Player), // Standard game piece for a specific player
    King(Player),    // Promoted piece with extended movement capabilities
}

// Represents the two players in the game
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    White,
    Black,
}

// Represents a move in the game, including origin, destination, and captures
#[derive(Debug, Clone)]
pub struct Move {
    pub from_x: usize,      // Starting row
    pub from_y: usize,      // Starting column
    pub to_x: usize,        // Destination row
    pub to_y: usize,        // Destination column
    pub captures: Vec<(usize, usize)>, // Positions of captured pieces
}