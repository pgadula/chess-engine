mod test_cases;

use std::collections::HashSet;
use std::num::ParseIntError;
use std::process::Command;
use std::str::Utf8Error;
use std::{collections::HashMap, usize};

use chess_core::utility::print_as_board;
use chess_core::{
    bitboard::{BitBoard, FenParser},
    types::{FileRank, PieceMove, BLACK_PAWN, WHITE_PAWN},
};
use test_cases::{TestPosition, TEST_CASE, TEST_CASES};
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn main() {
    let fen = "rnb1kbnr/pp1pq1pp/4p3/5p2/2PpP1P1/2NQB3/PP3P1P/R3KBNR w KQkq - 1 8";

    let mut game = BitBoard::deserialize(fen);
    let test_case = TestPosition {
        depth: 1,
        fen: "rnb1qk1r/pp1Pbppp/2p5/8/1PB5/8/P1P1NnPP/RNBQK2R w KQ - 1 9",
        nodes: 53,
    };
    for case in TEST_CASES {
        let mut calc = CalculationObject::new();
        calc.debug_move_generator(case)
    }

    // let mut calc = CalculationObject::new();
    // calc.debug_move_generator(&TEST_CASE)
}
#[derive(Debug)]
enum Error {
    CommandError(std::io::Error),
    Utf8Error(Utf8Error),
    ParseError(ParseIntError),
}
fn python_move_calculator(fen: &str, depth:&str) -> Result<usize, Error> {

    let result = Command::new("python")
        .args([
            "../../python_position_checker/position_calculator.py",
            fen,
            depth,
        ])
        .output();

    match result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).replace("\r\n", "");
            let count: Result<usize, Error> = stdout.parse().map_err(|op| Error::ParseError(op));
            if count.is_err() {
                eprintln!("{:?}", stdout)
            }
            return count;
            // Parse the string to an unsigned integer
            match stdout.trim().parse::<usize>() {
                Ok(value) => Ok(value), // Return the parsed value
                Err(err) => {
                    eprintln!("Failed to parse output to usize.");
                    Err(Error::ParseError(err))
                }
            }
        }
        Err(err) => {
            eprintln!("Error during calling python script: {:?}", err);
            Err(Error::CommandError(err))
        }
    }
}

fn python_get_moves(fen: &str) -> Option<Vec<String>> {
    let depth = "1";
    let result = Command::new("python")
        .args([
            "../../python_position_checker/move_generator.py",
            fen,
            depth,
        ])
        .output();

    match result {
        Ok(output) => {
            let result: Vec<String> = String::from_utf8_lossy(&output.stdout)
                .split('\n')
                .map(|s| s.to_string().replace('\r', ""))
                .filter(|s| s != "")
                .collect();
            return Some(result);
            // Parse the string to an unsigned integer
        }
        Err(err) => {
            eprintln!("Error during calling python script: {:?}", err);
            None // Default value in case of error
        }
    }
}
struct CalculationObject {
    unique_position: HashMap<String, usize>,
}

impl CalculationObject {
    fn new() -> Self {
        CalculationObject {
            unique_position: HashMap::new(),
        }
    }

    fn debug_move_generator(&mut self, test_position: &test_cases::TestPosition) {
        let mut game: BitBoard = BitBoard::deserialize(&test_position.fen);
        let nodes = self.get_total_nodes(&mut game, test_position.depth);

        println!("
         for fen:{}
         with depth {}
         internal_calculation: {}
         expected: {}
         ",test_position.fen, test_position.depth, nodes, test_position.nodes);
        if nodes != test_position.nodes {
            println!(
                "{}ERROR {}fen:{} depth:{}{}",
                RED, RESET, test_position.fen, test_position.depth, RESET
            );
            println!(
                "result: {} expected:{} {}",
                nodes, test_position.nodes, RESET
            );
            println!();
        } else {
            println!(
                "{}SUCCESS{}: depth:{} fen:{}",
                GREEN, RESET, test_position.depth, test_position.fen,
            );
            println!()
        }
    }

    pub fn get_total_nodes(&mut self, game: &mut BitBoard, depth: u8) -> usize {
        if depth == 0 {
            return 1;
        }
        game.calculate_pseudolegal_moves();
        let valid_attacks = game.get_valid_moves();
        let mut nodes  = 0;

        for attack in valid_attacks.iter() {
            // Apply the move and calculate the result
            let mut clone_game = game.clone();
            clone_game.apply(attack);
            let clone_fen = clone_game.serialize();
            clone_game.calculate_pseudolegal_moves();

            nodes+= self.get_total_nodes(&mut clone_game.clone(), depth - 1);
        }

        nodes
    }
}

fn log_diff(fen: &str, valid_attacks: Vec<&PieceMove>) {
    let valid_attack_strings = valid_attacks
        .iter()
        .map(|f| f.get_simple_notation())
        .collect::<Vec<String>>();

    let all_possibilities = python_get_moves(&fen).unwrap_or(Vec::new());
    let all_possibilities_set: HashSet<String> = all_possibilities.iter().cloned().collect();

    let valid_attack_set: HashSet<String> = valid_attack_strings.iter().cloned().collect();
    let difference_a: HashSet<String> = all_possibilities_set
        .difference(&valid_attack_set)
        .cloned()
        .collect();
    let difference_b: HashSet<String> = valid_attack_set
        .difference(&all_possibilities_set)
        .cloned()
        .collect();
    if (difference_a.len() > 0 || difference_b.len() > 0) {
        println!("redundant: {:?}", difference_b);
        println!("missing: {:?}", difference_a);
        println!("{fen}");
    } 
}
