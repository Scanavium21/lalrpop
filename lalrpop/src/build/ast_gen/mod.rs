use std::io::{Error, ErrorKind, Result};

use crate::collections::Map;
use crate::grammar::parse_tree as pt;
use crate::grammar::repr as r;
use crate::grammar::repr::{ActionFnDefn, ActionFnDefnKind, TypeRepr};

pub fn emit_ast(grammar: &pt::Grammar, normalized_grammar: &r::Grammar) -> Result<Vec<u8>> {
    let mut output = Vec::new();

    for item in &grammar.items {
        let (nonterminal, name) = match item {
            pt::GrammarItem::Nonterminal(item) => (item, &item.name),
            _ => continue,
        };

        let generate_name = match &nonterminal.type_decl {
            Some(pt::TypeRef::Generate(name)) => name,
            _ => continue,
        };

        let data = match normalized_grammar.nonterminals.get(name) {
            Some(data) => data,
            None => return Err(Error::new(ErrorKind::InvalidData, format!("data for nonterminal `{name}` not found"))),
        };

        infer_nonterminal_type(name, generate_name, data, normalized_grammar)?;
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

fn infer_nonterminal_type(
    name: &NonterminalString,
    generate_name: &Atom,
    data: &r::NonterminalData,
    grammar: &r::Grammar,
) -> Result<()> {
    for production in &data.productions {
        let action = &grammar.action_fn_defns[production.action.index()];
        let args = build_arg_map(action);

        let code = match &action.kind {
            ActionFnDefnKind::User(user_data) => &user_data.code,
            ActionFnDefnKind::Inline(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("AST generation not supported for inlined actions (`{name}`)"),
                ));
            }
            ActionFnDefnKind::Lookaround(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("AST generation not supported for lookaround actions (`{name}`)"),
                ));
            }
        };
    }

    Ok(())
}
