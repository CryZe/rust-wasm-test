use ast::*;
use ast::Token::*;

pub fn lambda(s: &str, t: Token) -> Token {
    Lambda(s.to_string(), Box::new(t))
}

pub fn function(s: &[&str], t: Token) -> Token {
    s.iter().rev().fold(t, |acc, &item| lambda(item, acc))
}

pub fn v(s: &str) -> Token {
    Variable(s.to_string())
}

pub fn app(a: Token, b: Token) -> Token {
    Application(Box::new(a), Box::new(b))
}

pub fn call(t: &[Token]) -> Token {
    let mut it = t.iter();
    let first = it.next().unwrap();
    it.fold(first.clone(), |acc, item| app(acc, item.clone()))
}

pub fn i(n: usize) -> Token {
    function(&["f", "x"], (0..n).fold(v("x"), |acc, _| app(v("f"), acc)))
}

pub fn b(c: bool) -> Token {
    function(&["t", "e"], v(if c { "t" } else { "e" }))
}

pub fn ifte(c: Token, t: Token, e: Token) -> Token {
    call(&[c, t, e])
}

pub fn and_() -> Token {
    function(&["a", "b"], ifte(v("a"), v("b"), v("a")))
}

pub fn and(a: Token, b: Token) -> Token {
    call(&[and_(), a, b])
}

pub fn not_() -> Token {
    lambda("a", ifte(v("a"), b(false), b(true)))
}

pub fn not(a: Token) -> Token {
    app(not_(), a)
}

pub fn or_() -> Token {
    function(&["a", "b"], ifte(v("a"), v("a"), v("b")))
}

pub fn or(a: Token, b: Token) -> Token {
    call(&[or_(), a, b])
}

pub fn boolean_equal_() -> Token {
    function(&["a", "b"],
             ifte(v("a"), v("b"), ifte(not(v("a")), not(v("b")), b(false))))
}

pub fn boolean_equal(a: Token, b: Token) -> Token {
    call(&[boolean_equal_(), a, b])
}

pub fn xor_() -> Token {
    function(&["a", "b"], not(boolean_equal(v("a"), v("b"))))
}

pub fn xor(a: Token, b: Token) -> Token {
    call(&[xor_(), a, b])
}

pub fn to_int(a: Token) -> usize {
    let mut i = 0;
    let number_function = a.evaluate();
    if let Lambda(_, ref inner) = number_function {
        if let &Lambda(_, ref inner2) = inner as &Token {
            let mut token: &Token = inner2;
            while let &Application(_, ref right) = token {
                i = i + 1;
                token = right;
            }
        }
    }
    i
}

pub fn successor_() -> Token {
    function(&["n", "f", "x"],
             app(v("f"), call(&[v("n"), v("f"), v("x")])))
}

pub fn successor(n: Token) -> Token {
    app(successor_(), n)
}

pub fn plus_() -> Token {
    lambda("m", app(v("m"), successor_()))
}

pub fn plus(m: Token, n: Token) -> Token {
    call(&[plus_(), m, n])
}

pub fn multiply_() -> Token {
    function(&["m", "n"], call(&[v("m"), app(plus_(), v("n")), i(0)]))
}

pub fn multiply(m: Token, n: Token) -> Token {
    call(&[multiply_(), m, n])
}

pub fn predecessor_() -> Token {
    function(&["n", "f", "x"],
             call(&[v("n"),
                    function(&["g", "h"], app(v("h"), app(v("g"), v("f")))),
                    lambda("u", v("x")),
                    lambda("u", v("u"))]))
}

pub fn predecessor(n: Token) -> Token {
    app(predecessor_(), n)
}

pub fn minus_() -> Token {
    function(&["m", "n"], call(&[v("n"), predecessor_(), v("m")]))
}

pub fn minus(m: Token, n: Token) -> Token {
    call(&[minus_(), m, n])
}

pub fn pow_() -> Token {
    function(&["m", "n"], app(v("n"), v("m")))
}

pub fn pow(m: Token, n: Token) -> Token {
    call(&[pow_(), m, n])
}

pub fn is_zero_() -> Token {
    lambda("n", call(&[v("n"), lambda("x", b(false)), b(true)]))
}

pub fn is_zero(n: Token) -> Token {
    app(is_zero_(), n)
}

pub fn is_less_than_or_equal_() -> Token {
    function(&["m", "n"], is_zero(minus(v("m"), v("n"))))
}

pub fn is_less_than_or_equal(m: Token, n: Token) -> Token {
    call(&[is_less_than_or_equal_(), m, n])
}

pub fn is_greater_than_() -> Token {
    function(&["m", "n"], not(is_less_than_or_equal(v("m"), v("n"))))
}

pub fn is_greater_than(m: Token, n: Token) -> Token {
    call(&[is_greater_than_(), m, n])
}

pub fn is_equal_() -> Token {
    function(&["m", "n"],
             and(is_less_than_or_equal(v("m"), v("n")),
                 is_less_than_or_equal(v("n"), v("m"))))
}

pub fn is_equal(m: Token, n: Token) -> Token {
    call(&[is_equal_(), m, n])
}

pub fn is_even_() -> Token {
    lambda("n", call(&[v("n"), not_(), b(true)]))
}

pub fn is_even(n: Token) -> Token {
    app(is_even_(), n)
}

pub fn is_odd_() -> Token {
    lambda("n", call(&[v("n"), not_(), b(false)]))
}

pub fn is_odd(n: Token) -> Token {
    app(is_odd_(), n)
}

pub fn y_() -> Token {
    let inner = lambda("x", app(v("f"), app(v("x"), v("x"))));
    lambda("f", app(inner.clone(), inner))
}

pub fn y(f: Token) -> Token {
    app(y_(), f)
}

pub fn factorial_() -> Token {
    y(function(&["factorial", "n"],
               ifte(is_zero(v("n")),
                    i(1),
                    multiply(v("n"), app(v("factorial"), predecessor(v("n")))))))
}

pub fn factorial(n: Token) -> Token {
    app(factorial_(), n)
}

pub fn modulo_() -> Token {
    y(function(&["modulo", "a", "b"],
               ifte(is_less_than_or_equal(v("b"), v("a")),
                    call(&[v("modulo"), minus(v("a"), v("b")), v("b")]),
                    v("a"))))
}

pub fn modulo(a: Token, b: Token) -> Token {
    call(&[modulo_(), a, b])
}

pub fn greatest_common_divisor_() -> Token {
    y(function(&["gcd", "a", "b"],
               ifte(is_greater_than(v("a"), v("b")),
                    call(&[v("gcd"), minus(v("a"), v("b")), v("b")]),
                    ifte(is_greater_than(v("b"), v("a")),
                         call(&[v("gcd"), v("a"), minus(v("b"), v("a"))]),
                         v("a")))))
}

pub fn greatest_common_divisor(a: Token, b: Token) -> Token {
    call(&[greatest_common_divisor_(), a, b])
}

pub fn divide_() -> Token {
    let recursion =
        y(function(&["div", "i", "a", "b"],
                   ifte(is_less_than_or_equal(v("b"), v("a")),
                        call(&[v("div"), successor(v("i")), minus(v("a"), v("b")), v("b")]),
                        v("i"))));

    app(recursion, i(0))
}

pub fn divide(a: Token, b: Token) -> Token {
    call(&[divide_(), a, b])
}

pub fn tuple_() -> Token {
    function(&["first", "second", "x"],
             ifte(v("x"), v("first"), v("second")))
}

pub fn tuple(first: Token, second: Token) -> Token {
    call(&[tuple_(), first, second])
}

pub fn first_() -> Token {
    lambda("tuple", app(v("tuple"), b(true)))
}

pub fn first(tuple: Token) -> Token {
    app(first_(), tuple)
}

pub fn second_() -> Token {
    lambda("tuple", app(v("tuple"), b(false)))
}

pub fn second(tuple: Token) -> Token {
    app(second_(), tuple)
}

pub fn list(elements: &[Token]) -> Token {
    let empty_list = tuple(i(0), i(0));
    elements.iter().cloned().rev().fold(empty_list, |acc, element| tuple(element, acc))
}

pub fn to_vec(list: &Token) -> Vec<Token> {
    let mut remaining_list = list.evaluate();
    let mut token_list = Vec::new();

    while let Variable(literal) = ifte(is_empty(remaining_list.clone()), v("t"), v("f"))
        .evaluate() {
        if literal == "t" {
            break;
        }
        token_list.push(get_element(remaining_list.clone()).evaluate());
        remaining_list = next(remaining_list.clone()).evaluate();
    }

    token_list
}

pub fn to_int_vec(list: &Token) -> Vec<usize> {
    let token_vec = to_vec(list);
    token_vec.iter().cloned().map(|x| to_int(x)).collect()
}

pub fn is_empty_() -> Token {
    lambda("list", is_zero(get_element(v("list"))))
}

pub fn is_empty(list: Token) -> Token {
    app(is_empty_(), list)
}

pub fn get_element_() -> Token {
    first_()
}

pub fn get_element(list: Token) -> Token {
    app(get_element_(), list)
}

pub fn next_() -> Token {
    second_()
}

pub fn next(list: Token) -> Token {
    app(next_(), list)
}

pub fn append_() -> Token {
    tuple_()
}

pub fn append(element: Token, list: Token) -> Token {
    call(&[append_(), element, list])
}

pub fn map_() -> Token {
    y(function(&["map", "function", "list"],
               ifte(is_empty(v("list")),
                    v("list"),
                    append(app(v("function"), get_element(v("list"))),
                           call(&[v("map"), v("function"), next(v("list"))])))))
}

pub fn map(function: Token, list: Token) -> Token {
    call(&[map_(), function, list])
}

pub fn filter_() -> Token {
    y(function(&["filter", "predicate", "list"],
               ifte(is_empty(v("list")),
                    v("list"),
                    ifte(app(v("predicate"), get_element(v("list"))),
                         append(get_element(v("list")),
                                call(&[v("filter"), v("predicate"), next(v("list"))])),
                         call(&[v("filter"), v("predicate"), next(v("list"))])))))
}

pub fn filter(predicate: Token, list: Token) -> Token {
    call(&[filter_(), predicate, list])
}

pub fn fold_() -> Token {
    y(function(&["fold", "function", "initial", "list"],
               ifte(is_empty(v("list")),
                    v("initial"),
                    call(&[v("fold"),
                           v("function"),
                           call(&[v("function"), v("initial"), get_element(v("list"))]),
                           next(v("list"))]))))
}

pub fn fold(function: Token, initial: Token, list: Token) -> Token {
    call(&[fold_(), function, initial, list])
}

pub fn fold1_() -> Token {
    function(&["function", "list"],
             call(&[fold_(), v("function"), get_element(v("list")), next(v("list"))]))
}

pub fn fold1(function: Token, list: Token) -> Token {
    call(&[fold1_(), function, list])
}

pub fn unfold_() -> Token {
    y(function(&["unfold", "function", "initial"],
               append(v("initial"),
                      call(&[v("unfold"), v("function"), app(v("function"), v("initial"))]))))
}

pub fn unfold(function: Token, initial: Token) -> Token {
    call(&[unfold_(), function, initial])
}

pub fn take_() -> Token {
    y(function(&["take", "count", "list"],
               ifte(is_empty(v("list")),
                    v("list"),
                    ifte(is_zero(v("count")),
                         list(&[]),
                         append(get_element(v("list")),
                                call(&[v("take"), predecessor(v("count")), next(v("list"))]))))))
}

pub fn take(count: Token, list: Token) -> Token {
    call(&[take_(), count, list])
}

pub fn range_() -> Token {
    function(&["min", "max"],
             take(minus(v("max"), v("min")), unfold(successor_(), v("min"))))
}

pub fn range(min: Token, max: Token) -> Token {
    call(&[range_(), min, max])
}

pub fn fold_based_factorial_() -> Token {
    lambda("n", fold(multiply_(), i(1), range(i(1), successor(v("n")))))
}

pub fn fold_based_factorial(n: Token) -> Token {
    app(fold_based_factorial_(), n)
}
