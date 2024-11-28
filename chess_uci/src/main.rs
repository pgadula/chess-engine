mod engine;
mod lexer;

use std::io;

use engine::Engine;
use lexer::Lexer;

fn main() {
    let mut engine = Engine::new();

    while engine.is_running  {
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
                        println!("next token {}", next_token);
                        match next_token.as_str() {
                            "startpos" => {
                                if let Some(move_uci) = lexer.next(){
                                    let mut uci_moves = Vec::new();
                                    uci_moves.push(move_uci);
                                    while let Some(another_uci) = lexer.next() {
                                        uci_moves.push(another_uci);
                                    }
                                    engine.apply_move(uci_moves)
                                }else{
                                    engine.new_game();
                                }

                            },
                            "fen" => {
                                let mut positon = Vec::new();
                                while let Some(position_token) = lexer.next()  {
                                    positon.push(position_token);
                                }
                                let fen = positon.concat();
                                println!("fen, {}", fen);
                                engine.from(&fen);
                                engine.print();
                            },
                            _=>{
                                println!("
                                Invalid usage of positon \n 
                                    position startpos: Sets the starting position.
                                    position fen [FEN]: Sets the position using a FEN string.
                                    position startpos moves [moves]: Sets the position from the starting position and the list of moves.
                                ")
                            }
                        }
                    }else{
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
                }
                "ucinewgame" => {
                    engine.new_game();
                }
                "stop" => {
                    engine.stop();
                }
                "quit" => {
                    engine.quit();
                }
                _ => {
                    eprintln!("Unknown command: {:?}", token);
                }
            }
        }

    }
}

