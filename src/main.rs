mod game; // Import the game logic
mod piece; // Import the piece definition

use game::Game; // Use the Game struct from game.rs
use std::io::{self, Write};

fn main() {
    let mut game = Game::new(); // Initialize the game

    loop {
        game.print_board(); // Print the board
        println!("Current player: {}", if game.current_player == piece::Piece::White { "White" } else { "Black" });
        println!("Enter move (from_row from_col to_row to_col) or 'q' to quit:");

        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "q" {
            break; // Exit the loop if the player enters "q"
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

        // Check if the game is over
        if let Some(result) = game.is_game_over() {
            println!("{}", result);
            break;
        }
    }
}