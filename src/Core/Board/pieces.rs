pub struct ChessPiece {
    piece_type: PieceType,
    color: Color,
}

pub enum PieceType {
    None = 0,
    King = 1,
    Pawn = 2,
    Knight = 3,
    Bishop = 4,
    Rook = 5,
    Queen = 6,
}

pub enum Color {
    White = 8,
    Black = 16,
}