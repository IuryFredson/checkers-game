// src/main.rs
// Main entry point for the checkers game application

// Module declarations for project structure
mod types;    // Type definitions
mod board;    // Game board logic
mod ui;       // User interface
mod game;     // Game management

// Import game management module
use game::Game;
use std::io;

// Application entry point
fn main() -> io::Result<()> {
    // Create a new game instance
    let mut game = Game::new();
    
    // Run the game loop
    game.run_local()?;
    
    Ok(())
}