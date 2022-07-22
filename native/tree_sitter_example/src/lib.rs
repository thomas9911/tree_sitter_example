#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif]
fn stuff(input: String) -> String {
    let code = input;
    let mut parser = tree_sitter::Parser::new();
    // let dot_file = std::fs::File::create("hallo.txt").unwrap();
    // parser.print_dot_graphs(&dot_file);
    parser.set_language(tree_sitter_json::language()).expect("Error loading json grammar");
    let tree = parser.parse(code, None).unwrap();

    println!("{:?}", tree);
    
    // format!("{}", tree.root_node().to_sexp())
    let node = tree.root_node();

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let mut sub_cursor = child.walk();

        for sub_child in child.children(&mut sub_cursor) {
            println!("{:?}", sub_child);
        }
    }

    String::new()
}

rustler::init!("Elixir.TreeSitterExample", [stuff]);
