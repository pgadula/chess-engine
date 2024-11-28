mod test_cases;

use std::collections::HashMap;
use std::io;
use std::num::ParseIntError;
use std::process::{Command, Stdio};
use std::str::Utf8Error;
use std::{collections::HashSet, io::Write};

use chess_core::utility::print_as_board;
use chess_core::{
    bitboard::{FenParser, GameState},
    types::PieceMove,
};
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn main() {
    let mut chess = GameState::deserialize("r6r/1b2k2q/5b2/8/7B/8/8/R3K2R w KQkq - 4 3");

    
    // chess.make_move(&PieceMove{
    //     from:FileRank::A1,
    //     target: FileRank::A8,
    //     move_type:chess_core::types::MoveType::Capture,
    //     piece: WHITE_ROOK,
    // });
    // chess.println();
    // println!("Hash:{}", chess.hash);

    // chess.unmake_move();
    // chess.println();
    // println!("Hash:{}", chess.hash);



    //r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQkq - 3 2
    //r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQkq - 3 2

}

#[derive(Debug)]
enum Error {
    CommandError(std::io::Error),
    Utf8Error(Utf8Error),
    ParseError(ParseIntError),
    MissingOutput,
}
#[derive(Debug)]

struct StockfishOutput {
    nodes_for_moves: HashMap<String, usize>,
    total_nodes: usize,
}
fn stock_fish_perft(fen: &str, depth: usize) -> Result<StockfishOutput, Error> {
    // Path to your Stockfish executable
    let stockfish_path = "C:\\Users\\przemek\\Downloads\\stockfish-windows-x86-64-sse41-popcnt\\stockfish\\stockfish.exe";

    // Start the Stockfish process with piped stdin and stdout
    let mut process = Command::new(stockfish_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(Error::CommandError)?;

    // Prepare the commands to send to Stockfish
    if let Some(mut stdin) = process.stdin.take() {
        let commands = format!("position fen {}\ngo perft {}\nquit\n", fen, depth);

        stdin
            .write_all(commands.as_bytes())
            .map_err(Error::CommandError)?;
        stdin.flush().map_err(Error::CommandError)?;
    } else {
        return Err(Error::MissingOutput);
    }

    let output = process.wait_with_output().map_err(Error::CommandError)?;

    if !output.status.success() {
        return Err(Error::CommandError(io::Error::new(
            io::ErrorKind::Other,
            "Stockfish process failed",
        )));
    }
    let mut count = 0;
    let mut move_node_map: HashMap<String, usize> = HashMap::new();
    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if let Some((move_str, count_str)) = line.split_once(": ") {
            let count = count_str
                .trim()
                .replace(",", "")
                .parse::<usize>()
                .map_err(Error::ParseError)?;
            move_node_map.insert(move_str.to_string(), count);
        }
        if line.starts_with("Nodes searched:") {
            if let Some(count_str) = line.split(':').nth(1) {
                count = count_str
                    .trim()
                    .replace(",", "")
                    .parse::<usize>()
                    .map_err(Error::ParseError)?;
            }
        }
    }
    return Ok(StockfishOutput {
        nodes_for_moves: move_node_map,
        total_nodes: count,
    });
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
fn make_move(fen: &str, move_uci: &str) -> Option<String> {
    let result = Command::new("python")
        .args(["../../python_position_checker/make_move.py", fen, move_uci])
        .output();

    match result {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output.stdout)
                .to_string()
                .replace("\r\n", "");
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
    stock_fish_output: StockfishOutput,
    fen: String,
    max_depth: usize,
}

impl CalculationObject {
    fn new(fen: &str, depth: usize) -> Self {
        let stock_fish_output = stock_fish_perft(fen, depth).unwrap();
        println!("Stock fish output");
        CalculationObject {
            stock_fish_output,
            fen: fen.clone().to_owned(),
            max_depth: depth,
        }
    }

    fn debug_move_generator(&mut self) {
        let mut game: GameState = GameState::deserialize(&self.fen);
        let (nodes, moves) = game.perft(self.max_depth);

        if (nodes as usize) != self.stock_fish_output.total_nodes {
            println!(
                "{}ERROR {}fen:{} depth:{}{}",
                RED, RESET, self.fen, self.max_depth, RESET
            );
            println!(
                "result: {} expected:{} {}",
                nodes, self.stock_fish_output.total_nodes, RESET
            );
            println!();
        } else {
            println!(
                "{}SUCCESS{}: depth:{} fen:{} nodes: {}",
                GREEN, RESET, self.max_depth, self.fen, nodes
            );
            println!()
        }
    }

    pub fn get_total_nodes(&mut self, game: &mut GameState, depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }

        game.calculate_pseudolegal_moves();
        let valid_attacks = game.get_valid_moves();
        let mut nodes = 0;
        let fen = GameState::serialize(&game);
        // log_diff(&fen, &valid_attacks);

        for valid_move in valid_attacks.iter() {
            // Apply the move and calculate the result
            let mut clone_game = game.clone();
            let before = clone_game.serialize();
            clone_game.make_move(valid_move);

            let move_nodes = self.get_total_nodes(&mut clone_game.clone(), depth - 1);
            nodes += move_nodes;

            // let calc_fen = make_move(&before, &move_uci).unwrap();
            // if after != calc_fen {
            //     println!();
            //     clone_game.print();
            //     println!(
            //         "
            //         ####################
            //         before: {before}
            //         move: {move_uci} {:?}
            //         expected: {calc_fen}
            //         received: {after}
            //         ####################
            //     ",
            //         valid_move.move_type
            //     );
            //     panic!("Invalid postion ")
            // }
        }

        nodes
    }
}

fn log_diff(fen: &str, valid_attacks: &Vec<&PieceMove>) {
    let valid_attack_strings = valid_attacks
        .iter()
        .map(|f| f.uci())
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
    if difference_a.len() > 0 || difference_b.len() > 0 {
        println!("redundant: {:?}", difference_b);
        println!("missing: {:?}", difference_a);
        println!("{fen}");
    }
}
