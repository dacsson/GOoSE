use std::path::PathBuf;
use clap::Parser;
use goose::parser::parse_go_file;
use goose::ssa::builder::SSABuilder;

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
            let decls = tree.decl;
            println!("{:#?}", decls);

            let mut ssa = SSABuilder::new(&decls);
            let ssa_tree = ssa.build();

            println!("{:#?}", ssa_tree);
        },
        Err(e) => eprintln!("Error parsing file: {}", e),
    }
}
