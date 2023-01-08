#![allow(unused_variables)]

mod compiler;
mod symboltree;
mod utils;

use crate::symboltree::SymbolTree;
use anyhow::{Context, Result};
use tree_sitter::{Node, Parser, Tree, TreeCursor};
use utils::GetStringFromNode;

fn main() -> Result<()> {
    let code = std::fs::read_to_string("./python/main.py").unwrap();

    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_python::language())
        .context("Failed to load the python tree-sitter grammars")?;

    let ast = parser
        .parse(&code, None)
        .context("Failed to parse the python code")?;

    check_syntax_errors(&ast, &code).unwrap();

    let symtree = SymbolTree::from_ast(&ast, &code, "main.py".to_string()).unwrap();

    println!("{symtree:#?}");

    Ok(())
}

fn check_syntax_errors(ast: &Tree, code: &String) -> Result<()> {
    fn find_error_node<'a>(cursor: &mut TreeCursor<'a>) -> Option<Node<'a>> {
        cursor.goto_first_child();

        loop {
            if cursor.node().is_error() {
                return Some(cursor.node());
            }

            if cursor.node().child_count() > 0 {
                if let Some(error_node) = find_error_node(cursor) {
                    return Some(error_node);
                }
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
        None
    }

    let mut cursor = ast.root_node().walk();

    if let Some(error_node) = find_error_node(&mut cursor) {
        return Err(anyhow::anyhow!(
            "Found an error '{}' on Line {}",
            error_node.get_string(code),
            error_node.start_position().row + 1,
        ));
    }
    Ok(())
}
