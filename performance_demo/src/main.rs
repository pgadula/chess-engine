use chess_uci::{engine::Engine, lexer::Lexer};
fn main() {
    let mut engine = Engine::new();
    engine.new_game();
    engine.go();
}
