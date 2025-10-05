use std::path::PathBuf;
use clap::Parser;
use goose::parse::parser::parse_go_file;

/// Go symbolic execution tool.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Go file input
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let file = PathBuf::from(args.file);

    let code = std::fs::read_to_string(file).expect("Unable to read file");
    let tree = parse_go_file(&code);

    match tree {
        Ok(tree) => {
            goose::parse::pretty::pretty_print_tree(&tree);
        },
        Err(e) => eprintln!("Error parsing file: {}", e),
    }
}
