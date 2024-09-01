use std::iter::zip;

use crate::{bitboard::BitBoard, file_rank::{
    self, FILE_NOT_A, FILE_NOT_AB, FILE_NOT_GH, FILE_NOT_H, NOT_RANK_1, NOT_RANK_1_2, NOT_RANK_7_8, NOT_RANK_8, RANK_1, RANK_3, RANK_6, RANK_8
}, types::{Color, FileRank, MoveType, Piece, PieceMove, PieceType, PROMOTION_PIECES}, utility::{get_file_ranks, pop_bit, pop_lsb, print_as_board, set_bit_by_index}};


pub fn get_pawn_moves(
    color: Color,
    pawns: u64,
    all_blockers: u64,
    opposite_blockers: u64,
    en_passant: &Option<FileRank>,
    attack_mask:& mut u64,
    flat_attacks: &mut Vec<PieceMove>
) {
    let mut pawns = pawns;

    let rank_3_or_6 = if color == Color::White {
        RANK_3
    } else {
        RANK_6
    };
    
    while pawns > 0 {
        let index = pawns.trailing_zeros();
        let pawn_file_rank = FileRank::get_file_rank(index as u8).unwrap();
        let en_passant_mask = if let Some(e) = en_passant  {
            let mut mask = 0u64;
            set_bit_by_index(&mut mask, e.index() as u8);
            mask
        }else{
            0u64
        };
        let mut attack_pattern = get_pawn_pattern_attacks(color, pawn_file_rank) & (opposite_blockers | en_passant_mask);

        let isolated_pawn = 1u64 << index as u64;
        let single_push: u64 = if color == Color::White {
            (isolated_pawn >> 8) & all_blockers
        } else {
            (isolated_pawn << 8) & all_blockers
        };

        for file_rank in get_file_ranks(attack_pattern){
            if file_rank.mask() & RANK_8 > 0 || file_rank.mask() & RANK_1 > 0{
                for piece_type in PROMOTION_PIECES {
                    flat_attacks.push(PieceMove{
                        from: pawn_file_rank,
                        piece: Piece::from(&PieceType::Pawn, &color),
                        target: file_rank,
                        move_type: MoveType::CaptureWithPromotion(piece_type)
                    }) 
                }
            }else{
                flat_attacks.push(PieceMove{
                    from: pawn_file_rank,
                    piece: Piece::from(&PieceType::Pawn, &color),
                    target: file_rank,
                    move_type: MoveType::Capture
                })
            }
         
        }
        for file_rank in get_file_ranks(single_push){
            if file_rank.mask() & RANK_8 > 0 || file_rank.mask() & RANK_1 > 0{
                for piece_type in PROMOTION_PIECES {
                    flat_attacks.push(PieceMove{
                        from: pawn_file_rank,
                        piece: Piece::from(&PieceType::Pawn, &color),
                        target: file_rank,
                        move_type: MoveType::Promotion(piece_type)
                    }) 
                }
            }else{
                flat_attacks.push(PieceMove{
                    from: pawn_file_rank,
                    piece: Piece::from(&PieceType::Pawn, &color),
                    target: file_rank,
                    move_type:MoveType::Quite
                })
            }
         
        }
        let double_push: u64 = if color == Color::White {
            (single_push & rank_3_or_6) >> 8 & all_blockers
        } else {
            (single_push & rank_3_or_6) << 8 & all_blockers
        };
        let all_moves_mask = single_push | double_push | attack_pattern;
        *attack_mask |= all_moves_mask;

       for file_rank in get_file_ranks(double_push) {
            let en_passant_fr = if color == Color::White{
                FileRank::get_file_rank(( file_rank.mask() >>  8) as u8).unwrap()
            }
            else{
                FileRank::get_file_rank(( file_rank.mask() << 8) as u8).unwrap()
            };

            flat_attacks.push(PieceMove{
                from: pawn_file_rank,
                piece: Piece::from(&PieceType::Pawn, &color),
                target: file_rank,
                move_type: MoveType::DoublePush(Some(en_passant_fr))
            })
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
    for f in (tf + 1)..8 {
        let shift = tr * 8 + f;
        let index: u64 = 1u64 << shift;
        let is_occupied: bool = BitBoard::get_by_index(bit_board, shift as u8);
        attacks |= index;
        if is_occupied {
            break;
        }
    }
    for f in (0..tf).rev() {
        let shift = tr * 8 + f;
        let index: u64 = 1u64 << shift;
        let is_occupied = BitBoard::get_by_index(bit_board, shift as u8);
        attacks |= index;
        if is_occupied {
            break;
        }
    }
    for r in (tr + 1)..8 {
        let shift = r * 8 + tf;
        let index = 1u64 << shift;
        let is_occupied: bool = BitBoard::get_by_index(bit_board, shift as u8);
        attacks |= index;
        if is_occupied {
            break;
        }
    }
    for r in (0..(tr)).rev() {
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
    for (r, f) in zip((tr + 1)..8, (tf + 1)..8) {
        let shift = r * 8 + f;
        attacks |= 1u64 << shift;
        let oc = BitBoard::get_by_index(bit_board, shift);
        if oc {
            break;
        }
    }
    for (r, f) in zip((0..tr).rev(), (0..(tf)).rev()) {
        let shift = r * 8 + f;
        attacks |= 1u64 << shift;
        let oc = BitBoard::get_by_index(bit_board, shift);
        if oc {
            break;
        }
    }

    for (r, f) in zip((0..tr).rev(), (tf + 1)..8) {
        let shift = r * 8 + f;
        attacks |= 1u64 << shift;
        let oc = BitBoard::get_by_index(bit_board, shift);
        if oc {
            break;
        }
    }

    for (r, f) in zip((tr + 1)..8, (0..(tf)).rev()) {
        let shift = r * 8 + f;
        attacks |= 1u64 << shift;
        let oc = BitBoard::get_by_index(bit_board, shift);
        if oc {
            break;
        }
    }
    attacks
}

pub fn get_pawn_pattern_attacks(side: Color, file_rank: FileRank) -> u64 {
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

pub fn fill_moves(
    piece_file_rank: FileRank,
    piece: Piece,
    mut bit_moves: u64,
    flat_attacks: &mut Vec<PieceMove>,
    opposite_blockers: u64
) {

    while bit_moves > 0 {
        let i: usize = pop_lsb(&mut bit_moves) as usize;
        let fr = FileRank::get_file_rank(i as u8).unwrap();
        let move_type = if opposite_blockers & fr.mask() > 0 {
            MoveType::Capture
        } else{
            MoveType::Quite
        };
        flat_attacks.push(PieceMove{
            piece,
            from: piece_file_rank,
            target: fr,
            move_type
        });

    }
}
