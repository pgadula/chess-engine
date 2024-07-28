use std::iter::zip;

use crate::{
    base_types::{Color, FileRank, PieceLocation, PieceType},
    file_rank::{
        FILE_NOT_A, FILE_NOT_AB, FILE_NOT_GH, FILE_NOT_H, NOT_RANK_1,
        NOT_RANK_1_2, NOT_RANK_7_8, NOT_RANK_8, RANK_3, RANK_6,
    },
    utility::bits::{pop_bit, pop_lsb},
    BitBoard,
};

pub fn get_pawn_moves(color: Color, pawns:u64, all_blockers: u64, moves: &mut [Vec<FileRank>; 64], attacked_squared: &mut [Vec<PieceLocation>; 64]) {
    let mut pawns = pawns;

    let rank_3_or_6 = if color == Color::White {
        RANK_3
    } else {
        RANK_6
    };

    while pawns > 0 {
        let index = pawns.trailing_zeros();
        let isolated_pawn = 1u64 << index as u64;
        let mut position: &mut Vec<FileRank> = &mut moves[index as usize];
        let single_push: u64 = if color == Color::White {
            (isolated_pawn >> 8) & all_blockers
        } else {
            (isolated_pawn << 8) & all_blockers
        };
        let double_push: u64 = if color == Color::White {
            (single_push & rank_3_or_6) >> 8 & all_blockers
        } else {
            (single_push & rank_3_or_6) << 8 & all_blockers
        };
        if single_push > 0 {
            let attack_index = single_push.trailing_zeros() as usize;
            let fr = FileRank::get_file_rank(attack_index as u8).unwrap();
            attacked_squared[attack_index].push(PieceLocation{
                color:color,
                file_rank:FileRank::get_file_rank(index as u8).unwrap(),
                piece: PieceType::Pawn
            });
            position.push( fr);
        }
        if double_push > 0 {
            let attack_index: usize = double_push.trailing_zeros() as usize;
            let fr = FileRank::get_file_rank(attack_index as u8).unwrap();
            position.push(fr);
            attacked_squared[attack_index].push(PieceLocation{
                color:color,
                file_rank:FileRank::get_file_rank(index as u8).unwrap(),
                piece: PieceType::Pawn
            });
        }

        pop_bit(&mut pawns, index as u8)
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

pub fn fill_moves(file_rank: FileRank, piece: PieceType, mut bit_moves: u64, position: &mut Vec<FileRank>) {
    while bit_moves > 0 {
        let i:usize = pop_lsb(&mut bit_moves) as usize;
        position.push(FileRank::get_file_rank(i as u8).unwrap());
    }
}
