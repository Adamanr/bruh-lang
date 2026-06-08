use crate::ast::{LoopCond, Stmt, Var};
use std::io::{self, Write};

pub struct Interpreter {
    vars: [i64; 3],
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self { vars: [0; 3] }
    }

    fn get(&self, v: Var) -> i64 {
        self.vars[v as usize]
    }

    fn set(&mut self, v: Var, val: i64) {
        self.vars[v as usize] = val;
    }

    pub fn run(&mut self, stmts: &[Stmt]) -> io::Result<()> {
        let stdout = io::stdout();
        let mut out = stdout.lock();
        self.exec(stmts, &mut out)
    }

    pub fn run_to<W: Write>(&mut self, stmts: &[Stmt], out: &mut W) -> io::Result<()> {
        self.exec(stmts, out)
    }

    fn exec<W: Write>(&mut self, stmts: &[Stmt], out: &mut W) -> io::Result<()> {
        for stmt in stmts {
            self.exec_stmt(stmt, out)?;
        }
        Ok(())
    }

    fn exec_stmt<W: Write>(&mut self, stmt: &Stmt, out: &mut W) -> io::Result<()> {
        match stmt {
            Stmt::IncBy1(v) => {
                let val = self.get(*v).wrapping_add(1);
                self.set(*v, val);
            }
            Stmt::DecBy1(v) => {
                let val = self.get(*v).wrapping_sub(1);
                self.set(*v, val);
            }
            Stmt::IncByVar(dst, src) => {
                let val = self.get(*dst).wrapping_add(self.get(*src));
                self.set(*dst, val);
            }
            Stmt::DecByVar(dst, src) => {
                let val = self.get(*dst).wrapping_sub(self.get(*src));
                self.set(*dst, val);
            }
            Stmt::Assign { dst, src } => {
                let val = self.get(*src);
                self.set(*dst, val);
            }
            Stmt::PrintInt(v) => {
                write!(out, "{}", self.get(*v))?;
            }
            Stmt::PrintChar(v) => {
                let byte = (self.get(*v) & 0xFF) as u8;
                out.write_all(&[byte])?;
            }
            Stmt::Loop { cond, body, .. } => loop {
                let cont = match cond {
                    LoopCond::NotEqual(a, b) => self.get(*a) != self.get(*b),
                    LoopCond::Greater(a, b) => self.get(*a) > self.get(*b),
                    LoopCond::Less(a, b) => self.get(*a) < self.get(*b),
                };
                if !cont {
                    break;
                }
                self.exec(body, out)?;
            },
        }
        Ok(())
    }
}
