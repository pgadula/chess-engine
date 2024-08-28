mod test_cases;

use std::{collections::HashMap, usize};

use chess_core::{
    bitboard::{BitBoard, FenParser},
    types::{FileRank, PieceMove, BLACK_PAWN, WHITE_PAWN},
};
use test_cases::{TestPosition, TEST_POSITIONS2};

fn main() {
    // let fen = "rnb1kbnr/pp1pq1pp/4p3/5p2/2PpP1P1/2NQB3/PP3P1P/R3KBNR w KQkq - 1 8";
    // let mut game = BitBoard::deserialize(fen);
    let test_case = TestPosition{
        depth:1,
        fen:"rnb1qk1r/pp1Pbppp/2p5/8/1PB5/8/P1P1NnPP/RNBQK2R w KQ - 1 9",
        nodes: 53
    };
    // for case in TEST_POSITIONS2 {
    //     let mut calc = CalculationObject::new();
    //     calc.debug_move_generator(&case)
    // }


    // println!("{:?}",game.en_passant);
    // print!("{}", game.id());

    let mut calc = CalculationObject::new();
    calc.debug_move_generator(&test_case)


}
struct CalculationObject {
    unique_position: HashMap<usize, usize>,
}

impl CalculationObject {
    fn new() -> Self {
        CalculationObject {
            unique_position: HashMap::new(),
        }
    }

    fn debug_move_generator(&mut self, test_position: &test_cases::TestPosition) {
        let mut game: BitBoard = BitBoard::deserialize(&test_position.fen);



        let total = self.get_total_nodes(&mut game, 2);
        if total != test_position.nodes {
            println!("fen:{} depth:{}", test_position.fen, test_position.depth);

            println!("result: {} expected:{}", total, test_position.nodes);
            println!();
        }
    }

    pub fn get_total_nodes(&mut self, game: &mut BitBoard, depth: u8) -> usize {
        if depth == 0 {
            return 0;
        }

        // if let Some(nodes) = self.unique_position.get(&game.id()) {
        //     // println!("hit cache {}", game.id());
        //     return *nodes;
        // }
        game.calculate_pseudolegal_moves();
        let valid_attacks = game.get_valid_moves();






        // let mut result_from_inner = 0;
        // result_from_inner = valid_attacks
        //     .iter()
        //     .map(|v| {
        //         let mut clone = game.clone();
        //         clone.apply(v);

        //         self.get_total_nodes(&mut clone, depth - 1)
        //     })
        //     .sum();



        // let result = valid_attacks.len() + result_from_inner;
        // self.unique_position.insert(game.id(), result);
        // result
        valid_attacks.len()
    }
}
