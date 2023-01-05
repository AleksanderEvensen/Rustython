use std::default;

use tree_sitter::{Parser, TreeCursor};

#[derive(Debug, Clone, Default)]
struct SymbolTree {
    filename: String,
    module_entry: SymbolTreeEntry,
}

#[derive(Debug, Clone, Default)]
enum SymbolTreeType {
    #[default]
    Module,
    Function,
    Class,
}

#[derive(Debug, Clone, Default)]
struct SymbolTreeEntry {
    symbol_type: SymbolTreeType,
    nested: bool,
    has_children: bool,
    identifiers: Vec<String>,
    symbols: Vec<SymbolTreeEntry>,
}

fn main() {
    let code = std::fs::read_to_string("./python/main.py").unwrap();

    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_python::language())
        .expect("Failed to load the python tree-sitter grammars");

    let ast = parser
        .parse(&code, None)
        .expect("Failed to parse the python code");

    // println!("{}", ast.root_node().to_sexp());

    let mut ast_walker = ast.root_node().walk();

    let mut symtree = SymbolTree {
        filename: "main.py".to_string(),
        module_entry: SymbolTreeEntry {
            has_children: true,
            nested: false,
            symbol_type: SymbolTreeType::Module,
            symbols: vec![],
            identifiers: vec![],
        },
    };

    if ast_walker.node().kind() == "module" {
        let (identifiers, symbols) = parse_block_symbols(&mut ast_walker, &code, false);
    }
}

fn parse_block_symbols(
    cursor: &mut TreeCursor,
    code: &String,
    nested: bool,
) -> (Vec<String>, Vec<SymbolTreeEntry>) {
    cursor.goto_first_child();

    let mut symbols = vec![];
    let mut identifiers = vec![];

    loop {
        match cursor.node().kind() {
            "function_definition" => {}
            "class_definition" => todo!("Implement class block parsing"),
            "expression_statement" => if cursor.goto_first_child() && cursor.goto_first_child() {},

            // Just ignore these, when parsing
            "comment" => {}
            v => {
                todo!("Add implementation for symbol kind '{v}'")
            }
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }

    while cursor.goto_next_sibling() {}

    cursor.goto_parent();

    return (identifiers, symbols);
}
