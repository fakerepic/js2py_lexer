use toy_lang_lexer::lexer::analyze;

fn main() {
    let code = "\
        var x;\n\
        var y;\n\
        input x;\n\
        if (x > 5) {\n\
            y = x * (x / 2 - 10);\n\
        }\n\
        print \"After \tif, finished!\";\
    ";

    let items = analyze(code);

    for item in items {
        println!("('{:?}', '{}')", item.typ, item.val);
    }
}
