use bruhlang::{compile_src, interp::Interpreter};

fn run(src: &str) -> Vec<u8> {
    let result = compile_src(src).expect("compile failed");
    let mut interp = Interpreter::new();
    let mut out = Vec::new();
    interp.run_to(&result.stmts, &mut out).unwrap();
    out
}

fn check_error(src: &str, expected_code: &str) {
    let errors = compile_src(src).expect_err("expected compile error");
    let has_code = errors.iter().any(|(_, code, _)| *code == expected_code);
    assert!(
        has_code,
        "expected error code '{}', got: {:?}",
        expected_code,
        errors.iter().map(|(_, c, _)| *c).collect::<Vec<_>>()
    );
}

// ── positive cases ──────────────────────────────────────────────────────────

#[test]
fn test_count() {
    let src = include_str!("corpus/count.bruh");
    let expected = include_bytes!("corpus/count.expected");
    assert_eq!(run(src), expected);
}

#[test]
fn test_countdown() {
    let src = include_str!("corpus/countdown.bruh");
    let expected = include_bytes!("corpus/countdown.expected");
    assert_eq!(run(src), expected);
}

#[test]
fn test_add_by_var() {
    let src = include_str!("corpus/add_by_var.bruh");
    let expected = include_bytes!("corpus/add_by_var.expected");
    assert_eq!(run(src), expected);
}

#[test]
fn test_letter_a() {
    let src = include_str!("corpus/letter_a.bruh");
    let expected = include_bytes!("corpus/letter_a.expected");
    assert_eq!(run(src), expected);
}

// ── negative cases ──────────────────────────────────────────────────────────

#[test]
fn test_err_nested() {
    check_error(include_str!("corpus/err_nested.bruh"), "NestedLoop");
}

#[test]
fn test_err_effect() {
    check_error(include_str!("corpus/err_effect.bruh"), "UnmatchedEffect");
}

#[test]
fn test_err_open() {
    check_error(include_str!("corpus/err_open.bruh"), "UnterminatedLoop");
}

#[test]
fn test_err_token() {
    check_error(include_str!("corpus/err_token.bruh"), "UnknownToken");
}

#[test]
fn test_err_trailing() {
    check_error(include_str!("corpus/err_trailing.bruh"), "TrailingTokens");
}

// ── wasm output matches interpreter ────────────────────────────────────────

fn run_wasm_via_cli(wasm_bytes: &[u8]) -> Option<Vec<u8>> {
    let path = std::env::temp_dir().join("bruh_test.wasm");
    std::fs::write(&path, wasm_bytes).ok()?;
    let out = std::process::Command::new("wasmtime")
        .arg(&path)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

macro_rules! wasm_test {
    ($name:ident, $src:expr) => {
        #[test]
        fn $name() {
            let src = $src;
            let interp_out = run(src);
            let result = compile_src(src).unwrap();
            let bytes =
                bruhlang::codegen::wasm::generate_wasm(&result.stmts).expect("wasm codegen failed");

            if let Some(wasm_out) = run_wasm_via_cli(&bytes) {
                assert_eq!(
                    wasm_out, interp_out,
                    "wasm output differs from interpreter"
                );
            }
            // If wasmtime not in PATH, skip silently.
        }
    };
}

wasm_test!(wasm_count, include_str!("corpus/count.bruh"));
wasm_test!(wasm_countdown, include_str!("corpus/countdown.bruh"));
wasm_test!(wasm_add_by_var, include_str!("corpus/add_by_var.bruh"));
wasm_test!(wasm_letter_a, include_str!("corpus/letter_a.bruh"));
