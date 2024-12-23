use chess_uci::{engine::Engine};
fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let mut engine = Engine::new();
    engine.new_game();
    engine.go(Some(10));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
