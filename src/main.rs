//! usage: ./{{project_name}} <filename>

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let code = std::fs::read_to_string(filename).expect("Failed to read file");

    toy_lang_lexer::lexer::analyze(
        &code,
        Box::new(|item| println!("('{:?}', {:?})", item.typ, item.val)),
    );
}
