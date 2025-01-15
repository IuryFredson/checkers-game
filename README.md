# Checkers Game in Rust / Jogo de Damas em Rust

This is a simple implementation of the classic Checkers game written in Rust.  
Este é um jogo simples de damas escrito em Rust.

---

## How to Use / Como Usar

1. Clone this repository / Clone este repositório:  
   `git clone https://github.com/IuryFredson/checkers-game.git`  
   `cd checkers`

2. Build and run the code using Cargo / Compile e execute o código com Cargo:  
   `cargo run`

3. Follow the prompts to input moves in the format / Siga as instruções para inserir movimentos no formato:  
   `from_row from_col to_row to_col`  
   For example / Exemplo: `2 1 3 2`.

4. Enter `q` to quit the game / Digite `q` para sair do jogo.

---

## Features / Funcionalidades

- **Player Turns / Turnos de Jogadores:** Alternates between White and Black players / Alterna entre os jogadores Branco e Preto.
- **Validations / Validações:** Ensures that moves adhere to Checkers rules / Garante que os movimentos sigam as regras do jogo de damas.
- **Captures / Capturas:** Removes opponent pieces after valid capture moves / Remove peças do oponente após capturas válidas.
- **King Promotion / Promoção a Dama:** Promotes a piece to a king upon reaching the opposite end of the board / Promove uma peça ao atingir o lado oposto do tabuleiro.

---

## Future Enhancements / Melhorias Futuras

- Add a graphical interface / Adicionar interface gráfica.
- Implement online multiplayer functionality / Implementar funcionalidade de multiplayer online.
