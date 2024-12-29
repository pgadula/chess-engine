use std::{io, time::Instant};

use chess_uci::{engine::Engine, lexer::Lexer};
fn main() {
    let mut engine = Engine::new();

    while engine.is_running {
        let mut buf = String::from("");
        let _ = io::stdin().read_line(&mut buf);
        let chars = &buf.chars().collect::<Vec<char>>();
        let mut lexer = Lexer::from(chars);
        while let Some(token) = lexer.next() {
            match token.as_str() {
                "uci" => {
                    println!("id name PrzemChess Chess");
                    println!("id author Przemyslaw Gadula");
                    println!("uciok");
                }
                "isready" => {
                    println!("readyok");
                }
                "position" => {
                    if let Some(next_token) = lexer.next() {
                        match next_token.as_str() {
                            "startpos" => {
                                    let mut tokens = Vec::new(); // Buffer to collect tokens

                                    while let Some(token) = lexer.next() {
                                        if token == "moves" {
                                            continue;
                                        }
                                        tokens.push(token);
                                    }

                                    let uci_moves: Vec<String> = tokens
                                    .join(" ")
                                    .split_whitespace() 
                                    .map(|s| s.to_string()) 
                                    .collect(); 

                                    if uci_moves.len() > 0 {
                                        engine.apply_move(uci_moves);
                                    } else {
                                        engine.new_game();
                                    }
                            }
                            "fen" => {
                                let mut positon = Vec::new();
                                while let Some(position_token) = lexer.next() {
                                    positon.push(position_token);
                                }
                                let fen = positon.concat();
                                println!("fen, {}", fen);
                                engine.from(&fen);
                            }
                            _ => {
                                println!("
                                Invalid usage of position command:
                                  position startpos       : Sets the starting position.
                                  position fen [FEN]      : Sets the position using a FEN string.
                                  position startpos moves [moves] : Sets the position from the starting position and the list of moves.
                                ");
                            }
                        }
                    } else {
                        println!("
                        Invalid usage of positon \n 
                            position startpos: Sets the starting position.
                            position fen [FEN]: Sets the position using a FEN string.
                            position startpos moves [moves]: Sets the position from the starting position and the list of moves.
                        ")
                    }
                }
                "go" => {
                    println!("UciGo");
                    let now = Instant::now();
                    engine.go(Some(24));
                    let elapsed = now.elapsed();
                    println!("Thinking time: {:.2?}", elapsed);
                }
                "ucinewgame" => {
                    engine.new_game();
                }
                "stop" => {
                    engine.stop_thinking();
                }
                "quit" => {
                    engine.quit();
                }
                "d" =>{
                    engine.print();
                }
                _ => {
                    eprintln!("Unknown command: {:?}", token);
                }
            }
        }
    }
}
