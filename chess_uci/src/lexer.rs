#[derive(Debug, Clone, Copy)]
pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn from(content: &'a [char]) -> Lexer<'a> {
        return Lexer { content: content };
    }

    fn chop(&mut self, n: usize) -> String {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        return token.iter().collect();
    }

    pub fn chop_fn<Predicate>(&mut self, mut predicate: Predicate) -> String
    where
        Predicate: FnMut(&char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }
        if n == 0 {
            n += 1;
        }

        self.chop(n)
    }

    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..]
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.trim_left();
        while !self.content.is_empty() && self.content.len() > 0 {
            self.trim_left();
            return Some(
                self.chop_fn(|c| (c.is_alphabetic() || c.is_alphanumeric() || *c == '-') && *c != '\n'),
            );
        }

        return None;
    }
}
