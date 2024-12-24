use chess_uci::engine::Engine;
fn main() {
    use std::time::Instant;
    let depth = 8;
    let now = Instant::now();
    let mut engine = Engine::new();
    engine.new_game();
    engine.use_multithreading = true;
    engine.go(Some(8));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}, depth: {depth}", elapsed);
}
