#[derive(Debug)]
pub enum Piece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}
#[derive(Debug)]

pub enum Color {
    White,
    Black,
}
#[derive(Debug)]

pub struct PieceMove{
    piece:Piece,
    color: Color
}
