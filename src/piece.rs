// Define the piece types in the game
#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    Empty,
    White,
    Black,
    WhiteKing,
    BlackKing,
}
