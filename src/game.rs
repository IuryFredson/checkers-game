// src/game.rs
// Manages game flow and player interactions
use crate::board::Board;
use crate::ui::GameUI;
use crate::types::{Move, Player};
use std::io;

// Represents the entire game state and interface
pub struct Game {
    // Game board with all game logic
    pub board: Board,
    // User interface for displaying game state
    pub ui: GameUI,
}

impl Game {
    // Creates a new game with a fresh board and UI
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            ui: GameUI::new(),
        }
    }

    // Main game loop for local play
    pub fn run_local(&mut self) -> io::Result<()> {
        loop {
            // Display current board state
            self.ui.print_board(&self.board.get_board());
            
            // Show current player
            let current_player = self.board.get_current_player();
            println!("Jogador atual: {}", match current_player {
                Player::White => "Branco (○)",
                Player::Black => "Preto (●)",
            });
            
            // Check and display mandatory captures
            let captures = self.board.get_mandatory_captures();
            if !captures.is_empty() {
                println!("Há capturas obrigatórias disponíveis!");
                for (i, mov) in captures.iter().enumerate() {
                    println!("{}. De ({},{}) para ({},{})",
                        i + 1, mov.from_x, mov.from_y, mov.to_x, mov.to_y);
                }
            }
            
            // Get player move input
            if let Some((from_x, from_y, to_x, to_y)) = self.ui.get_move_input() {
                let mov = Move {
                    from_x,
                    from_y,
                    to_x,
                    to_y,
                    captures: vec![],
                };
                
                // Attempt to make the move
                if self.board.make_move(&mov) {
                    // Check for game end condition
                    if self.board.is_game_over() {
                        println!("Fim de jogo! {} venceu!", match self.board.get_current_player() {
                            Player::White => "Preto",
                            Player::Black => "Branco",
                        });
                        break;
                    }
                } else {
                    // Invalid move handling
                    println!("Movimento inválido! Tente novamente.");
                }
            } else {
                // Invalid input handling
                println!("Entrada inválida! Use o formato: linha_origem,coluna_origem,linha_destino,coluna_destino");
            }
        }
        
        Ok(())
    }
}