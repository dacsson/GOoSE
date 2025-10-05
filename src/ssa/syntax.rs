// struct Phi {
//     operands:
// }

use std::fmt::Display;
use crate::ssa::builder::VarName;

// Symbolic value for a variable
pub struct SymbolValue {
    pub name: String,
    // Version
    pub count: usize,
}

impl SymbolValue {
    pub(crate) fn new(name: &VarName) -> Self {
        SymbolValue {
            name: name.to_string(),
            count: 0,
        }
    }
}

impl Display for SymbolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from("%") + &self.name + "_" + &self.count.to_string()
        )
    }
}