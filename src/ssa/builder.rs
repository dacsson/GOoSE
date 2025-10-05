//! Construct SSA form from AST
//!
//! For reference, see:
//! [Simple and Efficient Construction of Static Single Assignment Form](https://www.researchgate.net/publication/236997796_Simple_and_Efficient_Construction_of_Static_Single_Assignment_Form)

use std::collections::HashMap;
use gosyn::ast::{BlockStmt, Declaration, Expression, FuncDecl, Statement};
use crate::ssa::syntax::SymbolValue;

/// Just a number of declarations
pub type AST = Vec<Declaration>;
pub type VarName = String;
pub type FuncName = String;

// Id block by his position in the AST
type BlockId = (usize, usize);

pub struct SSABuilder<'ast> {
    ast: &'ast AST,

    // For each basic block we keep a mapping from
    // each source variable to
    // its current defining expression
    current_def: HashMap<BlockId, HashMap<VarName, SymbolValue>>,
}

impl<'ast> SSABuilder<'ast> {
    // When encountering an assignment to a variable,
    // we record the IR of the right-hand side of the assignment as current definition
    // of the variable
    // TODO: do not clone
    fn write_variable(&mut self, var_name: VarName, block_id: BlockId) {
        let block = self
            .current_def
            .entry(block_id)
            .or_insert_with(|| HashMap::new());

        if block.contains_key(&var_name) {
            // Increase count (i.e. make a fresh variable) if it exists in block
            // i.e. "x" => "%x2"
            block.entry(var_name).and_modify(|x| x.count += 1);
        } else {
            // Initialize a new variable, i.e. "x" => "%x0"
            let symbol_value = SymbolValue::new(&var_name);
            block.insert(var_name, symbol_value);
        }
    }

    // When a variable is read, we look up its current
    // definition
    fn read_variable(&self, var_name: VarName, block_id: BlockId) -> Option<&SymbolValue> {
        if let Some(map) = self.current_def.get(&block_id) {
            // Local value numbering
            return map.get(&var_name);
        }

        None
    }

    fn on_block(&mut self, block: BlockStmt) -> BlockStmt {
        let stmts = block.list;
        let mut res = vec![];
        for stmt in stmts {
            match stmt {
                // Write variable on assignment
                Statement::Assign(mut assign_stmt) => {
                    let lhs = &mut assign_stmt.left[0];
                    let rhs = &mut assign_stmt.right[0];

                    // Read variables used in expression and register them in defs
                    self.check_expression(rhs, block.pos);

                    if let Expression::Ident(ident) = lhs {
                        let var_name = VarName::from(&ident.name);
                        ident.name = format!("{}", SymbolValue::new(&ident.name));
                        self.write_variable(var_name, block.pos)
                    }

                    res.push(Statement::Assign(assign_stmt))
                }
                _ => res.push(stmt)
            }
        }

        BlockStmt {
            list: res,
            pos: block.pos,
        }
    }

    // Recursively call to rename variables to their SSA form
    fn check_expression(&self, expr: &mut Expression, block_id: BlockId) {
        match expr {
            Expression::Operation(op_expr) => {
                self.check_expression(&mut op_expr.x, block_id);
                if let Some(_) = &op_expr.y {
                    self.check_expression(op_expr.y.as_mut().unwrap(), block_id);
                }
            }
            Expression::Ident(ident) => {
                let name = VarName::from(&ident.name);
                let symbol_value = self.read_variable(name, block_id);

                if let Some(value) = symbol_value {
                    ident.name = format!("{}", value);
                }
            }
            _ => {}
        }
    }

    pub fn build(&mut self) -> AST {
        let decls = self.ast;
        let mut ssa_tree = vec![];

        for decl in decls {
            match decl {
                Declaration::Function(func_decl) => {
                    // TODO
                    let body = func_decl.body.as_ref().unwrap().to_owned();
                    let block = self.on_block(body);

                    let func_decl = Declaration::Function(
                        FuncDecl {
                            body: Some(block),
                            ..func_decl.clone()
                        }
                    );

                    ssa_tree.push(func_decl);
                }
                _ => ssa_tree.push(decl.clone())
            }
        }

        ssa_tree
    }

    pub fn new(ast: &'ast AST) -> Self {
        SSABuilder {
            ast,
            current_def: HashMap::new(),
        }
    }
}