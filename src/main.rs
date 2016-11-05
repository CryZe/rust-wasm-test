#[macro_use]
extern crate webplatform;
extern crate libc;

mod grammar;
mod lambda;
mod ast;
mod helpers;

use std::borrow::Cow;

// fn calculate(program: &str) -> Cow<'static, str> {
//     if program.is_empty() {
//         return "Type an expression".into();
//     }

//     match grammar::parse_Expr(program) {
//         Ok(v) => v.to_string().into(),
//         Err(e) => format!("Error: {:?}", e).into(),
//     }
// }

fn calculate(program: &str) -> Cow<'static, str> {
    if program.is_empty() {
        return "Type an expression".into();
    }

    match lambda::parse_Expr(program) {
        Ok(v) => v.evaluate().to_string().into(),
        Err(e) => format!("Error: {:?}", e).into(),
    }
}

fn main() {
    let document = webplatform::init();
    let body = document.element_query("body").unwrap();
    body.html_set(r#"
    <h1>Calculator</h1>
    <form>
    <p><input id="text" name="text"></p>
    </form>
    <div>Type an expression</div>
    "#);

    let input = document.element_query("input").unwrap();
    let div = document.element_query("div").unwrap();

    input.on("input", move |_| {
        let input = document.element_query("input").unwrap();
        let program = input.prop_get_str("value");
        let result = calculate(&program);
        div.html_set(&result);
    });

    webplatform::spin();
}
