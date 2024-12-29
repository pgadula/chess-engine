use chess_uci::{engine::Engine, transposition_table::CollisionStrategy};
use std::time::{Duration, Instant};

fn main() {
    let depth = 10;
    let fen = "rnQq1k1r/pp2bppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R b KQ - 0 8";
    let num_iter = 10;

    let e1 = setup_engine(&CollisionStrategy::ReplaceWithRandHash);
    let e2 = setup_engine(&CollisionStrategy::ReplaceWithShallowDepth);

    let engines = vec![e1, e2];
    let mut engine_index = 0;

    let mut summary_list: Vec<Summary> = Vec::new();
    println!("INFO: Benchmark search engine at depth: {depth}");
    
    for mut setup in engines {
        // Collect times for each iteration
        let mut iteration_times = Vec::with_capacity(num_iter);

        for i in 0..num_iter {
            let start = Instant::now();
            setup.engine.go(Some(depth));
            setup.engine.from(fen);
            let duration = start.elapsed();
            iteration_times.push(duration);
        }

        // --- Compute Stats ---
        let total_time = iteration_times.iter().sum::<Duration>();
        let avg_time = total_time / (num_iter as u32);

        let &min_time = iteration_times.iter().min().unwrap_or(&Duration::ZERO);
        let &max_time = iteration_times.iter().max().unwrap_or(&Duration::ZERO);

        // Convert times to f64 seconds for easier standard deviation math
        let times_in_secs: Vec<f64> = iteration_times
            .iter()
            .map(|d| d.as_secs_f64())
            .collect();

        let mean_secs = times_in_secs.iter().sum::<f64>() / (times_in_secs.len() as f64);
        let variance = times_in_secs
            .iter()
            .map(|&t| {
                let diff = t - mean_secs;
                diff * diff
            })
            .sum::<f64>()
            / (times_in_secs.len() as f64);
        let std_dev = variance.sqrt();

        summary_list.push(Summary {
            engine_index,
            total_time,
            collision_strategy: setup.collision_strategy,
            min_time,
            max_time,
            avg_time,
            std_dev,
        });

        engine_index += 1;
    }

    // Print final summary
    println!("Summary:");
    for s in &summary_list {
        println!(
            "Engine #{} => 
               total:       {:.2?}
               min/iter:    {:.2?}
               max/iter:    {:.2?}
               avg/iter:    {:.2?}
               std_dev:     {:.5}
               strategy:    {:?}
            ",
            s.engine_index,
            s.total_time,
            s.min_time,
            s.max_time,
            s.avg_time,
            s.std_dev,
            s.collision_strategy
        );
    }
}

struct Summary {
    engine_index: usize,
    collision_strategy: CollisionStrategy,

    /// Sum of all iterations
    total_time: Duration,
    /// Minimum iteration time
    min_time: Duration,
    /// Maximum iteration time
    max_time: Duration,
    /// Average (mean) iteration time
    avg_time: Duration,
    /// Standard deviation (in seconds)
    std_dev: f64,
}

struct SetupEngine {
    engine: Engine,
    collision_strategy: CollisionStrategy,
}

fn setup_engine(collision_strategy: &CollisionStrategy) -> SetupEngine {
    let mut engine = Engine::new();
    engine.new_game();
    engine.search_engine.transposition_table.collision_strategy = collision_strategy.clone();

    SetupEngine {
        engine,
        collision_strategy: collision_strategy.clone(),
    }
}