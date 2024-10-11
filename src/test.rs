use std::cell::RefCell;
use std::rc::Rc;
use toy_lang_lexer::lexer::analyze;
use toy_lang_lexer::token::Item;
#[test]
fn basic_test() {
    let code = "\
        var x;\n\
        var y;\n\
        input x;\n\
        if (x > 5) {\n\
            y = x * (x / 2 - 10);\n\
        }\n\
        print \"After \tif, finished!\";\
    ";

    let vec = Rc::new(RefCell::new(Vec::new()));
    let vec_clone = Rc::clone(&vec);
    let f = move |item: Item| {
        vec_clone.borrow_mut().push(item);
    };

    analyze(code, Box::new(f));

    assert!(vec.borrow().len() > 0);
}
