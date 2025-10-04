//! Pretty print the AST of a Go file.

// TODO: elaborate on dat
pub fn pretty_print_tree(tree: &gosyn::ast::File) {
    let decls = &tree.decl;
    println!("{:#?}", decls);
}
