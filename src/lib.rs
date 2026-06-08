pub mod ast;
pub mod codegen;
pub mod error;
pub mod interp;
pub mod lexer;
pub mod parser;
pub mod sema;
pub mod token;

#[derive(Debug)]
pub struct CompileResult {
    pub stmts: Vec<ast::Stmt>,
}

pub fn compile_src(src: &str) -> Result<CompileResult, Vec<(usize, &'static str, String)>> {
    let (token_lines, lex_errors) = lexer::lex(src);
    let (raw, parse_errors) = parser::parse(&token_lines);
    let (stmts, sema_errors) = sema::analyze(raw);

    let mut all: Vec<(usize, &'static str, String)> = Vec::new();

    for e in &lex_errors {
        all.push((
            e.line,
            "UnknownToken",
            format!("unknown token '{}' at column {}", e.raw, e.col),
        ));
    }
    for e in &parse_errors {
        all.push((e.line, e.code, e.msg.clone()));
    }
    for e in &sema_errors {
        all.push((e.line, e.code, e.msg.clone()));
    }

    if !all.is_empty() {
        all.sort_by_key(|(l, _, _)| *l);
        return Err(all);
    }

    Ok(CompileResult { stmts })
}
