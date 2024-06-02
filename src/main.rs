mod types;
mod game;
use game::Game;
use types::{Color, Piece};


fn main() {

    // let positions = vec!
    // [
    //     "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
    //     "8/8/8/4p1K1/2k1P3/8/8/8 b - - 0 1",
    //     "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50",
    //     "4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1",
    // ];

    // positions.into_iter().map(|fen| Game::from_fen(fen))
    // .for_each(|game| {
    //     game.print();
    //     println!();
    // });

        let mut game = Game::empty();
        game.set_piece(&(Piece::Pawn, Color::White), game::FileRank::H8);
        game.print();

}

