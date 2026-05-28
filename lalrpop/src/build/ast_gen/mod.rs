use crate::collections::Map;
use crate::grammar::parse_tree as pt;
use crate::grammar::repr as r;
use crate::grammar::repr::{ActionFnDefn, ActionFnDefnKind, TypeRepr};

pub fn emit_ast(grammar: &pt::Grammar, normalized_grammar: &r::Grammar) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();

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

        for production in &normalized_nonterminal.productions {
            let action = &normalized_grammar.action_fn_defns[production.action.index()];
            let args = build_arg_map(action);
        }
    }

    Ok(output)
}

fn build_arg_map(action: &ActionFnDefn) -> Map<String, TypeRepr> {
    let mut map = Map::new();
    if let ActionFnDefnKind::User(ref data) = action.kind {
        for (pattern, ty) in data.arg_patterns.iter().zip(data.arg_types.iter()) {
            let name = pattern.name();
            map.insert(name, ty.clone());
        }
    }
    map
}
