use crate::types::{Castling, Clock};
use crate::zobrist_hashing::ZobristHashing;
use crate::{
    file_rank::{
        BLACK_KING_CASTLE_MASK, BLACK_QUEEN_CASTLE_MASK, RANK_3, WHITE_KING_CASTLE_MASK,
        WHITE_QUEEN_CASTLE_MASK,
    },
    magic_gen::MoveLookupTable,
    moves_gen::{fill_moves, get_king_attacks, get_knight_attacks, get_pawn_moves},
    types::{
        get_piece_from_char, BoardSide, Color, FileRank, MoveType, Piece, PieceIndex, PieceMove,
        PieceType, BLACK_ROOK, PIECES_ARRAY, WHITE_ROOK,
    },
    utility::{clear_bit, get_file_ranks, set_bit},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GameState {
    pub bitboard: [u64; 12],

    pub turn: Color,
    pub castling: Castling,
    pub halfmove_clock: Clock,
    pub fullmove_number: Clock,
    pub en_passant: Option<FileRank>,

    //moves
    pub move_lookup_table: Arc<MoveLookupTable>,
    pub flat_white_moves: Vec<PieceMove>,
    pub flat_black_moves: Vec<PieceMove>,
    pub w_moves_mask: u64,
    pub b_moves_mask: u64,

    pub zobrssist_hashing: Arc<ZobristHashing>
}

pub trait FenParser {
    fn deserialize(fen: &str) -> GameState;
    fn serialize(&self) -> String;
}

impl GameState {
    pub fn id(&self) -> usize {
        let mut id = self
            .bitboard
            .iter()
            .copied()
            .reduce(|acc, b| acc ^ b)
            .unwrap_or(0u64);
        if let Some(en_passant) = self.en_passant {
            id ^= en_passant.mask()
        }
        return id as usize;
    }
    pub fn empty() -> GameState {
        Default::default()
    }

    pub fn apply(&mut self, piece_move: &PieceMove) {
        self.halfmove_clock += 1;
        let mut en_passant_is_updated = false;
        self.increment_full_move();

        match piece_move.move_type {
            MoveType::Capture => {
                self.handle_capture(piece_move);
                self.move_piece(piece_move);
            }
            MoveType::Promotion(piece) => self.promote(piece_move, &piece),
            MoveType::CaptureWithPromotion(piece) => {
                self.capture_with_promotion(piece_move, &piece)
            }
            MoveType::CastleKingSide => {
                self.set_and_clear(&piece_move);
                match piece_move.piece.color {
                    Color::White => {
                        self.clear_piece(&WHITE_ROOK, &FileRank::H1);
                        self.set_piece(&WHITE_ROOK, &FileRank::F1);
                        self.castling.disable_white_castling_rights();
                    }
                    Color::Black => {
                        self.clear_piece(&BLACK_ROOK, &FileRank::H8);
                        self.set_piece(&BLACK_ROOK, &FileRank::F8);
                        self.castling.disable_black_castling_rights();
                    }
                }
            }
            MoveType::CastleQueenSide => {
                self.set_and_clear(&piece_move);
                match piece_move.piece.color {
                    Color::White => {
                        self.clear_piece(&WHITE_ROOK, &FileRank::A1);
                        self.set_piece(&WHITE_ROOK, &FileRank::D1);
                        self.castling.disable_white_castling_rights();
                    }
                    Color::Black => {
                        self.clear_piece(&BLACK_ROOK, &FileRank::A8);
                        self.set_piece(&BLACK_ROOK, &FileRank::D8);
                        self.castling.disable_black_castling_rights();
                    }
                }
            }
            MoveType::DoublePush(en_passant_option) => {
                self.move_piece(piece_move);
                self.en_passant = en_passant_option;
                if en_passant_option.is_some() {
                    en_passant_is_updated = true;
                }
                self.halfmove_clock.reset()
            }
            MoveType::Quite => {
                if piece_move.piece.piece_type == PieceType::Pawn {
                    self.halfmove_clock.reset()
                }
                self.check_rook_move(piece_move);
                self.move_piece(piece_move)
            }
            _ => {
                eprint!("Unrecognized move {:?}", piece_move)
            }
        }
        if !en_passant_is_updated {
            self.en_passant = None;
        }

        self.change_current_turn();
    }

    fn change_current_turn(&mut self) {
        self.turn = self.turn.flip();
    }

    fn check_rook_move(&mut self, piece_move: &PieceMove) {
        if piece_move.piece.piece_type == PieceType::Rook {
            match piece_move.piece.color {
                Color::White => {
                    if piece_move.from == FileRank::A1 && self.castling.get_queen_side(&Color::White) == true {
                        self.castling.disable_queen_side(&Color::White)
                    }
                    if piece_move.from == FileRank::H1 && self.castling.get_king_side(&Color::White) == true {
                        self.castling.disable_king_side(&Color::White)
                    }
                }
                Color::Black => {
                    if piece_move.from == FileRank::A8 && self.castling.get_queen_side(&Color::Black) == true {
                        self.castling.disable_queen_side(&Color::Black)
                    }
                    if piece_move.from == FileRank::H8 && self.castling.get_king_side(&Color::Black) == true {
                        self.castling.disable_king_side(&Color::Black);
                    }
                }
            }
        }
    }

    fn increment_full_move(&mut self) {
        if self.turn == Color::Black {
            self.fullmove_number.tick();
        }
    }

    fn move_piece(&mut self, piece_move: &PieceMove) {
        match (piece_move.piece.color, piece_move.piece.piece_type) {
            (Color::White, PieceType::King) => self.castling.disable_white_castling_rights(),
            (Color::Black, PieceType::King) => self.castling.disable_black_castling_rights(),
            (Color::White, PieceType::Rook) => match piece_move.from {
                FileRank::A1 => {
                    self.castling.disable_queen_side(&Color::White)
                }
                FileRank::H1 => {
                    self.castling.disable_king_side(&Color::White)
                }
                _ => {}
            },
            (Color::Black, PieceType::Rook) => match piece_move.from {
                FileRank::A8 => {
                    self.castling.disable_queen_side(&Color::Black)
                }
                FileRank::H8 => {
                    self.castling.disable_king_side(&Color::Black)
                }
                _ => {}
            },
            _ => {}
        }

        self.set_and_clear(piece_move);
    }

    fn set_and_clear(&mut self, piece_move: &PieceMove) {
        self.clear_piece(&piece_move.piece, &piece_move.from);
        self.set_piece(&piece_move.piece, &piece_move.target);
    }

    fn capture_with_promotion(&mut self, piece_move: &PieceMove, promotion: &PieceType) {
        self.handle_capture(piece_move);
        self.promote(piece_move, promotion);
    }

    fn promote(&mut self, piece_move: &PieceMove, promotion: &PieceType) {
        self.halfmove_clock.reset();
        let new_piece = &Piece {
            piece_type: *promotion,
            color: piece_move.piece.color.clone(),
        };
        self.clear_piece(&piece_move.piece, &piece_move.from);
        self.set_piece(new_piece, &piece_move.target);
    }

    fn handle_capture(&mut self, piece_move: &PieceMove) {
        self.halfmove_clock.reset();

        if let Some(target_piece) = self.get_piece_at(&piece_move.target) {
            self.clear_piece(&target_piece, &piece_move.target);
        } else {
            if piece_move.piece.piece_type == PieceType::Pawn {
                if let Some(en_passant_file_rank) = self.en_passant {
                    if en_passant_file_rank == piece_move.target {
                        let file_rank_mask = en_passant_file_rank.mask();
                        let target_file_rank = if (file_rank_mask & RANK_3) > 0 {
                            FileRank::get_from_mask(file_rank_mask >> 8).unwrap()
                        } else {
                            FileRank::get_from_mask(file_rank_mask << 8).unwrap()
                        };
                        self.clear_piece(
                            &Piece {
                                color: self.turn.flip(),
                                piece_type: PieceType::Pawn,
                            },
                            &target_file_rank,
                        )
                    }
                }
            }
        }
    }

    pub fn calculate_pseudolegal_moves(&mut self) {
        let db = &self.move_lookup_table;
        let white = self.get_board_side_info(&Color::White);
        let black = self.get_board_side_info(&Color::Black);

        let (w_mask, w_flat_moves) = self.get_pseudolegal_moves(&white);
        let (b_mask, b_flat_moves) = self.get_pseudolegal_moves(&black);

        self.b_moves_mask = b_mask;
        self.w_moves_mask = w_mask;

        self.flat_white_moves = w_flat_moves;
        self.flat_black_moves = b_flat_moves;

        let side = &self.get_board_side_info(&self.turn);

        if let Some(castlings) = self.get_castling_moves(side) {
            for ele in castlings {
                if Color::White == self.turn {
                    self.flat_white_moves.push(ele);
                    self.w_moves_mask |= ele.target.mask();
                } else {
                    self.flat_black_moves.push(ele);
                    self.b_moves_mask |= ele.target.mask();
                }
            }
        }
    }

    pub fn detect_check(&self, king_mask: &u64, attacks_mask: &u64) -> bool {
        let mask = king_mask & attacks_mask;
        mask > 0
    }

    pub fn get_valid_moves(&self) -> Vec<&PieceMove> {
        let moves = if self.turn == Color::White {
            &self.flat_white_moves
        } else {
            &self.flat_black_moves
        };

        let valid_attacks: Vec<&PieceMove> = moves
            .iter()
            .map(|piece_move: &PieceMove| {
                let mut game: GameState = self.clone();
                game.apply(piece_move);
                game.calculate_pseudolegal_moves();
                let BoardSide {
                    king,
                    opposite_attacks,
                    ..
                } = game.get_board_side_info(&game.turn.flip());

                let check = game.detect_check(&king, &opposite_attacks);
                (check, piece_move)
            })
            .filter(|attack| attack.0 == false)
            .map(|tuple| tuple.1)
            .collect();
        valid_attacks
    }

    pub fn get_pseudolegal_moves(&self, side: &BoardSide) -> (u64, Vec<PieceMove>) {
        let BoardSide {
            mut bishops,
            mut friendly_blockers,
            mut opposite_blockers,
            mut king,
            mut queens,
            mut rooks,
            mut pawns,
            mut knights,
            color,
            ..
        } = side;
        let color = color.clone();
        let lookup_table = self.move_lookup_table.clone();
        let rev_friendly_blockers = !friendly_blockers;

        let mut flat_moves: Vec<PieceMove> = Vec::with_capacity(50);
        let mut move_mask = 0u64;

        for rook_position in get_file_ranks(rooks) {
            let mut rook_move: u64 =
                lookup_table.get_rook_attack(rook_position, self) & rev_friendly_blockers;
            move_mask |= rook_move;
            fill_moves(
                rook_position,
                Piece::from(&PieceType::Rook, &color),
                rook_move,
                &mut flat_moves,
                opposite_blockers,
            );
        }
        for bishop_position in get_file_ranks(bishops) {
            let bishop_moves: u64 =
                lookup_table.get_bishop_attack(bishop_position, self) & rev_friendly_blockers;
            move_mask |= bishop_moves;

            fill_moves(
                bishop_position,
                Piece::from(&PieceType::Bishop, &color),
                bishop_moves,
                &mut flat_moves,
                opposite_blockers,
            );
        }

        for queen_position in get_file_ranks(queens) {
            let bishop_moves: u64 = lookup_table.get_bishop_attack(queen_position, &self);
            let rook_moves: u64 = lookup_table.get_rook_attack(queen_position, &self);
            let sliding_moves = (bishop_moves | rook_moves) & rev_friendly_blockers;
            move_mask |= sliding_moves;
            fill_moves(
                queen_position,
                Piece::from(&PieceType::Queen, &color),
                sliding_moves,
                &mut flat_moves,
                opposite_blockers,
            );
        }

        for knight_position in get_file_ranks(knights) {
            let attacks = get_knight_attacks(knight_position) & rev_friendly_blockers;
            move_mask |= attacks;
            fill_moves(
                knight_position,
                Piece::from(&PieceType::Knight, &color),
                attacks,
                &mut flat_moves,
                opposite_blockers,
            );
        }

        get_pawn_moves(
            side.color,
            side.pawns,
            self.empty_square(),
            side.opposite_blockers,
            &self.en_passant,
            &mut move_mask,
            &mut flat_moves,
        );

        for king_position in get_file_ranks(king) {
            let i = king_position.index();
            let attacks = get_king_attacks(king_position) & rev_friendly_blockers;
            move_mask |= attacks;
            fill_moves(
                king_position,
                Piece::from(&PieceType::King, &color),
                attacks,
                &mut flat_moves,
                opposite_blockers,
            );
        }

        (move_mask, flat_moves)
    }

    fn get_castling_moves(&self, side: &BoardSide) -> Option<Vec<PieceMove>> {
        let BoardSide {
            king,
            rooks,
            castling_king_side,
            castling_queen_side,
            opposite_attacks,
            king_mask_castling,
            queen_mask_castling,
            color,
            friendly_blockers,
            opposite_blockers,
            ..
        } = side;

        let blockers = opposite_blockers | friendly_blockers;

        let has_check = self.detect_check(&king, &opposite_attacks);
        if has_check || !(castling_king_side | castling_queen_side) {
            return None;
        }

        let mut castlings: Vec<PieceMove> = Vec::with_capacity(2);
        let king_piece = Piece {
            piece_type: PieceType::King,
            color: *color,
        };
        let starting_king_file_rank = if *color == Color::White {
            FileRank::E1
        } else {
            FileRank::E8
        };
        let king_is_on_start_field = starting_king_file_rank.mask() & king > 0;

        if king_is_on_start_field == false {
            return None;
        }

        let targets = if *color == Color::White {
            [FileRank::C1, FileRank::G1]
        } else {
            [FileRank::C8, FileRank::G8]
        };
        let rooks_starting_field = if *color == Color::White {
            [FileRank::A1, FileRank::H1]
        } else {
            [FileRank::A8, FileRank::H8]
        };
        let blockers_without_king = blockers & (!king);
        let king_side_free_from_attack = (opposite_attacks & king_mask_castling) == 0;
        let has_space_king_side = (blockers_without_king & king_mask_castling) == 0;

        if *castling_king_side
            && king_side_free_from_attack
            && has_space_king_side
            && rooks_starting_field[1].mask() & rooks > 0
        {
            castlings.push({
                PieceMove {
                    piece: king_piece,
                    from: starting_king_file_rank,
                    target: targets[1],
                    move_type: MoveType::CastleKingSide,
                }
            })
        }

        let required_space_mask = if *color == Color::White {
            queen_mask_castling | (1u64 << 57)
        } else {
            queen_mask_castling | (1u64 << 1)
        };
        let queen_side_free_from_attack = (opposite_attacks & queen_mask_castling) == 0;
        let has_space_queen_side = (blockers_without_king & required_space_mask) == 0;
        if *castling_queen_side
            && queen_side_free_from_attack
            && has_space_queen_side
            && (rooks_starting_field[0].mask() & rooks) > 0
        {
            castlings.push({
                PieceMove {
                    piece: king_piece,
                    from: starting_king_file_rank,
                    target: targets[0],
                    move_type: MoveType::CastleQueenSide,
                }
            })
        }

        if castlings.len() > 0 {
            return Some(castlings);
        }

        return None;
    }

    pub fn new_game() -> GameState {
        GameState::deserialize("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn set_piece(&mut self, piece: &Piece, file_rank: &FileRank) {
        let mut bitboard = self.get_piece_bitboard(piece, file_rank);
        set_bit(&mut bitboard, &file_rank);
    }

    pub fn clear_piece(&mut self, piece: &Piece, file_rank: &FileRank) {
        {
            let mut bitboard = self.get_piece_bitboard(piece, file_rank);
            clear_bit(&mut bitboard, file_rank);
        }
    }

    fn get_piece_bitboard(&mut self, piece: &Piece, file_rank: &FileRank) -> &mut u64 {
        return &mut self.bitboard[piece.bitboard_index()];
    }

    pub fn get_all_pieces(&self) -> u64 {
        return (self.get_white_pieces()) | (self.get_black_pieces());
    }

    pub fn get_white_pieces(&self) -> u64 {
        let result = self.bitboard[0..6]
            .iter()
            .copied()
            .reduce(|acc, b| acc | b)
            .unwrap_or(0u64);
        result
    }

    pub fn get_black_pieces(&self) -> u64 {
        let result = self.bitboard[6..12]
            .iter()
            .copied()
            .reduce(|acc, b| acc | b)
            .unwrap_or(0u64);
        result
    }

    pub fn empty_square(&self) -> u64 {
        return !self.get_all_pieces();
    }

    pub fn has(bit_board: u64, file_rank: &FileRank) -> bool {
        let file_rank_num = (*file_rank) as u8;
        let mask = 1u64 << file_rank_num;
        return (bit_board & mask) != 0;
    }

    pub fn get_by_index(bit_board: u64, index: u8) -> bool {
        let mask: u64 = 1u64 << index;
        return (bit_board & mask) != 0;
    }

    pub fn print(&self) {
        println!("  a b c d e f g h");
        println!(" +----------------+");

        FileRank::iter().for_each(|file_rank| {
            let row = 7 - file_rank.rank();
            let col = file_rank.file();

            if col == 0 {
                if row != 7 {
                    println!("|{}", row + 2);
                }
                print!("{}|", row + 1);
            }

            let piece = self.get_piece_at(&file_rank);

            if let Some(p) = piece {
                print!("{} ", p.piece_symbol());
            } else {
                print!(". ");
            }
        });

        println!("|1");
        println!(" +----------------+");
        println!("  a b c d e f g h");
        println!("fen: {}", GameState::serialize(&self))
    }

    pub fn get_piece_at(&self, file_rank: &FileRank) -> Option<Piece> {
        for piece in PIECES_ARRAY {
            if GameState::has(self.bitboard[piece.bitboard_index()], file_rank) {
                return Some(piece);
            }
        }
        None
    }

    pub fn get_board_side_info(&self, color: &Color) -> BoardSide {
        if *color == Color::White {
            BoardSide {
                bishops: self.bitboard[PieceIndex::B.idx()],
                king: self.bitboard[PieceIndex::K.idx()],
                knights: self.bitboard[PieceIndex::N.idx()],
                pawns: self.bitboard[PieceIndex::P.idx()],
                queens: self.bitboard[PieceIndex::Q.idx()],
                rooks: self.bitboard[PieceIndex::R.idx()],

                opposite_bishops: self.bitboard[PieceIndex::b.idx()],
                opposite_king: self.bitboard[PieceIndex::k.idx()],
                opposite_knights: self.bitboard[PieceIndex::n.idx()],
                opposite_pawns: self.bitboard[PieceIndex::p.idx()],
                opposite_queens: self.bitboard[PieceIndex::q.idx()],
                opposite_rooks: self.bitboard[PieceIndex::r.idx()],
                opposite_attacks: self.b_moves_mask,

                opposite_blockers: self.get_black_pieces(),
                friendly_blockers: self.get_white_pieces(),

                castling_king_side: self.castling.get_king_side(&Color::White),
                castling_queen_side: self.castling.get_queen_side(&Color::White),
                king_mask_castling: WHITE_KING_CASTLE_MASK,
                queen_mask_castling: WHITE_QUEEN_CASTLE_MASK,
                color: Color::White,
            }
        } else {
            BoardSide {
                bishops: self.bitboard[PieceIndex::b.idx()],
                king: self.bitboard[PieceIndex::k.idx()],
                knights: self.bitboard[PieceIndex::n.idx()],
                pawns: self.bitboard[PieceIndex::p.idx()],
                queens: self.bitboard[PieceIndex::q.idx()],
                rooks: self.bitboard[PieceIndex::r.idx()],

                opposite_bishops: self.bitboard[PieceIndex::B.idx()],
                opposite_king: self.bitboard[PieceIndex::K.idx()],
                opposite_knights: self.bitboard[PieceIndex::N.idx()],
                opposite_pawns: self.bitboard[PieceIndex::P.idx()],
                opposite_queens: self.bitboard[PieceIndex::Q.idx()],
                opposite_rooks: self.bitboard[PieceIndex::R.idx()],
                opposite_attacks: self.w_moves_mask,

                opposite_blockers: self.get_white_pieces(),
                friendly_blockers: self.get_black_pieces(),

                castling_king_side: self.castling.get_king_side(&Color::Black),
                castling_queen_side: self.castling.get_queen_side(&Color::Black),

                king_mask_castling: BLACK_KING_CASTLE_MASK,
                queen_mask_castling: BLACK_QUEEN_CASTLE_MASK,
                color: Color::Black,
            }
        }
    }
}

impl FenParser for GameState {
    fn deserialize(fen: &str) -> GameState {
        let mut parts = fen.split_whitespace();
        let piece_placement = parts.next().unwrap_or("");
        let active_color = parts.next().unwrap_or("");
        let castling_rights = parts.next().unwrap_or("");
        let en_passant = parts.next().unwrap_or("");
        let halfmove_clock = parts.next().unwrap_or("");
        let fullmove_number = parts.next().unwrap_or("");

        let mut game = GameState::empty();
        let mut row: u8 = 0;
        let mut col: u8 = 0;

        for char in piece_placement.chars() {
            if let Some(piece) = get_piece_from_char(char) {
                let index = (row * 8) + col;
                if let Some(rank_file) = FileRank::get_file_rank(index) {
                    game.set_piece(piece, &rank_file);
                }
                col += 1;
            } else {
                match char {
                    '/' => {
                        row += 1;
                        col = 0;
                    }
                    '1'..='8' => {
                        if let Some(offset) = char.to_digit(10) {
                            col += offset as u8;
                        }
                    }
                    _ => {}
                }
            }
        }

        game.turn = match active_color {
            "w" => Color::White,
            "b" => Color::Black,
            _ => game.turn, // default value
        };

        let mut castling = Castling::new();
        for char in castling_rights.chars() {
            match char {
                'K' => castling.mask |= WHITE_KING_CASTLE_MASK,
                'Q' => castling.mask |= WHITE_QUEEN_CASTLE_MASK,
                'k' => castling.mask |= BLACK_KING_CASTLE_MASK,
                'q' => castling.mask |= BLACK_QUEEN_CASTLE_MASK,
                _ => {}
            }
        }

        game.halfmove_clock = Clock::from_string(halfmove_clock);
        game.fullmove_number = Clock::from_string(fullmove_number);
        game.castling = castling;
        game.en_passant = FileRank::from_string(en_passant);

        game
    }

    fn serialize(&self) -> String {
        let mut piece_placement = String::new();
        let mut row_pieces = vec![0u8; 8];
        let mut active_color = if self.turn == Color::White { "w" } else { "b" };
        let mut castling_rights = String::new();
        let mut en_passant = String::from("-");
        let halfmove_clock = self.halfmove_clock.to_string();
        let fullmove_number = self.fullmove_number.to_string();

        for rank in 0..8 {
            let mut empty_count = 0;
            for file in 0..8 {
                let index = rank * 8 + file;
                let file_rank = FileRank::get_file_rank(index).unwrap();
                if let Some(piece) = self.get_piece_at(&file_rank) {
                    if empty_count > 0 {
                        piece_placement.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    piece_placement.push(piece.symbol());
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                piece_placement.push_str(&empty_count.to_string());
            }
            if rank < 7 {
                piece_placement.push('/');
            }
        }

        if self.castling.get_king_side(&Color::White) {
            castling_rights.push('K');
        }
        if self.castling.get_queen_side(&Color::White) {
            castling_rights.push('Q');
        }
        if self.castling.get_king_side(&Color::Black) {
            castling_rights.push('k');
        }
        if self.castling.get_queen_side(&Color::Black) {
            castling_rights.push('q');
        }
        if castling_rights.is_empty() {
            castling_rights.push('-');
        }

        if let Some(en_passant_square) = self.en_passant {
            en_passant = en_passant_square.to_string();
        }

        let fen_string = format!(
            "{} {} {} {} {} {}",
            piece_placement,
            active_color,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number
        );

        fen_string
    }
}

impl Default for GameState {
    fn default() -> Self {
        let move_lookup_table = Arc::new(MoveLookupTable::init());
        let zobrist_hashing = ZobristHashing::new();
        Self {
            move_lookup_table,
            bitboard: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            turn: Color::White,
            castling: Castling::new(),
            en_passant: None,
            halfmove_clock: Clock::new(),
            fullmove_number: Clock::new(),
            flat_black_moves: Vec::new(),
            flat_white_moves: Vec::new(),
            w_moves_mask: 0u64,
            b_moves_mask: 0u64,
            zobrssist_hashing: Arc::new(zobrist_hashing)
        }
    }
}
