use crate::grammar::parse_tree as pt;
use crate::grammar::repr as r;

pub fn emit_ast(grammar: &pt::Grammar, normalized_grammar: &r::Grammar) -> std::io::Result<Vec<u8>> {
    for item in &grammar.items {
        let nonterminal = match item {
            pt::GrammarItem::Nonterminal(item) => item,
            _ => continue,
        };

        let generate_name = match &nonterminal.type_decl {
            Some(pt::TypeRef::Generate(name)) => name.clone(),
            _ => continue,
        };

        let normalized_nonterminal = match normalized_grammar.nonterminals.get(&nonterminal.name) {
            Some(data) => data,
            None => continue,
        };
    }

    Ok(Vec::new())
}
