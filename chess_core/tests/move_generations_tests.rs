#[cfg(test)]
mod tests {
    use chess_core::bitboard::{FenParser, GameState};

    // Test function that will run `perft` against all positions in `TEST_CASES`
    #[test]
    fn test_perft_positions() {
        for test_case in TEST_CASES {
            // Create a new game state from the FEN string of the test case
            let mut game = GameState::deserialize(test_case.fen);
            println!("current fen:{}", test_case.fen);
            let (total_nodes, _) = game.perft(test_case.depth as usize);

            // Compare the calculated nodes to the expected nodes
            assert_eq!(
                total_nodes, test_case.nodes,
                "Failed at FEN: {} for depth: {}, expected nodes: {}, but got: {}",
                test_case.fen, test_case.depth, test_case.nodes, total_nodes
            );
        }
    }

    // Test function that will run `perft` against all positions in `TEST_CASES`
    #[test]
    fn test_unmake_fn() {
        for test_case in TEST_CASES {
            // Create a new game state from the FEN string of the test case
            let mut game = GameState::deserialize(test_case.fen);
            println!();
            println!();
            println!("[starting fen]:{}", test_case.fen);
            inner_nodes(game, 1);
        }
    }

    fn inner_nodes(mut original_game: GameState, max_depth: usize) {
        let expected_hash = original_game.hash;
        let fen = GameState::serialize(&original_game);
        println!("------------------------------");
        println!("current fen:{} hash: {}", fen, expected_hash);

        original_game.calculate_pseudolegal_moves();
        for mv in original_game.get_valid_moves() {
            let mut cloned_game = original_game.clone();
            cloned_game.make_move(&mv);
            cloned_game.unmake_move();
            assert_eq!(
                expected_hash, cloned_game.hash,
                "Failed at starting FEN: {} after move {} {:?} {:?} and unmake expected hash: {}, but got: {}, depth: {}",
                fen, mv.uci(), mv.move_type, mv.piece,  expected_hash, cloned_game.hash, max_depth
            );
            if max_depth > 0 {
                let mut inner_game = original_game.clone();
                inner_game.make_move(&mv);
                inner_nodes(GameState::deserialize(&GameState::serialize(&inner_game)), max_depth-1);
            }
        }
    }
    
    #[derive(Debug, Clone)]
    pub struct TestPosition {
        pub depth: u8,
        pub nodes: usize,
        pub fen: &'static str,
    }

    pub const TEST_CASES: &[TestPosition] = &[
        TestPosition {
            depth: 1,
            nodes: 8,
            fen: "r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2",
        },
        TestPosition {
            depth: 1,
            nodes: 8,
            fen: "8/8/8/2k5/2pP4/8/B7/4K3 b - d3 0 3",
        },
        TestPosition {
            depth: 1,
            nodes: 19,
            fen: "r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 2 2",
        },
        TestPosition {
            depth: 1,
            nodes: 5,
            fen: "r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQkq - 3 2",
        },
        TestPosition {
            depth: 1,
            nodes: 44,
            fen: "2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2",
        },
        TestPosition {
            depth: 1,
            nodes: 39,
            fen: "rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w KQ - 3 9",
        },
        TestPosition {
            depth: 1,
            nodes: 9,
            fen: "2r5/3pk3/8/2P5/8/2K5/8/8 w - - 5 4",
        },
        TestPosition {
            depth: 3,
            nodes: 62379,
            fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        },
        TestPosition {
            depth: 3,
            nodes: 89890,
            fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        },
        TestPosition {
            depth: 4,
            nodes: 10138,
            fen: "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1",
        },
        TestPosition {
            depth: 6,
            nodes: 1015133,
            fen: "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1",
        },
        TestPosition {
            depth: 6,
            nodes: 1440467,
            fen: "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1",
        },
        TestPosition {
            depth: 6,
            nodes: 661072,
            fen: "5k2/8/8/8/8/8/8/4K2R w K - 0 1",
        },
        TestPosition {
            depth: 6,
            nodes: 803711,
            fen: "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1",
        },
        TestPosition {
            depth: 4,
            nodes: 1274206,
            fen: "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1",
        },
        TestPosition {
            depth: 4,
            nodes: 1720476,
            fen: "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1",
        },
        TestPosition {
            depth: 6,
            nodes: 3821001,
            fen: "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1",
        },
        TestPosition {
            depth: 5,
            nodes: 1004658,
            fen: "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1",
        },
        TestPosition {
            depth: 6,
            nodes: 217342,
            fen: "4k3/1P6/8/8/8/8/K7/8 w - - 0 1",
        },
        TestPosition {
            depth: 6,
            nodes: 92683,
            fen: "8/P1k5/K7/8/8/8/8/8 w - - 0 1",
        },
        TestPosition {
            depth: 6,
            nodes: 2217,
            fen: "K1k5/8/P7/8/8/8/8/8 w - - 0 1",
        },
        TestPosition {
            depth: 7,
            nodes: 567584,
            fen: "8/k1P5/8/1K6/8/8/8/8 w - - 0 1",
        },
        TestPosition {
            depth: 4,
            nodes: 23527,
            fen: "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1",
        },
    ];

    pub const TEST_POSITIONS2: &[TestPosition] = &[
        TestPosition {
            depth: 1,
            nodes: 45,
            fen: "rnbq1k1r/pp1Pb1pp/2p2p2/8/2B5/2P5/PP2NnPP/RNBQK2R w KQ - 0 9",
        },
        TestPosition {
            depth: 1,
            nodes: 44,
            fen: "rnbq1k1r/p2Pbppp/1pp5/8/2B5/2P5/PP2NnPP/RNBQK2R w KQ - 0 9",
        },
        TestPosition {
            depth: 1,
            nodes: 44,
            fen: "rnbq1k1r/1p1Pbppp/p1p5/8/2B5/2P5/PP2NnPP/RNBQK2R w KQ - 0 9",
        },
        TestPosition {
            depth: 1,
            nodes: 44,
            fen: "rnbq1k1r/pp1Pbppp/8/2p5/2B5/2P5/PP2NnPP/RNBQK2R w KQ - 0 9",
        },
        TestPosition {
            depth: 1,
            nodes: 44,
            fen: "rnbq1k1r/pp1Pbpp1/2p5/7p/2B5/2P5/PP2NnPP/RNBQK2R w KQ - 0 9",
        },
        TestPosition {
            depth: 1,
            nodes: 43,
            fen: "rnbq1k1r/pp1Pbp1p/2p5/6p1/2B5/2P5/PP2NnPP/RNBQK2R w KQ - 0 9",
        },
        TestPosition {
            depth: 1,
            nodes: 45,
            fen: "rnbq1k1r/pp1Pb1pp/2p5/5p2/2B5/2P5/PP2NnPP/RNBQK2R w KQ - 0 9",
        },
        TestPosition {
            depth: 1,
            nodes: 43,
            fen: "rnbq1k1r/p2Pbppp/2p5/1p6/2B5/2P5/PP2NnPP/RNBQK2R w KQ - 0 9",
        },
        TestPosition {
            depth: 1,
            nodes: 44,
            fen: "rnbq1k1r/1p1Pbppp/2p5/p7/2B5/2P5/PP2NnPP/RNBQK2R w KQ - 0 9",
        },
        TestPosition {
            depth: 1,
            nodes: 34,
            fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/1P6/P1P1NnPP/RNBQK2R b KQ - 0 8",
        },
        TestPosition {
            depth: 1,
            nodes: 44,
            fen: "rnbq1kr1/pp1Pbppp/2p5/8/2B5/1P6/P1P1NnPP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 44,
            fen: "rnbq2kr/pp1Pbppp/2p5/8/2B5/1P6/P1P1NnPP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 52,
            fen: "rnb1qk1r/pp1Pbppp/2p5/8/2B5/1P6/P1P1NnPP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 40,
            fen: "rnb2k1r/pp1qbppp/2p5/8/2B5/1P6/P1P1NnPP/RNBQK2R w KQ - 0 9",
        },
        TestPosition {
            depth: 1,
            nodes: 48,
            fen: "rnb2k1r/ppqPbppp/2p5/8/2B5/1P6/P1P1NnPP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 47,
            fen: "rnb2k1r/pp1Pbppp/1qp5/8/2B5/1P6/P1P1NnPP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 9,
            fen: "rnb2k1r/pp1Pbppp/2p5/q7/2B5/1P6/P1P1NnPP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 41,
            fen: "rn1q1k1r/pp1bbppp/2p5/8/2B5/1P6/P1P1NnPP/RNBQK2R w KQ - 0 9",
        },
        TestPosition {
            depth: 1,
            nodes: 41,
            fen: "r1bq1k1r/pp1nbppp/2p5/8/2B5/1P6/P1P1NnPP/RNBQK2R w KQ - 0 9",
        },
        TestPosition {
            depth: 1,
            nodes: 44,
            fen: "r1bq1k1r/pp1Pbppp/n1p5/8/2B5/1P6/P1P1NnPP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 44,
            fen: "rnbq1k1r/pp1P1ppp/2p2b2/8/2B5/1P6/P1P1NnPP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 44,
            fen: "rnbq1k1r/pp1P1ppp/2pb4/8/2B5/1P6/P1P1NnPP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 42,
            fen: "rnbq1k1r/pp1P1ppp/2p5/6b1/2B5/1P6/P1P1NnPP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 43,
            fen: "rnbq1k1r/pp1P1ppp/2p5/2b5/2B5/1P6/P1P1NnPP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 42,
            fen: "rnbq1k1r/pp1P1ppp/2p5/8/2B4b/1P6/P1P1NnPP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 8,
            fen: "rnbq1k1r/pp1P1ppp/2p5/8/1bB5/1P6/P1P1NnPP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 42,
            fen: "rnbq1k1r/pp1P1ppp/2p5/8/2B5/bP6/P1P1N1PP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 42,
            fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B1n3/1P6/P1P1N1PP/RNBQK2R w KQ - 1 9",
        },
        TestPosition {
            depth: 1,
            nodes: 41,
            fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/1P1n4/P1P1N1PP/RNBQK2R w KQ - 1 9",
        },
    ];
}
