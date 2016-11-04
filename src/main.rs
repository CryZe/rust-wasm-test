extern crate webplatform;

/// A simple integer calculator:
/// `+` or `-` means add or subtract by 1
/// `*` or `/` means multiply or divide by 2
fn calculate(program: &str) -> u64 {
    let mut accumulator = 0;

    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => {
                // ignore everything else
            }
        }
    }

    accumulator
}

fn main() {
    let document = webplatform::init();
    let body = document.element_query("body").unwrap();
    body.html_set(r#"
    <h1>Calculator</h1>
    <input id="text" name="text"><br/>
    <button>Calculate</button><br/>
    <div />
    "#);

    let button = document.element_query("button").unwrap();
    let input = document.element_query("input").unwrap();
    let div = document.element_query("div").unwrap();

    button.on("click", move |_| {
        let program = input.prop_get_str("value");
        let value = calculate(&program);
        div.html_set(&format!("Value is: {}", value));
    });

    webplatform::spin();
}
