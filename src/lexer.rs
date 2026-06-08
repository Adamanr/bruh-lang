use crate::error::LexError;
use crate::token::{Pos, Spanned, Token};

pub fn lex(src: &str) -> (Vec<Vec<Spanned>>, Vec<LexError>) {
    let mut lines = Vec::new();
    let mut errors = Vec::new();

    for (idx, line) in src.lines().enumerate() {
        let lineno = idx + 1;
        let stripped = match line.find('#') {
            Some(p) => &line[..p],
            None => line,
        };

        let mut tokens = Vec::new();
        let mut remaining = stripped;
        let mut byte_off = 0usize;

        loop {
            let leading = remaining
                .find(|c: char| !c.is_whitespace())
                .unwrap_or(remaining.len());
            byte_off += leading;
            remaining = &remaining[leading..];
            if remaining.is_empty() {
                break;
            }

            let word_len = remaining
                .find(|c: char| c.is_whitespace())
                .unwrap_or(remaining.len());
            let word = &remaining[..word_len];
            let col = byte_off + 1;

            let tok = match word {
                "bruh." => Some(Token::VarA),
                "bruh!" => Some(Token::VarB),
                "bruh?" => Some(Token::VarC),
                "momentum" => Some(Token::Momentum),
                "moment" => Some(Token::Moment),
                "sound" => Some(Token::Sound),
                "effect" => Some(Token::Effect),
                _ => None,
            };

            match tok {
                Some(t) => tokens.push(Spanned {
                    tok: t,
                    pos: Pos { line: lineno, col },
                }),
                None => errors.push(LexError {
                    line: lineno,
                    col,
                    raw: word.to_string(),
                }),
            }

            byte_off += word_len;
            remaining = &remaining[word_len..];
        }

        lines.push(tokens);
    }

    (lines, errors)
}
