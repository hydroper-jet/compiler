use clap::Parser;
use file_paths::FlexPath;
use std::{env, fs, io};
use hydroper_jet_compiler::ns::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    source_path: String,

    #[arg(short, long)]
    file_log: bool,
}

fn main() -> io::Result<()> {
    let arguments = Arguments::parse();
    let source_path = FlexPath::from_n_native([env::current_dir().unwrap().to_string_lossy().into_owned().as_ref(), arguments.source_path.as_ref()]).to_string_with_flex_separator();

    // Canonicalize path
    // let source_path = std::path::Path::new(&source_path).canonicalize().unwrap().to_string_lossy().into_owned();

    let jetpm_target_path = FlexPath::new_native(&source_path).resolve("../jetpm-target").to_string_with_flex_separator();
    let source_path_ast_json = FlexPath::new_native(&source_path).change_extension(".ast.json").to_string_with_flex_separator();
    let source_path_diagnostics = FlexPath::new_native(&source_path).change_extension(".diag").to_string_with_flex_separator();
    let source_content = fs::read_to_string(&source_path)?;
    let host = SymbolHost::new(&jetpm_target_path);
    let compilation_unit = CompilationUnit::new(Some(source_path), source_content, &CompilerOptions::new());
    if let Some(program) = ParserFacade::parse_program(&compilation_unit, &host) {
        if arguments.file_log {
            fs::write(&source_path_ast_json, serde_json::to_string_pretty(&program).unwrap())?;
        } else {
            println!("Jet program successfuly parsed.");
        }
    } else {
        if arguments.file_log {
            fs::write(&source_path_ast_json, "{}")?;
        } else {
            println!("Jet program failed to parse.");
        }
    }
    let mut diagnostics = vec![];
    compilation_unit.sort_diagnostics();
    for diagnostic in compilation_unit.diagnostics() {
        diagnostics.push(diagnostic.format_english());
    }
    if arguments.file_log {
        fs::write(&source_path_diagnostics, diagnostics.join("\n"))?;
    } else {
        for diagnostic in diagnostics {
            println!("{diagnostic}");
        }
    }
    Ok(())
}