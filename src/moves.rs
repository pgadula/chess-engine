use std::iter::zip;

use crate::{
    base_types::{Color, FileRank, Move}, constants::{NOT_A, NOT_H, RANK_3, RANK_6}, game::Game
};

pub fn get_pawn_moves(game: &Game) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let mut pawns = if game.w_turn {
        game.w_pawn
    } else {
        game.b_pawn
    };
    let rank_3_or_6 = if game.w_turn { RANK_3 } else { RANK_6 };

    let blockers = game.empty_square();

    while pawns > 0 {
        let index = pawns.trailing_zeros() as u8;
        let isolated_pawn = 1u64 << index as u64;
        let single_push: u64 = if game.w_turn {
            (isolated_pawn >> 8) & blockers
        } else {
            (isolated_pawn << 8) & blockers
        };
        let double_push: u64 = if game.w_turn {
            (single_push & rank_3_or_6) >> 8 & blockers
        } else {
            (single_push & rank_3_or_6) << 8 & blockers
        };
        if single_push > 0 {
            let mv = Move {
                from: index,
                to: single_push.trailing_zeros() as u8,
            };
            println!("{}", mv);
            moves.push(mv);
        }
        if double_push > 0 {
            let mv = Move {
                from: index,
                to: double_push.trailing_zeros() as u8,
            };
            println!("{}", mv);

            moves.push(mv);
        }
        println!();

        Game::clear_bit_by_index(&mut pawns, index)
    }

    return moves;
}

pub fn _gen_rook_attacks_mask(file_rank: FileRank) -> u64 {
    let mut attacks: u64 = 0;
    let tr: u8 = file_rank.rank();
    let tf: u8 = file_rank.file();
    for f in (tf + 1)..7 {
        attacks |= 1u64 << (tr * 8 + f);
    }
    for f in (1..tf).rev() {
        attacks |= 1u64 << (tr * 8 + f);
    }
    for r in (tr + 1)..7 {
        attacks |= 1u64 << (r * 8 + tf);
    }
    for r in (1..(tr)).rev() {
        attacks |= 1u64 << (r * 8 + tf);
    }
    attacks
}

pub fn _gen_bishop_attacks_mask(file_rank: FileRank) -> u64 {
    let mut attacks: u64 = 0;
    let tr: u8 = file_rank.rank();
    let tf: u8 = file_rank.file();
    for (r, f) in zip((tr + 1)..7, (tf + 1)..7) {
        attacks |= 1u64 << (r * 8 + f);
    }
    for (r, f) in zip((1..tr).rev(), (1..(tf)).rev()) {
        attacks |= 1u64 << (r * 8 + f);
    }

    for (r, f) in zip((1..tr).rev(), (tf + 1)..7) {
        attacks |= 1u64 << (r * 8 + f);
    }

    for (r, f) in zip((tr + 1)..7, (1..(tf)).rev()) {
        attacks |= 1u64 << (r * 8 + f);
    }
    attacks
}


pub fn _get_pawn_attacks(side:Color, file_rank: FileRank)->u64{
    let tr: u8 = file_rank.rank();
    let tf: u8 = file_rank.file();
    let start = 1u64 << tr * 8 + tf;
    match side {
        Color::White => (start & NOT_H) >> 7 | (start & NOT_A) >> 9,
        Color::Black => (start & NOT_A) << 7 | (start & NOT_H) << 9,
    }
}