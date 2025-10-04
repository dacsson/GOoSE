//! Parse a Go file into an AST

use gosyn::parse_source;

pub fn parse_go_file(code: &str) -> Result<gosyn::ast::File, Box<dyn std::error::Error>> {
    let tree = parse_source(code)?;
    Ok(tree)
}