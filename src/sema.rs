use crate::ast::{LoopCond, Stmt};
use crate::error::BruhError;
use crate::parser::RawStmt;

pub fn analyze(raw: Vec<RawStmt>) -> (Vec<Stmt>, Vec<BruhError>) {
    let mut stmts: Vec<Stmt> = Vec::new();
    let mut errors: Vec<BruhError> = Vec::new();

    let mut in_loop = false;
    let mut loop_line = 0usize;
    let mut loop_cond: Option<LoopCond> = None;
    let mut loop_body: Vec<Stmt> = Vec::new();

    for item in raw {
        match item {
            RawStmt::Sound(cond, line) => {
                if in_loop {
                    errors.push(BruhError {
                        line,
                        code: "NestedLoop",
                        msg: format!("nested loop at line {line}"),
                    });
                } else {
                    in_loop = true;
                    loop_line = line;
                    loop_cond = Some(cond);
                    loop_body = Vec::new();
                }
            }

            RawStmt::Effect(line) => {
                if !in_loop {
                    errors.push(BruhError {
                        line,
                        code: "UnmatchedEffect",
                        msg: format!("'effect' without matching 'sound' at line {line}"),
                    });
                } else {
                    in_loop = false;
                    let cond = loop_cond.take().unwrap();
                    let body = std::mem::take(&mut loop_body);
                    stmts.push(Stmt::Loop {
                        cond,
                        body,
                        line: loop_line,
                    });
                }
            }

            RawStmt::Stmt(stmt, _) => {
                if in_loop {
                    loop_body.push(stmt);
                } else {
                    stmts.push(stmt);
                }
            }
        }
    }

    if in_loop {
        errors.push(BruhError {
            line: loop_line,
            code: "UnterminatedLoop",
            msg: format!("loop opened at line {loop_line} is never closed"),
        });
    }

    (stmts, errors)
}
