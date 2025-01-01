mod sandbox_main;

use std::io;

use chess_uci::engine::Engine;
fn main() {
    let mut engine = Engine::new();
    while engine.is_running {
        let mut buf = String::from("");
        let _ = io::stdin().read_line(&mut buf);
        let chars = &buf.chars().collect::<Vec<char>>();
        engine.process_command(chars);
    }
}
