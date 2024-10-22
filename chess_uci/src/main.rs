mod engine;
mod lexer;

use std::{io, str::Chars};

use engine::Engine;
use lexer::Lexer;

fn main() {
    let mut engine = Engine::new();

    while engine.is_running  {
        let mut buf = String::from("");
        let body = io::stdin().read_line(&mut buf);
        let chars = &buf.chars().collect::<Vec<char>>();
        let mut lexer = Lexer::from(chars);
        while let Some(token) = lexer.next() {
            match token.as_str() {
                "uci" => {
                    println!("id name PrzemChess Chess-AI");
                    println!("id author Przemyslaw Gadula");
                    println!("uciok");
                }
                "isready" => {
                    println!("readyok");
                }
                "position" => {
                    let tokens: Vec<&str> = lexer.collect();  // This collects the rest of the tokens
                    uci_position(tokens);
                }
                "go" => {
                    println!("UciGo");
                }
                "ucinewgame" => {
                    engine.new_game();
                }
                "stop" => {
                    engine.stop();
                }
                "quit" => {
                    engine.quit();
                },
                _ => {
                    eprintln!("Unknown command: {}", token);
                }
            }
        }
    }
}

pub fn uci_position(tokens: Vec<String>){
    println!("uci");
}