mod sandbox_main;

use std::{io, mem::transmute};

use chess_core::{
    types::{Color, FileRank, MoveType, Piece, PieceType, FILE_RANK},
    utility::print_as_board,
};
use chess_uci::engine::Engine;
use sandbox_main::debug_move;


fn main() {
    let mut engine = Engine::new();
    while engine.is_running {
        let mut buf = String::from("");
        let _ = io::stdin().read_line(&mut buf);
        let chars = &buf.chars().collect::<Vec<char>>();
        engine.process_command(chars);
    }



    debug_move();
}
