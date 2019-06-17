// Location info, pair of from_offset, to_offset.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Loc(usize, usize);

// impl Loc {
//     fn merge(&self, other: &Loc) -> Loc {
//         use std::cmp::{max, min};
//         Loc(min(self.0, other.0), max(self.1, other.1))
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annot<T> {
    value: T,
    loc: Loc,
}
impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Number(u64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
}

pub type Token = Annot<TokenKind>;

// TOOD: impl by macro
impl Token {
    fn number(n: u64, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }
    fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }
    fn minus(loc: Loc) -> Self {
        Self::new(TokenKind::Minus, loc)
    }
    fn asterisk(loc: Loc) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }
    fn slash(loc: Loc) -> Self {
        Self::new(TokenKind::Slash, loc)
    }
    fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }
    fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
}

#[derive(Debug, PartialEq)]
pub enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

pub type LexError = Annot<LexErrorKind>;
impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }
    fn eof(loc: Loc) -> Self {
        LexError::new(LexErrorKind::Eof, loc)
    }
}
type LexResult<T> = Result<T, LexError>;
type LexValue = (Token, usize);
fn recognize_many(input: &[u8], pos: usize, mut f: impl FnMut(u8) -> bool) -> usize {
    let mut ret = pos;
    while ret < input.len() && f(input[ret]) {
        ret += 1;
    }
    ret
}
fn lex_number(input: &[u8], pos: usize) -> LexResult<LexValue> {
    let pattern = b"0123456789";
    let next_pos = recognize_many(input, pos, |b| pattern.contains(&b));
    let n = std::str::from_utf8(&input[pos..next_pos])
        .unwrap()
        .parse()
        .unwrap();
    let token = Token::number(n, Loc(pos, next_pos));
    Ok((token, next_pos))
}
fn skip_spaces(input: &[u8], pos: usize) -> LexResult<((), usize)> {
    let pattern = b" \n\t";
    let next_pos = recognize_many(input, pos, |b| pattern.contains(&b));
    Ok(((), next_pos))
}
fn consume_byte(input: &[u8], pos: usize, expected: u8) -> LexResult<(u8, usize)> {
    if input.len() <= pos {
        return Err(LexError::eof(Loc(pos, pos)));
    }
    if input[pos] == expected {
        Ok((input[pos], pos + 1))
    } else {
        Err(LexError::invalid_char(
            input[pos] as char,
            Loc(pos, pos + 1),
        ))
    }
}
fn lex_plus(input: &[u8], start: usize) -> LexResult<LexValue> {
    consume_byte(input, start, b'+').map(|(_, end)| (Token::plus(Loc(start, end)), end))
}
fn lex_minus(input: &[u8], start: usize) -> LexResult<LexValue> {
    consume_byte(input, start, b'-').map(|(_, end)| (Token::minus(Loc(start, end)), end))
}
fn lex_asterisk(input: &[u8], start: usize) -> LexResult<LexValue> {
    consume_byte(input, start, b'*').map(|(_, end)| (Token::asterisk(Loc(start, end)), end))
}
fn lex_slash(input: &[u8], start: usize) -> LexResult<LexValue> {
    consume_byte(input, start, b'/').map(|(_, end)| (Token::slash(Loc(start, end)), end))
}
fn lex_lparen(input: &[u8], start: usize) -> LexResult<LexValue> {
    consume_byte(input, start, b'(').map(|(_, end)| (Token::lparen(Loc(start, end)), end))
}
fn lex_rparen(input: &[u8], start: usize) -> LexResult<LexValue> {
    consume_byte(input, start, b')').map(|(_, end)| (Token::rparen(Loc(start, end)), end))
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let input = input.as_bytes();
    let mut pos = 0;
    macro_rules! lex_a_token {
        ($lexer:expr) => {{
            let (tok, p) = $lexer?;
            tokens.push(tok);
            pos = p;
        }};
    }
    while pos < input.len() {
        match input[pos] {
            b'0'...b'9' => lex_a_token!(lex_number(input, pos)),
            b'+' => lex_a_token!(lex_plus(input, pos)),
            b'-' => lex_a_token!(lex_minus(input, pos)),
            b'*' => lex_a_token!(lex_asterisk(input, pos)),
            b'/' => lex_a_token!(lex_slash(input, pos)),
            b'(' => lex_a_token!(lex_lparen(input, pos)),
            b')' => lex_a_token!(lex_rparen(input, pos)),
            b' ' | b'\n' | b'\t' => {
                let ((), p) = skip_spaces(input, pos)?;
                pos = p;
            }
            b => return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1))),
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::{lex, Loc, Token, TokenKind};
    #[test]
    fn test_lex() {
        let examples = vec![
            ("1", Ok(vec![Token::new(TokenKind::Number(1), Loc(0, 1))])),
            (
                "1 + 2 * 3 - -10",
                Ok(vec![
                    Token::number(1, Loc(0, 1)),
                    Token::plus(Loc(2, 3)),
                    Token::number(2, Loc(4, 5)),
                    Token::asterisk(Loc(6, 7)),
                    Token::number(3, Loc(8, 9)),
                    Token::minus(Loc(10, 11)),
                    Token::minus(Loc(12, 13)),
                    Token::number(10, Loc(13, 15)),
                ]),
            ),
        ];
        for (input, expected) in examples {
            let actual = lex(input);
            assert_eq!(actual, expected);
        }
    }
}
