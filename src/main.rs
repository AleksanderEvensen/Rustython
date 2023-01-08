#![allow(unused)]

use std::collections::HashSet;

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
    symbol_name: String,
    symbol_type: SymbolTreeType,
    nested: bool,
    has_children: bool,
    identifiers: HashSet<String>,
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

    let mut ast_walker = ast.root_node().walk();

    let mut symtree = SymbolTree {
        filename: "main.py".to_string(),
        module_entry: SymbolTreeEntry {
            symbol_name: "MODULE".to_string(),
            has_children: false,
            nested: false,
            symbol_type: SymbolTreeType::Module,
            symbols: vec![],
            identifiers: HashSet::new(),
        },
    };

    if ast_walker.node().kind() == "module" {
        let (identifiers, symbols) = parse_block_symbols(&mut ast_walker, &code, false);

        symtree.module_entry.has_children = symbols.len() > 0;
        symtree.module_entry.identifiers = identifiers;
        symtree.module_entry.symbols = symbols;
    }

    println!("{symtree:#?}");
}

fn get_string_at_cursor(cursor: &TreeCursor, code: &String) -> String {
    let node = cursor.node();
    code.chars()
        .skip(node.start_byte())
        .take(node.end_byte() - node.start_byte())
        .collect()
}

fn parse_block_symbols(
    cursor: &mut TreeCursor,
    code: &String,
    nested: bool,
) -> (HashSet<String>, Vec<SymbolTreeEntry>) {
    cursor.goto_first_child();

    let mut symbols = vec![];
    let mut identifiers: HashSet<String> = HashSet::new();

    loop {
        match cursor.node().kind() {
            "function_definition" => {
                cursor.goto_first_child();
                cursor.goto_next_sibling();

                let fn_name = get_string_at_cursor(cursor, code);

                identifiers.insert(fn_name.clone());

                cursor.goto_next_sibling();

                let (mut fn_identifiers, _) = parse_block_symbols(cursor, &code, true);

                loop {
                    if cursor.node().kind() == "block" {
                        break;
                    }
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                if cursor.node().kind() == "block" {
                    let (mut fn_block_identifiers, fn_symbols) =
                        parse_block_symbols(cursor, &code, true);

                    fn_identifiers.extend(fn_block_identifiers);

                    symbols.push(SymbolTreeEntry {
                        symbol_name: fn_name,
                        symbol_type: SymbolTreeType::Function,
                        nested,
                        has_children: fn_symbols.len() > 0,
                        identifiers: fn_identifiers,
                        symbols: fn_symbols,
                    });
                }

                cursor.goto_parent();
            }
            "class_definition" => {
                cursor.goto_first_child();
                cursor.goto_next_sibling();

                let class_name = get_string_at_cursor(cursor, code);
                identifiers.insert(class_name.clone());

                loop {
                    if cursor.node().kind() == "block" {
                        break;
                    }
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }

                if cursor.node().kind() == "block" {
                    let (class_identifiers, class_symbols) =
                        parse_block_symbols(cursor, code, true);

                    symbols.push(SymbolTreeEntry {
                        symbol_name: class_name,
                        symbol_type: SymbolTreeType::Class,
                        nested,
                        has_children: symbols.len() > 0,
                        identifiers: class_identifiers,
                        symbols: class_symbols,
                    });
                }

                cursor.goto_parent();
            }
            "decorated_definition" => {
                todo!("Implement the logic for function decorators")
            }

            _ => identifiers.extend(get_all_block_identifiers(cursor, &code)),
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }
    cursor.goto_parent();

    return (identifiers, symbols);
}

fn get_all_block_identifiers(cursor: &mut TreeCursor, code: &String) -> Vec<String> {
    let moved_into_child = cursor.goto_first_child();
    let mut all_of_kind: Vec<String> = vec![];
    loop {
        let node = cursor.node();

        if node.kind() == "identifier" {
            all_of_kind.push(get_string_at_cursor(cursor, code));
        } else if node.kind() == "default_parameter" {
            cursor.goto_first_child();
            all_of_kind.push(get_string_at_cursor(cursor, code));
            cursor.goto_parent();
        } else if (node.child_count() > 0) {
            all_of_kind.append(&mut get_all_block_identifiers(cursor, code))
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }
    if moved_into_child {
        cursor.goto_parent();
    }

    return all_of_kind;
}
