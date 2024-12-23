use std::str::Chars;

use crate::types::AlgebraicNotationToken;

pub struct AlgebraicNotation<'a> {
    pub(crate) input: Chars<'a>,
    pub(crate) castling_symbol: char,
}

impl<'a> AlgebraicNotation<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars(),
            castling_symbol: 'O',
        }
    }

    pub fn next_token(&mut self) -> Option<AlgebraicNotationToken> {
        let _castling_symbol = self.castling_symbol;
        while let Some(c) = self.input.next() {
            match c {
                'K' | 'Q' | 'R' | 'B' | 'N' | 'P' => return Some(AlgebraicNotationToken::Piece(c)),
                'a'..='h' => return Some(AlgebraicNotationToken::File(c)),
                '1'..='8' => return Some(AlgebraicNotationToken::Rank(c)),
                'x' => return Some(AlgebraicNotationToken::Capture),
                '+' => return Some(AlgebraicNotationToken::Check),
                '#' => return Some(AlgebraicNotationToken::Checkmate),
                'O' | '0' => {
                    return self.parse_castling();
                },
                '=' => {
                    if let Some(promotion_piece) = self.input.next() {
                        return Some(AlgebraicNotationToken::Promotion(promotion_piece));
                    }
                },
                '-' => return Some(AlgebraicNotationToken::MoveIndicator),
                _ => {}
            }
        }
        None
    }

    pub(crate) fn parse_castling(&mut self) -> Option<AlgebraicNotationToken> {
        let symbol = self.castling_symbol;

        let king_castling_pattern = format!("-{}", symbol);
        let queen_castling_pattern = format!("-{}-{}", symbol, symbol);

        let peek_input: String = self.input.clone().take(4).collect::<String>();

        if peek_input.starts_with(&queen_castling_pattern) {
            self.consume_pattern(&queen_castling_pattern);
            return Some(AlgebraicNotationToken::CastleQueenSide);
        }

        if peek_input.starts_with(&king_castling_pattern) {
            self.consume_pattern(&king_castling_pattern);
            return Some(AlgebraicNotationToken::CastleKingSide);
        }

        None
    }
    pub(crate) fn consume_pattern(&mut self, pattern: &str) {
        for _ in pattern.chars() {
            self.input.next();
        }
    }
}
