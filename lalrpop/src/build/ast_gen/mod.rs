use crate::grammar::parse_tree as pt;
use crate::grammar::repr as r;

pub fn emit_ast(grammar: &pt::Grammar, normalized_grammar: &r::Grammar) -> std::io::Result<Vec<u8>> {
    Ok(Vec::new())
}
