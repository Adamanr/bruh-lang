use crate::ast::{LoopCond, Stmt, Var};

// Memory layout:
//   [0..31]  number string buffer (written right-to-left by print_i64)
//   [32..35] iovec.ptr  (i32)
//   [36..39] iovec.len  (i32)
//   [40..43] nwritten   (i32)

const PRINT_I64_FUNC: &str = r#"  (func $print_i64 (param $val i64)
    (local $neg i32)
    (local $uval i64)
    (local $pos i32)
    (local $digit i64)
    (if (i64.eqz (local.get $val))
      (then
        (i32.store8 (i32.const 31) (i32.const 48))
        (i32.store (i32.const 32) (i32.const 31))
        (i32.store (i32.const 36) (i32.const 1))
        (drop (call $fd_write (i32.const 1) (i32.const 32) (i32.const 1) (i32.const 40)))
        return
      )
    )
    (local.set $neg (i64.lt_s (local.get $val) (i64.const 0)))
    (if (local.get $neg)
      (then (local.set $uval (i64.sub (i64.const 0) (local.get $val))))
      (else (local.set $uval (local.get $val)))
    )
    (local.set $pos (i32.const 31))
    (block $brk
      (loop $top
        (br_if $brk (i64.eqz (local.get $uval)))
        (local.set $digit (i64.rem_u (local.get $uval) (i64.const 10)))
        (i32.store8
          (local.get $pos)
          (i32.add (i32.const 48) (i32.wrap_i64 (local.get $digit)))
        )
        (local.set $pos (i32.sub (local.get $pos) (i32.const 1)))
        (local.set $uval (i64.div_u (local.get $uval) (i64.const 10)))
        (br $top)
      )
    )
    (if (local.get $neg)
      (then
        (i32.store8 (local.get $pos) (i32.const 45))
        (local.set $pos (i32.sub (local.get $pos) (i32.const 1)))
      )
    )
    (i32.store (i32.const 32) (i32.add (local.get $pos) (i32.const 1)))
    (i32.store (i32.const 36) (i32.sub (i32.const 31) (local.get $pos)))
    (drop (call $fd_write (i32.const 1) (i32.const 32) (i32.const 1) (i32.const 40)))
  )
"#;

fn var_name(v: Var) -> &'static str {
    match v {
        Var::A => "$A",
        Var::B => "$B",
        Var::C => "$C",
    }
}

fn emit_stmts(stmts: &[Stmt], out: &mut String, depth: usize, loop_idx: &mut usize) {
    for s in stmts {
        emit_stmt(s, out, depth, loop_idx);
    }
}

fn emit_stmt(stmt: &Stmt, out: &mut String, depth: usize, loop_idx: &mut usize) {
    let pad = "  ".repeat(depth);
    match stmt {
        Stmt::IncBy1(v) => {
            let v = var_name(*v);
            out.push_str(&format!(
                "{pad}(global.set {v} (i64.add (global.get {v}) (i64.const 1)))\n"
            ));
        }
        Stmt::DecBy1(v) => {
            let v = var_name(*v);
            out.push_str(&format!(
                "{pad}(global.set {v} (i64.sub (global.get {v}) (i64.const 1)))\n"
            ));
        }
        Stmt::IncByVar(dst, src) => {
            let d = var_name(*dst);
            let s = var_name(*src);
            out.push_str(&format!(
                "{pad}(global.set {d} (i64.add (global.get {d}) (global.get {s})))\n"
            ));
        }
        Stmt::DecByVar(dst, src) => {
            let d = var_name(*dst);
            let s = var_name(*src);
            out.push_str(&format!(
                "{pad}(global.set {d} (i64.sub (global.get {d}) (global.get {s})))\n"
            ));
        }
        Stmt::Assign { dst, src } => {
            let d = var_name(*dst);
            let s = var_name(*src);
            out.push_str(&format!("{pad}(global.set {d} (global.get {s}))\n"));
        }
        Stmt::PrintInt(v) => {
            let v = var_name(*v);
            out.push_str(&format!("{pad}(call $print_i64 (global.get {v}))\n"));
        }
        Stmt::PrintChar(v) => {
            let v = var_name(*v);
            out.push_str(&format!(
                "{pad}(i32.store8 (i32.const 0) (i32.wrap_i64 (i64.and (global.get {v}) (i64.const 255))))\n\
                 {pad}(i32.store (i32.const 32) (i32.const 0))\n\
                 {pad}(i32.store (i32.const 36) (i32.const 1))\n\
                 {pad}(drop (call $fd_write (i32.const 1) (i32.const 32) (i32.const 1) (i32.const 40)))\n"
            ));
        }
        Stmt::Loop { cond, body, .. } => {
            let idx = *loop_idx;
            *loop_idx += 1;

            let (exit_instr, lhs, rhs) = match cond {
                LoopCond::NotEqual(a, b) => ("i64.eq", var_name(*a), var_name(*b)),
                LoopCond::Greater(a, b) => ("i64.le_s", var_name(*a), var_name(*b)),
                LoopCond::Less(a, b) => ("i64.ge_s", var_name(*a), var_name(*b)),
            };

            out.push_str(&format!("{pad}(block $exit_{idx}\n"));
            out.push_str(&format!("{pad}  (loop $top_{idx}\n"));
            out.push_str(&format!(
                "{pad}    (br_if $exit_{idx} ({exit_instr} (global.get {lhs}) (global.get {rhs})))\n"
            ));
            emit_stmts(body, out, depth + 2, loop_idx);
            out.push_str(&format!("{pad}    (br $top_{idx})\n"));
            out.push_str(&format!("{pad}  )\n"));
            out.push_str(&format!("{pad})\n"));
        }
    }
}

pub fn generate_wat(program: &[Stmt]) -> String {
    let mut wat = String::new();

    wat.push_str("(module\n");
    wat.push_str(
        "  (import \"wasi_snapshot_preview1\" \"fd_write\"\n\
             (func $fd_write (param i32 i32 i32 i32) (result i32)))\n",
    );
    wat.push_str("  (memory (export \"memory\") 1)\n");
    wat.push_str("  (global $A (mut i64) (i64.const 0))\n");
    wat.push_str("  (global $B (mut i64) (i64.const 0))\n");
    wat.push_str("  (global $C (mut i64) (i64.const 0))\n");
    wat.push_str(PRINT_I64_FUNC);
    wat.push_str("  (func $_start (export \"_start\")\n");

    let mut body = String::new();
    let mut loop_idx = 0usize;
    emit_stmts(program, &mut body, 2, &mut loop_idx);
    wat.push_str(&body);

    wat.push_str("  )\n)\n");
    wat
}

pub fn generate_wasm(program: &[Stmt]) -> Result<Vec<u8>, wat::Error> {
    let text = generate_wat(program);
    wat::parse_str(&text)
}
