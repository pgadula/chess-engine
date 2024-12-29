#[cfg(test)]
mod tests {
    use chess_core::{
        bitboard::{ GameState, TEMP_VALID_MOVE_SIZE}, fen::FenParser, types::PieceMove
    };

    use crate::{PERFT_NEW_GAME_TESTS, PERFT_POSITION_5};

    #[test]
    fn zorbrist_hash_gen_tests() {
        let mut game = GameState::new_game();

        let mv_gen = PieceMove::from_uci;
        let mv_uci_list = [
            "a2a4", "h7h5", // White pushes a-pawn, black does a random move
            "a4a5", "h5h4",
            "a5a6", "h4h3",
            "a6b7", "h3g2",  // Now white is ready to promote

        ];
        let mut move_index = 0;
        for mv_uci in mv_uci_list {
            let mv = mv_gen(&mv_uci, &game);
            game.make_move(&mv);
            game.println();
            let fen = GameState::serialize(&game);
            let deep_cloned = GameState::deserialize(&fen);
            let expected_hash = game.zobrist_hashing.get_hash_from_scratch(&game);
            for row in 0..8 {
                for col in 0..8 {
                    print!("{:?}", game.board[col+row*8])
                }
                println!()
            }

            assert_eq!(deep_cloned.hash , expected_hash, "After deep cloned failed on hash");
            assert_eq!(
                game.hash, expected_hash,
                "Zobrist hash mismatch after move {mv_uci}, move index {move_index}"
            );
        }
        move_index = move_index + 1;
    }

    #[test]
    fn test_perft_positions() {
        for test_case in TEST_CASES {
            // Create a new game state from the FEN string of the test case
            let game = GameState::deserialize(test_case.fen);
            println!("current fen:{}", test_case.fen);
            let (total_nodes, _) = game.perft(test_case.depth as usize);

            assert_eq!(
                total_nodes, test_case.nodes,
                "Failed at FEN: {} for depth: {}, expected nodes: {}, but got: {}",
                test_case.fen, test_case.depth, test_case.nodes, total_nodes
            );
        }
    }

    #[test]
    fn test_for_new_game() {
        let depth: usize = 6;
        let game = GameState::new_game();
        let (total_nodes, _) = game.perft(depth as usize);
        let expected_total_nodes = PERFT_NEW_GAME_TESTS[depth].total_nodes;
        assert_eq!(
            total_nodes, expected_total_nodes,
            "Failed for depth: {}, expected nodes: {}, but got: {}",
            depth, expected_total_nodes, total_nodes
        );
    }

    #[test]
    fn test_for_position_5() {
        let depth: usize = 5;
        let game = GameState::deserialize("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
        let (total_nodes, _) = game.perft(depth as usize);
        let expected_total_nodes = PERFT_POSITION_5.last().unwrap().total_nodes;
        assert_eq!(
            total_nodes, expected_total_nodes,
            "Failed for depth: {}, expected nodes: {}, but got: {}",
            depth, expected_total_nodes, total_nodes
        );
    }


    // Test function that will run `perft` against all positions in `TEST_CASES`
    #[test]
    fn test_unmake_fn() {
        for test_case in TEST_CASES {
            // Create a new game state from the FEN string of the test case
            let game = GameState::deserialize(test_case.fen);
            println!("[starting fen]:{}", test_case.fen);
            inner_nodes(game, 3);
        }
    }

    fn inner_nodes(mut original_game: GameState, max_depth: usize) {
        let expected_hash = original_game.hash;
        let fen = GameState::serialize(&original_game);
        println!("------------------------------");
        println!("current fen:{} hash: {}", fen, expected_hash);

        original_game.calculate_pseudolegal_moves();
        let mut valid_moves = [PieceMove::default(); TEMP_VALID_MOVE_SIZE];
        let count = original_game.fill_valid_moves(&mut valid_moves);
        let mut cloned_game = original_game.clone();
        for mv in &valid_moves[..count] {
            cloned_game.make_move(&mv);
            cloned_game.unmake_move();
            cloned_game.hash = cloned_game
                .zobrist_hashing
                .get_hash_from_scratch(&cloned_game);

            assert_eq!(
                    expected_hash, cloned_game.hash,
                    "Failed at starting FEN: {} after move {} {:?} {:?} and unmake expected hash: {}, but got: {}, depth: {}",
                    fen, mv.uci(), mv.move_type, mv.piece,  expected_hash, cloned_game.hash, max_depth
                );

            if max_depth > 0 {
                let mut inner_game = original_game.clone();
                inner_game.make_move(&mv);
                inner_game.history.clear();
                inner_nodes(inner_game, max_depth - 1);
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
}

#[derive(Debug, Clone)]
struct PerftCase {
    depth: u8,
    total_nodes: usize,
}
const PERFT_NEW_GAME_TESTS: &[PerftCase] = &[
    PerftCase {
        depth: 0,
        total_nodes: 1,
    },
    PerftCase {
        depth: 1,
        total_nodes: 20,
    },
    PerftCase {
        depth: 2,
        total_nodes: 400,
    },
    PerftCase {
        depth: 3,
        total_nodes: 8_902,
    },
    PerftCase {
        depth: 4,
        total_nodes: 197_281,
    },
    PerftCase {
        depth: 5,
        total_nodes: 4_865_609,
    },
    PerftCase {
        depth: 6,
        total_nodes: 119_060_324,
    },
    PerftCase {
        depth: 7,
        total_nodes: 3_195_901_860,
    },
    PerftCase {
        depth: 8,
        total_nodes: 84_998_978_956,
    },
    PerftCase {
        depth: 9,
        total_nodes: 2_439_530_234_167,
    },
    PerftCase {
        depth: 10,
        total_nodes: 69_352_859_712_417,
    },
    PerftCase {
        depth: 11,
        total_nodes: 2_097_651_003_696_806,
    },
    PerftCase {
        depth: 12,
        total_nodes: 62_854_969_236_701_747,
    },
    PerftCase {
        depth: 13,
        total_nodes: 1_981_066_775_000_396_239,
    },
];


const PERFT_POSITION_5: &[PerftCase] = &[
    PerftCase {
        depth: 0,
        total_nodes: 44,
    },
    PerftCase {
        depth: 1,
        total_nodes: 62_379,
    },
    PerftCase {
        depth: 2,
        total_nodes: 2_103_487,
    },
    PerftCase {
        depth: 3,
        total_nodes: 89_941_194,
    },
];