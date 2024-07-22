use std::iter::zip;

use crate::{
    base_types::{Color, FileRank, Move},
    file_rank::{
        FILE_NOT_A, FILE_NOT_AB, FILE_NOT_B, FILE_NOT_G, FILE_NOT_GH, FILE_NOT_H, NOT_RANK_1,
        NOT_RANK_1_2, NOT_RANK_7_8, NOT_RANK_8, RANK_3, RANK_6,
    },
    game::BitBoard, utility::bits::{pop_bit, pop_lsb},
};

pub fn get_pawn_moves(game: &BitBoard, moves: &mut [Vec<u8>; 64]) {
    let mut pawns = if game.turn == Color::White {
        game.w_pawn
    } else {
        game.b_pawn
    };
    let rank_3_or_6 = if game.turn == Color::White {
        RANK_3
    } else {
        RANK_6
    };

    let blockers = game.empty_square();

    while pawns > 0 {
        let index = pawns.trailing_zeros() as u8;
        let isolated_pawn = 1u64 << index as u64;
        let mut position: &mut Vec<u8> = &mut moves[index as usize];
        let single_push: u64 = if game.turn == Color::White {
            (isolated_pawn >> 8) & blockers
        } else {
            (isolated_pawn << 8) & blockers
        };
        let double_push: u64 = if game.turn == Color::White {
            (single_push & rank_3_or_6) >> 8 & blockers
        } else {
            (single_push & rank_3_or_6) << 8 & blockers
        };
        if single_push > 0 {
            let mv = Move {
                from: index,
                to: single_push.trailing_zeros() as u8,
            };
            position.push(single_push.trailing_zeros() as u8)
        }
        if double_push > 0 {
            let mv = Move {
                from: index,
                to: double_push.trailing_zeros() as u8,
            };

            position.push(double_push.trailing_zeros() as u8)
        }

        pop_bit(&mut pawns, index)
    }
}

pub fn _gen_rook_attacks_mask(file_rank: FileRank) -> u64 {
    let mut attacks = 0u64;
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

pub fn _gen_rook_move_fly(file_rank: FileRank, bit_board: u64) -> u64 {
    let mut attacks = 0u64;
    let tr: u8 = file_rank.rank();
    let tf: u8 = file_rank.file();
    for f in (tf + 1)..7 {
        let shift = tr * 8 + f;
        let index: u64 = 1u64 << shift;
        let is_occupied: bool = BitBoard::get_by_index(bit_board, shift as u8);
        attacks |= index;
        if is_occupied {
            break;
        }
    }
    for f in (1..tf).rev() {
        let shift = tr * 8 + f;
        let index: u64 = 1u64 << shift;
        let is_occupied = BitBoard::get_by_index(bit_board, shift as u8);
        attacks |= index;
        if is_occupied {
            break;
        }
    }
    for r in (tr + 1)..7 {
        let shift = r * 8 + tf;
        let index = 1u64 << shift;
        let is_occupied: bool = BitBoard::get_by_index(bit_board, shift as u8);
        attacks |= index;
        if is_occupied {
            break;
        }
    }
    for r in (1..(tr)).rev() {
        let shift = r * 8 + tf;
        let index = 1u64 << shift;
        let is_occupied: bool = BitBoard::get_by_index(bit_board, shift as u8);
        attacks |= index;
        if is_occupied {
            break;
        }
    }
    attacks
}

pub fn _gen_bishop_attacks_mask(file_rank: FileRank) -> u64 {
    let mut attacks = 0u64;
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

pub fn _gen_bishop_attacks_on_the_fly(file_rank: FileRank, bit_board: u64) -> u64 {
    let mut attacks = 0u64;
    let tr: u8 = file_rank.rank();
    let tf: u8 = file_rank.file();
    for (r, f) in zip((tr + 1)..7, (tf + 1)..7) {
        let shift = r * 8 + f;
        attacks |= 1u64 << shift;
        let oc = BitBoard::get_by_index(bit_board, shift);
        if oc {
            break;
        }
    }
    for (r, f) in zip((1..tr).rev(), (1..(tf)).rev()) {
        let shift = r * 8 + f;
        attacks |= 1u64 << shift;
        let oc = BitBoard::get_by_index(bit_board, shift);
        if oc {
            break;
        }
    }

    for (r, f) in zip((1..tr).rev(), (tf + 1)..7) {
        let shift = r * 8 + f;
        attacks |= 1u64 << shift;
        let oc = BitBoard::get_by_index(bit_board, shift);
        if oc {
            break;
        }
    }

    for (r, f) in zip((tr + 1)..7, (1..(tf)).rev()) {
        let shift = r * 8 + f;
        attacks |= 1u64 << shift;
        let oc = BitBoard::get_by_index(bit_board, shift);
        if oc {
            break;
        }
    }
    attacks
}

pub fn get_pawn_attacks(side: Color, file_rank: FileRank) -> u64 {
    let tr: u8 = file_rank.rank();
    let tf: u8 = file_rank.file();
    let start = 1u64 << tr * 8 + tf;
    match side {
        Color::White => (start & FILE_NOT_H) >> 7 | (start & FILE_NOT_A) >> 9,
        Color::Black => (start & FILE_NOT_A) << 7 | (start & FILE_NOT_H) << 9,
    }
}

pub fn get_knight_attacks(file_rank: FileRank) -> u64 {
    let mut attacks = 0u64;
    let tr: u8 = file_rank.rank();
    let tf: u8 = file_rank.file();
    let start = 1u64 << tr * 8 + tf;
    attacks |= (start & (FILE_NOT_H & NOT_RANK_7_8)) >> 15;
    attacks |= (start & (FILE_NOT_A & NOT_RANK_7_8)) >> 17;
    attacks |= (start & (FILE_NOT_A & NOT_RANK_1_2)) << 15;
    attacks |= (start & (FILE_NOT_H & NOT_RANK_1_2)) << 17;
    attacks |= (start & (FILE_NOT_GH & NOT_RANK_8)) >> 6;
    attacks |= (start & (FILE_NOT_AB & NOT_RANK_8)) >> 10;
    attacks |= (start & (FILE_NOT_AB & NOT_RANK_1)) << 6;
    attacks |= (start & (FILE_NOT_GH & NOT_RANK_1)) << 10;

    attacks
}

pub fn get_king_attacks(file_rank: FileRank) -> u64 {
    let tr: u8 = file_rank.rank();
    let tf: u8 = file_rank.file();
    let start = 1u64 << tr * 8 + tf;
    let mut attacks = 0u64;

    attacks |= (start & (NOT_RANK_8)) >> 8;
    attacks |= (start & (NOT_RANK_8 & FILE_NOT_H)) >> 7;
    attacks |= (start & (NOT_RANK_8 & FILE_NOT_A)) >> 9;

    attacks |= (start & (NOT_RANK_1)) << 8;
    attacks |= (start & (NOT_RANK_1 & FILE_NOT_A)) << 7;
    attacks |= (start & (NOT_RANK_1 & FILE_NOT_H)) << 9;

    attacks |= (start & (FILE_NOT_H)) << 1;
    attacks |= (start & (FILE_NOT_A)) >> 1;

    attacks
}


pub fn fill_moves(mut bit_moves: u64, position: &mut Vec<u8>, move_counter: &mut u32) {
    while bit_moves > 0 {
        let i: u8 = pop_lsb(&mut bit_moves) as u8;
        position.push(i);
        *move_counter += 1;
    }
}