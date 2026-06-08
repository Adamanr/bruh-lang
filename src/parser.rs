use crate::ast::{LoopCond, Stmt, Var};
use crate::error::BruhError;
use crate::token::{Spanned, Token};

pub enum RawStmt {
    Stmt(Stmt, usize),
    Sound(LoopCond, usize),
    Effect(usize),
}

fn as_var(tok: &Token) -> Option<Var> {
    match tok {
        Token::VarA => Some(Var::A),
        Token::VarB => Some(Var::B),
        Token::VarC => Some(Var::C),
        _ => None,
    }
}

fn parse_line(toks: &[Spanned]) -> Result<RawStmt, BruhError> {
    let line = toks[0].pos.line;

    macro_rules! err {
        ($code:expr, $msg:expr) => {
            return Err(BruhError { line, code: $code, msg: $msg })
        };
    }

    match &toks[0].tok {
        Token::Sound => {
            if toks.len() < 3 {
                err!("MalformedStatement", format!("expected: sound <var> [momentum|moment] <var> at line {line}"));
            }
            let lhs = as_var(&toks[1].tok)
                .ok_or_else(|| BruhError { line, code: "MalformedStatement", msg: format!("expected variable after 'sound' at line {line}") })?;

            if toks.len() == 3 {
                let rhs = as_var(&toks[2].tok)
                    .ok_or_else(|| BruhError { line, code: "MalformedStatement", msg: format!("expected variable at line {line}") })?;
                return Ok(RawStmt::Sound(LoopCond::NotEqual(lhs, rhs), line));
            }

            if toks.len() == 4 {
                let rhs = as_var(&toks[3].tok)
                    .ok_or_else(|| BruhError { line, code: "MalformedStatement", msg: format!("expected variable at line {line}") })?;
                let cond = match &toks[2].tok {
                    Token::Momentum => LoopCond::Greater(lhs, rhs),
                    Token::Moment => LoopCond::Less(lhs, rhs),
                    _ => err!("MalformedStatement", format!("expected 'momentum' or 'moment' at line {line}")),
                };
                return Ok(RawStmt::Sound(cond, line));
            }

            err!("TrailingTokens", format!("trailing tokens after loop header at line {line}"));
        }

        Token::Effect => {
            if toks.len() > 1 {
                err!("TrailingTokens", format!("trailing tokens after 'effect' at line {line}"));
            }
            Ok(RawStmt::Effect(line))
        }

        Token::Momentum => {
            if toks.len() < 2 {
                err!("MalformedStatement", format!("expected variable after 'momentum' at line {line}"));
            }
            if toks.len() > 2 {
                err!("TrailingTokens", format!("trailing tokens at line {line}"));
            }
            let v = as_var(&toks[1].tok)
                .ok_or_else(|| BruhError { line, code: "MalformedStatement", msg: format!("expected variable at line {line}") })?;
            Ok(RawStmt::Stmt(Stmt::PrintChar(v), line))
        }

        Token::Moment => {
            if toks.len() < 2 {
                err!("MalformedStatement", format!("expected variable after 'moment' at line {line}"));
            }
            if toks.len() > 2 {
                err!("TrailingTokens", format!("trailing tokens at line {line}"));
            }
            let v = as_var(&toks[1].tok)
                .ok_or_else(|| BruhError { line, code: "MalformedStatement", msg: format!("expected variable at line {line}") })?;
            Ok(RawStmt::Stmt(Stmt::PrintInt(v), line))
        }

        tok if as_var(tok).is_some() => {
            let dst = as_var(tok).unwrap();

            if toks.len() < 2 {
                err!("MalformedStatement", format!("incomplete statement at line {line}"));
            }

            match &toks[1].tok {
                Token::VarA | Token::VarB | Token::VarC => {
                    if toks.len() > 2 {
                        err!("TrailingTokens", format!("trailing tokens at line {line}"));
                    }
                    let src = as_var(&toks[1].tok).unwrap();
                    Ok(RawStmt::Stmt(Stmt::Assign { dst, src }, line))
                }

                Token::Momentum => {
                    if toks.len() == 2 {
                        Ok(RawStmt::Stmt(Stmt::IncBy1(dst), line))
                    } else if toks.len() == 3 {
                        let src = as_var(&toks[2].tok)
                            .ok_or_else(|| BruhError { line, code: "MalformedStatement", msg: format!("expected variable at line {line}") })?;
                        Ok(RawStmt::Stmt(Stmt::IncByVar(dst, src), line))
                    } else {
                        err!("TrailingTokens", format!("trailing tokens at line {line}"));
                    }
                }

                Token::Moment => {
                    if toks.len() == 2 {
                        Ok(RawStmt::Stmt(Stmt::DecBy1(dst), line))
                    } else if toks.len() == 3 {
                        let src = as_var(&toks[2].tok)
                            .ok_or_else(|| BruhError { line, code: "MalformedStatement", msg: format!("expected variable at line {line}") })?;
                        Ok(RawStmt::Stmt(Stmt::DecByVar(dst, src), line))
                    } else {
                        err!("TrailingTokens", format!("trailing tokens at line {line}"));
                    }
                }

                _ => err!("MalformedStatement", format!("unexpected token at line {line}")),
            }
        }

        _ => err!("MalformedStatement", format!("unexpected token at start of line {line}")),
    }
}

pub fn parse(lines: &[Vec<Spanned>]) -> (Vec<RawStmt>, Vec<BruhError>) {
    let mut raw = Vec::new();
    let mut errors = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        match parse_line(line) {
            Ok(s) => raw.push(s),
            Err(e) => errors.push(e),
        }
    }

    (raw, errors)
}
