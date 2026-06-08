use bruhlang::{codegen, compile_src, interp};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "bruh", about = "bruh-lang compiler and interpreter")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Interpret and run a bruh-lang program
    Run { file: PathBuf },
    /// Compile a bruh-lang program to WebAssembly
    Build {
        file: PathBuf,
        #[arg(short = 'o', long = "output")]
        output: Option<PathBuf>,
        #[arg(long = "emit", default_value = "wasm")]
        emit: EmitFormat,
    },
    /// Check syntax and semantics without running
    Check { file: PathBuf },
}

#[derive(Clone, Debug)]
enum EmitFormat {
    Wasm,
    Wat,
}

impl std::str::FromStr for EmitFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "wasm" => Ok(EmitFormat::Wasm),
            "wat" => Ok(EmitFormat::Wat),
            other => Err(format!("unknown emit format '{}', expected wasm or wat", other)),
        }
    }
}

fn read_file(path: &PathBuf) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("bruh: IO error reading {}: {}", path.display(), e);
        process::exit(2);
    })
}

fn compile(path: &PathBuf) -> Vec<bruhlang::ast::Stmt> {
    let src = read_file(path);
    compile_src(&src).unwrap_or_else(|errors| {
        for (line, code, msg) in &errors {
            eprintln!("bruh: error[{code}] at line {line}: {msg}");
        }
        process::exit(1);
    }).stmts
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => {
            let stmts = compile(&file);
            let mut interp = interp::Interpreter::new();
            if let Err(e) = interp.run(&stmts) {
                eprintln!("bruh: IO error: {e}");
                process::exit(2);
            }
        }

        Commands::Check { file } => {
            compile(&file);
        }

        Commands::Build { file, output, emit } => {
            let stmts = compile(&file);

            match emit {
                EmitFormat::Wat => {
                    let out_path = output.unwrap_or_else(|| file.with_extension("wat"));
                    let text = codegen::wasm::generate_wat(&stmts);
                    fs::write(&out_path, text).unwrap_or_else(|e| {
                        eprintln!("bruh: IO error writing {}: {e}", out_path.display());
                        process::exit(2);
                    });
                }
                EmitFormat::Wasm => {
                    let out_path = output.unwrap_or_else(|| file.with_extension("wasm"));
                    let bytes = codegen::wasm::generate_wasm(&stmts).unwrap_or_else(|e| {
                        eprintln!("bruh: WASM codegen error: {e}");
                        process::exit(1);
                    });
                    fs::write(&out_path, &bytes).unwrap_or_else(|e| {
                        eprintln!("bruh: IO error writing {}: {e}", out_path.display());
                        process::exit(2);
                    });
                }
            }
        }
    }
}
