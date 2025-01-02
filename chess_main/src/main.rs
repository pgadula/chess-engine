mod sandbox_main;

use std::{io, mem::transmute};

use chess_core::{
    types::{Color, FileRank, MoveType, Piece, PieceType, FILE_RANK},
    utility::print_as_board,
};
use chess_uci::engine::Engine;
use sandbox_main::debug_move;

struct PackedPieceMove {
    mask: u64,
}
const MOVE_MASKS: u64 = 0b111_111_1;
//                    color piece_type
//                        ^ ^

impl PackedPieceMove {
    pub fn pack(
        from: u8,
        to: u8,
        color: u8,
        piece_type: u8,
        move_type: u8,
        promotion: u8,
        en_passant: u8
    ) -> PackedPieceMove {
        let mut mask: u64 = 0;
        let piece = 0xFF & (color << 0) | (piece_type << 1);
        mask |= (from as u64 & MOVE_MASKS) << 0;
        mask |= (to as u64 & MOVE_MASKS) << 8;
        mask |= (piece as u64) << 16;
        mask |= (move_type as u64) << 24;
        mask |= (promotion as u64) << 32;
        mask |= (en_passant as u64) << 40;

        return PackedPieceMove { mask };
    }

    pub fn get_from(&self) -> FileRank {
        let index = 0xFF & (self.mask >> 0);
        FILE_RANK[index as usize]
    }

    pub fn get_to(&self) -> FileRank {
        let index = 0xFF & (self.mask >> 8);
        FILE_RANK[index as usize]
    }

    pub fn get_color(&self) -> Color {
        let index = 0b1 & (self.mask >> 16);
        if index == 0{
            return Color::White
        }else{
            return Color::Black
        }
    }
    
    pub fn get_piece_type(&self) -> PieceType {
        let index = (0b111_111_0 & (self.mask >> 17)) ;
        return unsafe {
            transmute(index as u8)
        }
    }

    pub fn get_move_type(&self) -> u64 {
        let index = (0xFF & (self.mask >> 24)) ;
        return index;
    }

    pub fn get_promotion_piece(&self) -> u64 {
        let index = 0xFF & (self.mask >> 32) ;
        return index;
    }

    pub fn get_en_passant(&self) -> FileRank {
        let index = 0xFF & (self.mask >> 40) ;
        FILE_RANK[index as usize]    
    }
}

fn main() {
    // let mut engine = Engine::new();
    // while engine.is_running {
    //     let mut buf = String::from("");
    //     let _ = io::stdin().read_line(&mut buf);
    //     let chars = &buf.chars().collect::<Vec<char>>();
    //     engine.process_command(chars);
    // }

    let packed = PackedPieceMove::pack(
        FileRank::E2.index() as u8,
        FileRank::H8.index() as u8,
        0,
        PieceType::Bishop as u8,
        MoveType::EnPassantCapture.value() as u8,
        PieceType::Knight as u8,
        0
    );

    let from = packed.get_from();
    let to: FileRank = packed.get_to();
    let color  = packed.get_color();
    let piece  = packed.get_piece_type();
    let move_type  = packed.get_move_type();
    let promotion_piece_type  = packed.get_promotion_piece();


    println!("from {:?}", from);
    println!("to {:?}", to);
    println!("color {:?}", color);
    println!("piece {:?}", piece);
    println!("move_Type {:?}", move_type);
    println!("promotion_piece_type {:?}", promotion_piece_type);





    // debug_move();
}
