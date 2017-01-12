extern crate brain;

fn main() {
    let source = r#"
        # foo comment
        ="some text"
        a="some other text"
    "#;

    println!("Source Code:\n\n{}\n", source);

    println!("AST:\n\n{:?}", brain::parse(source.as_bytes()));
}
