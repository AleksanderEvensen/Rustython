#![allow(unused_variables)]

mod symboltree;
use tree_sitter::Parser;

use crate::symboltree::SymbolTree;

fn main() {
    let code = std::fs::read_to_string("./python/main.py").unwrap();

    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_python::language())
        .expect("Failed to load the python tree-sitter grammars");

    let ast = parser
        .parse(&code, None)
        .expect("Failed to parse the python code");

    let symtree = SymbolTree::from_ast(&ast, &code, "main.py".to_string()).unwrap();

    println!("{symtree:#?}");
}
