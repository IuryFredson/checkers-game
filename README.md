# Rust Checkers Game 🎲♟️

A fully functional Checkers (Damas) game implemented in Rust, featuring complete game logic, move validation, and console-based gameplay.

## 🚀 Project Structure

The game is organized into several key modules:
- `types.rs`: Defines core game types like `Piece`, `Player`, and `Move`
- `board.rs`: Implements game board logic and move calculations
- `game.rs`: Manages game flow and player interactions
- `ui.rs`: Handles console rendering and user input
- `main.rs`: Application entry point

## ✨ Features

- **Complete Game Logic**
  - Supports standard Checkers rules
  - Validates all moves and captures
  - Tracks player turns
  - Detects mandatory captures

- **Piece Movement**
  - Regular pieces move diagonally
  - Kings can move in all diagonal directions
  - Automatic piece promotion when reaching opposite board end

- **Capture Mechanics**
  - Mandatory capture detection
  - Multiple jump captures supported
  - Captures handled recursively

- **Game State Management**
  - Tracks current player
  - Detects game-over conditions
  - Displays game board with Unicode piece representations

## 🎮 How to Play

### Prerequisites
- Rust programming language (latest stable version)
- Cargo package manager

### Running the Game
1. Clone the repository
   ```bash
   git clone https://github.com/IuryFredson/checkers-game.git
   cd checkers-game/src
   ```

2. Build and run
   ```bash
   cargo run
   ```

### Making Moves
- Input moves in the format: `from_row,from_col,to_row,to_col`
- Example: `2,1,3,2` moves a piece from row 2, column 1 to row 3, column 2
- The game will validate and execute legal moves

## 🌟 Game Board Representation

The board uses Unicode symbols for clear visualization:
- `·` : Empty square
- `○` : White regular piece
- `●` : Black regular piece
- `□` : White king piece
- `■` : Black king piece

## 🔜 Upcoming Improvements
- Graphical user interface (GUI)
- Online multiplayer support
- Enhanced AI opponent
- Persistent game state saving

## 🤝 Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License
MIT License

# Jogo de Damas em Rust 🎲♟️

Um jogo de Damas totalmente funcional implementado em Rust, com lógica de jogo completa, validação de movimentos e jogabilidade baseada em console.

## 🚀 Estrutura do Projeto

O jogo é organizado em vários módulos principais:
- `types.rs`: Define tipos fundamentais como `Piece`, `Player` e `Move`
- `board.rs`: Implementa lógica do tabuleiro e cálculo de movimentos
- `game.rs`: Gerencia fluxo do jogo e interações dos jogadores
- `ui.rs`: Controla renderização de console e entrada do usuário
- `main.rs`: Ponto de entrada da aplicação

## ✨ Funcionalidades

- **Lógica Completa de Jogo**
  - Suporta regras tradicionais de Damas
  - Valida todos os movimentos e capturas
  - Controla turnos dos jogadores
  - Detecta capturas obrigatórias

- **Movimentação de Peças**
  - Peças regulares movem-se diagonalmente
  - Damas podem se mover em todas as direções diagonais
  - Promoção automática ao atingir o lado oposto do tabuleiro

- **Mecânica de Captura**
  - Detecção de capturas obrigatórias
  - Capturas com múltiplos saltos suportadas
  - Capturas tratadas recursivamente

- **Gerenciamento de Estado do Jogo**
  - Rastreia o jogador atual
  - Detecta condições de fim de jogo
  - Exibe tabuleiro com representações Unicode das peças

## 🎮 Como Jogar

### Pré-requisitos
- Linguagem Rust (versão estável mais recente)
- Gerenciador de pacotes Cargo

### Executando o Jogo
1. Clone o repositório
   ```bash
   git clone https://github.com/IuryFredson/checkers-game.git
   cd checkers-game/src
   ```

2. Compilar e executar
   ```bash
   cargo run
   ```

### Fazendo Movimentos
- Insira movimentos no formato: `linha_origem,coluna_origem,linha_destino,coluna_destino`
- Exemplo: `2,1,3,2` move uma peça da linha 2, coluna 1 para linha 3, coluna 2
- O jogo validará e executará movimentos legais

## 🌟 Representação do Tabuleiro

O tabuleiro usa símbolos Unicode para visualização clara:
- `·` : Quadrado vazio
- `○` : Peça branca regular
- `●` : Peça preta regular
- `□` : Peça branca dama
- `■` : Peça preta dama

## 🔜 Melhorias Futuras
- Interface gráfica (GUI)
- Suporte a multiplayer online
- Oponente de IA aprimorado
- Salvamento de estado do jogo

## 🤝 Contribuições
Contribuições são bem-vindas! Sinta-se à vontade para enviar um Pull Request.

## 📄 Licença
Licença MIT