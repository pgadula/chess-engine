use std::str::Chars;

use crate::base_types::Token;

pub struct AlgebraicNotation<'a> {
    input: Chars<'a>,
    castling_symbol: char,
}

impl<'a> AlgebraicNotation<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars(),
            castling_symbol: 'O',
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let castling_symbol = self.castling_symbol;
        while let Some(c) = self.input.next() {
            match c {
                'K' | 'Q' | 'R' | 'B' | 'N' | 'P' => return Some(Token::Piece(c)),
                'a'..='h' => return Some(Token::File(c)),
                '1'..='8' => return Some(Token::Rank(c)),
                'x' => return Some(Token::Capture),
                '+' => return Some(Token::Check),
                '#' => return Some(Token::Checkmate),
                castling_symbol => {
                    return self.parse_castling();
                }
                '=' => {
                    if let Some(promotion_piece) = self.input.next() {
                        return Some(Token::Promotion(promotion_piece));
                    }
                }
                '-' => return Some(Token::MoveIndicator),
                _ => {}
            }
        }
        None
    }

    fn parse_castling(&mut self) -> Option<Token> {
        let symbol = self.castling_symbol;

        let king_castling_pattern = format!("-{}", symbol);
        let queen_castling_pattern = format!("-{}-{}", symbol, symbol);

        let mut peek_input = self.input.clone().take(4).collect::<String>();

        if peek_input.starts_with(&queen_castling_pattern) {
            self.consume_pattern(&queen_castling_pattern);
            return Some(Token::CastleQueenSide);
        }

        if peek_input.starts_with(&king_castling_pattern) {
            self.consume_pattern(&king_castling_pattern);
            return Some(Token::CastleKingSide);
        }

        None
    }
    fn consume_pattern(&mut self, pattern: &str) {
        for _ in pattern.chars() {
            self.input.next();
        }
    }
}
