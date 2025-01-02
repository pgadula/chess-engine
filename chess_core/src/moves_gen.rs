use std::iter::zip;

use crate::{bitboard::GameState, file_rank::{
    FILE_A, FILE_NOT_A, FILE_NOT_AB, FILE_NOT_GH, FILE_NOT_H, NOT_RANK_1, NOT_RANK_1_2, NOT_RANK_7_8, NOT_RANK_8, RANK_1, RANK_3, RANK_6, RANK_8
}, precalculated::PAWN_ATTACK_MASK, types::{Color, FileRank, MoveBuffer, MoveType, Piece, PieceMove, BLACK_PAWN, FILE_RANK, PROMOTION_PIECES, WHITE_PAWN}, utility::{get_file_ranks, pop_bit, pop_lsb, print_as_board, set_bit_by_index}};


pub fn get_pawn_moves(
    color: Color,
    pawns: u64,
    all_blockers: u64,
    opposite_blockers: u64,
    en_passant: &Option<FileRank>,
    attack_mask:& mut u64,
    quite_attacks: &mut Vec<PieceMove>,
    capture_attacks: &mut Vec<PieceMove>

) {
    let mut pawns = pawns;
    let rank_3_or_6 = if color == Color::White {RANK_3} else {RANK_6};
    let shift_for_color = if color == Color::White {|x: u64| x >> 8} else {|x: u64| x << 8};

    let piece: Piece = match color {
        Color::White => WHITE_PAWN,
        Color::Black => BLACK_PAWN,
    };
    
    let en_passant_mask = if let Some(file_rank) = en_passant  {
        let mut mask = 0u64;
        set_bit_by_index(&mut mask, file_rank.index() as u8);
        mask
    }else{
        0u64
    };

    let attack_index_offset:usize = match color {
        Color::White => 0,
        Color::Black => 64,
    };

    
    while pawns > 0 {
        let index = pawns.trailing_zeros();
        let from = FileRank::get_file_rank(index as u8).unwrap();

        let attack_pattern = PAWN_ATTACK_MASK[index as usize + attack_index_offset] & (opposite_blockers | en_passant_mask);

        let isolated_pawn = 1u64 << index as u64;
        
        let single_push: u64 = shift_for_color(isolated_pawn) & all_blockers;

        let double_push: u64 = shift_for_color(single_push & rank_3_or_6)  & all_blockers;
        let all_moves_mask = single_push | double_push | attack_pattern;

        *attack_mask |= all_moves_mask;

        for target in get_file_ranks(all_moves_mask){
            let file_rank_mask = target.mask();
           let is_capture_attack =  (attack_pattern & file_rank_mask) > 0;
           let is_single_push =  (single_push & file_rank_mask) > 0;
           let is_double_push =  (double_push & file_rank_mask) > 0;

            if is_capture_attack {
                if target.mask() & RANK_8 > 0 || target.mask() & RANK_1 > 0{
                    for piece_type in PROMOTION_PIECES {
                        capture_attacks.push(PieceMove{
                            from,
                            piece,
                            target,
                            move_type: MoveType::CaptureWithPromotion(piece_type)
                        }) 
                    }
                }else{
                    capture_attacks.push(PieceMove{
                        from,
                        piece,
                        target,
                        move_type: MoveType::Capture
                    })
                }
            }
            if is_single_push{
                if target.mask() & RANK_8 > 0 || target.mask() & RANK_1 > 0{
                    for piece_type in PROMOTION_PIECES {
                        capture_attacks.push(PieceMove{
                            from,
                            piece,
                            target,
                            move_type: MoveType::Promotion(piece_type)
                        }) 
                    }
                }else{
                    quite_attacks.push(PieceMove{
                        from,
                        piece,
                        target,
                        move_type:MoveType::Quiet
                    })
                }
            }
            if is_double_push{
                let en_passant_fr = if color == Color::White{
                    FileRank::get_from_mask(target.mask() <<  8).unwrap()
                }
                else{
                    FileRank::get_from_mask(target.mask() >> 8 ).unwrap()
                };
    
                quite_attacks.push(PieceMove{
                    from,
                    piece,
                    target,
                    move_type: MoveType::DoublePush(Some(en_passant_fr))
                })
            }
        
        }

        pop_bit(&mut pawns, index as u8)
    }
}

pub fn get_pawn_moves_vectorized(
    color: Color,
    pawns: u64,            
    empty_squares: u64,    
    opposite_blockers: u64, 
    en_passant: &Option<FileRank>,
    attack_mask: &mut u64,
    quiet_moves: &mut MoveBuffer,
    capture_moves: &mut MoveBuffer,
) {


    let en_passant_mask = if let Some(fr) = en_passant {
        1u64 << fr.index()
    } else {
        0
    };

    let (single_push_all, double_push_all) = match color {
        Color::White => {
            let single = (pawns >> 8) & empty_squares;
            let dbl = ((single & RANK_3) >> 8) & empty_squares;
            (single, dbl)
        }
        Color::Black => {
            let single = (pawns << 8) & empty_squares;
            let dbl = ((single & RANK_6) << 8) & empty_squares;
            (single, dbl)
        }
    };

    let capture_mask = opposite_blockers | en_passant_mask;
    let (left_attacks_all, right_attacks_all) = match color {
        Color::White => {
            let left  = (pawns >> 7) & FILE_NOT_A & capture_mask;
            let right = (pawns >> 9) & FILE_NOT_H & capture_mask;
            (left, right)
        }
        Color::Black => {
            let left  = (pawns << 7) & FILE_NOT_H & capture_mask;
            let right = (pawns << 9) & FILE_NOT_A & capture_mask;
            (left, right)
        }
    };


    *attack_mask |= single_push_all;
    *attack_mask |= double_push_all;
    *attack_mask |= left_attacks_all;
    *attack_mask |= right_attacks_all;

    let piece = if color == Color::White { WHITE_PAWN } else { BLACK_PAWN };

    let file_rank_of = |sq: u32| FILE_RANK[sq as usize]; 

    let mut sp = single_push_all;
    while sp != 0 {
        let to_sq = sp.trailing_zeros();    
        pop_bit(&mut sp, to_sq as u8);      

        let from_sq = match color {
            Color::White => to_sq + 8,
            Color::Black => to_sq - 8,
        };
        let from = file_rank_of(from_sq);
        let target   = file_rank_of(to_sq);

        let is_promotion = match color {
            Color::White => (1u64 << to_sq) & RANK_8 != 0,
            Color::Black => (1u64 << to_sq) & RANK_1 != 0,
        };

        if is_promotion {
            for piece_type in PROMOTION_PIECES {
                quiet_moves.push(PieceMove {
                    from,
                    piece,
                    target,
                    move_type: MoveType::Promotion(piece_type),
                });
            }
        } else {
            // Normal quiet single push
            quiet_moves.push(PieceMove {
                from,
                piece,
                target,
                move_type: MoveType::Quiet,
            });
        }
    }

    let mut dp = double_push_all;
    while dp != 0 {
        let to_sq = dp.trailing_zeros();
        pop_bit(&mut dp, to_sq as u8);

        let from_sq = match color {
            Color::White => to_sq + 16,
            Color::Black => to_sq - 16,
        };
        let from = file_rank_of(from_sq);
        let to   = file_rank_of(to_sq);

        let en_passant_fr = match color {
            Color::White => file_rank_of(from_sq - 8),
            Color::Black => file_rank_of(from_sq + 8),
        };

        quiet_moves.push(PieceMove {
            from,
            piece,
            target: to,
            move_type: MoveType::DoublePush(Some(en_passant_fr)),
        });
    }

    let mut la = left_attacks_all;
    while la != 0 {
        let to_sq = la.trailing_zeros();
        pop_bit(&mut la, to_sq as u8);

        let from_sq = match color {
            Color::White => to_sq + 7,
            Color::Black => to_sq - 7,
        };
        let from = file_rank_of(from_sq);
        let to   = file_rank_of(to_sq);

        let is_promotion = match color {
            Color::White => (1u64 << to_sq) & RANK_8 != 0,
            Color::Black => (1u64 << to_sq) & RANK_1 != 0,
        };

        let is_en_passant = (1u64 << to_sq) & en_passant_mask != 0;

        if is_promotion {
            for piece_type in PROMOTION_PIECES {
                capture_moves.push(PieceMove {
                    from,
                    piece,
                    target: to,
                    move_type: MoveType::CaptureWithPromotion(piece_type),
                });
            }
        } else if is_en_passant {
            capture_moves.push(PieceMove {
                from,
                piece,
                target: to,
                move_type: MoveType::Capture,
            });
        } else {
            capture_moves.push(PieceMove {
                from,
                piece,
                target: to,
                move_type: MoveType::Capture,
            });
        }
    }

    let mut ra = right_attacks_all;
    while ra != 0 {
        let to_sq = ra.trailing_zeros();
        pop_bit(&mut ra, to_sq as u8);

        let from_sq = match color {
            Color::White => to_sq + 9,
            Color::Black => to_sq - 9,
        };
        let from = file_rank_of(from_sq);
        let to   = file_rank_of(to_sq);

        let is_promotion = match color {
            Color::White => (1u64 << to_sq) & RANK_8 != 0,
            Color::Black => (1u64 << to_sq) & RANK_1 != 0,
        };

        let is_en_passant = (1u64 << to_sq) & en_passant_mask != 0;

        if is_promotion {
            for piece_type in PROMOTION_PIECES {
                capture_moves.push(PieceMove {
                    from,
                    piece,
                    target: to,
                    move_type: MoveType::CaptureWithPromotion(piece_type),
                });
            }
        } else if is_en_passant {
            capture_moves.push(PieceMove {
                from,
                piece,
                target: to,
                move_type: MoveType::Capture,
            });
        } else {
            capture_moves.push(PieceMove {
                from,
                piece,
                target: to,
                move_type: MoveType::Capture,
            });
        }
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
        let is_occupied: bool = GameState::get_by_index(bit_board, shift as u8);
        attacks |= index;
        if is_occupied {
            break;
        }
    }
    for f in (0..tf).rev() {
        let shift = tr * 8 + f;
        let index: u64 = 1u64 << shift;
        let is_occupied = GameState::get_by_index(bit_board, shift as u8);
        attacks |= index;
        if is_occupied {
            break;
        }
    }
    for r in (tr + 1)..8 {
        let shift = r * 8 + tf;
        let index = 1u64 << shift;
        let is_occupied: bool = GameState::get_by_index(bit_board, shift as u8);
        attacks |= index;
        if is_occupied {
            break;
        }
    }
    for r in (0..(tr)).rev() {
        let shift = r * 8 + tf;
        let index = 1u64 << shift;
        let is_occupied: bool = GameState::get_by_index(bit_board, shift as u8);
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
        let oc = GameState::get_by_index(bit_board, shift);
        if oc {
            break;
        }
    }
    for (r, f) in zip((0..tr).rev(), (0..(tf)).rev()) {
        let shift = r * 8 + f;
        attacks |= 1u64 << shift;
        let oc = GameState::get_by_index(bit_board, shift);
        if oc {
            break;
        }
    }

    for (r, f) in zip((0..tr).rev(), (tf + 1)..8) {
        let shift = r * 8 + f;
        attacks |= 1u64 << shift;
        let oc = GameState::get_by_index(bit_board, shift);
        if oc {
            break;
        }
    }

    for (r, f) in zip((tr + 1)..8, (0..(tf)).rev()) {
        let shift = r * 8 + f;
        attacks |= 1u64 << shift;
        let oc = GameState::get_by_index(bit_board, shift);
        if oc {
            break;
        }
    }
    attacks
}

pub fn get_pawn_pattern_attacks(side: &Color, file_rank: &FileRank) -> u64 {
    let tr: u8 = file_rank.rank();
    let tf: u8 = file_rank.file();
    let start = 1u64 << tr * 8 + tf;
    match side {
        Color::White => (start & FILE_NOT_H) >> 7 | (start & FILE_NOT_A) >> 9,
        Color::Black => (start & FILE_NOT_A) << 7 | (start & FILE_NOT_H) << 9,
    }
}

pub fn get_knight_attacks(file_rank: &FileRank) -> u64 {
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

pub fn get_king_attacks(file_rank: &FileRank) -> u64 {
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
    from: FileRank,
    piece: Piece,
    mut bit_moves: u64,
    quiet_attacks: &mut MoveBuffer,
    killer_attacks: &mut MoveBuffer,
    opposite_blockers: u64
) {

    while bit_moves > 0 {
        let i: usize = pop_lsb(&mut bit_moves) as usize;
        let fr = FileRank::get_file_rank(i as u8).unwrap();
        let move_type = if opposite_blockers & fr.mask() > 0 {
            MoveType::Capture
        } else{
            MoveType::Quiet
        };
        let mv = PieceMove{
            piece,
            from,
            target: fr,
            move_type
        };
        
        if move_type == MoveType::Quiet   {
            quiet_attacks.push(mv);
        }else{
            killer_attacks.push(mv);
        }

    }
}
