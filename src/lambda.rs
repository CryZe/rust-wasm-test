use ast::Token;
use helpers;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::Token;
    use helpers;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2e_22(&'input str),
        Term_22and_22(&'input str),
        Term_22false_22(&'input str),
        Term_22or_22(&'input str),
        Term_22true_22(&'input str),
        Term_22xor_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5b_5c_5c_5c_5c_3bb_5d_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_2b_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(&'input str),
        NtApplication(Token),
        NtExpr(Token),
        NtLit(Token),
        NtStrLit(String),
        NtTerm(Token),
        Nt____Expr(Token),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        7, // on "(", goto 6
        0, // on ")", error
        0, // on ".", error
        8, // on "and", goto 7
        9, // on "false", goto 8
        10, // on "or", goto 9
        11, // on "true", goto 10
        12, // on "xor", goto 11
        13, // on r#"[0-9]+"#, goto 12
        14, // on r#"[\\\\λ]"#, goto 13
        15, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, goto 14
        // State 1
        7, // on "(", goto 6
        0, // on ")", error
        0, // on ".", error
        8, // on "and", goto 7
        9, // on "false", goto 8
        10, // on "or", goto 9
        11, // on "true", goto 10
        12, // on "xor", goto 11
        13, // on r#"[0-9]+"#, goto 12
        0, // on r#"[\\\\λ]"#, error
        15, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, goto 14
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on ".", error
        0, // on "and", error
        0, // on "false", error
        0, // on "or", error
        0, // on "true", error
        0, // on "xor", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[\\\\λ]"#, error
        0, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, error
        // State 3
        -13, // on "(", reduce `Term = Lit => ActionFn(5);`
        0, // on ")", error
        0, // on ".", error
        -13, // on "and", reduce `Term = Lit => ActionFn(5);`
        -13, // on "false", reduce `Term = Lit => ActionFn(5);`
        -13, // on "or", reduce `Term = Lit => ActionFn(5);`
        -13, // on "true", reduce `Term = Lit => ActionFn(5);`
        -13, // on "xor", reduce `Term = Lit => ActionFn(5);`
        -13, // on r#"[0-9]+"#, reduce `Term = Lit => ActionFn(5);`
        0, // on r#"[\\\\λ]"#, error
        -13, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Term = Lit => ActionFn(5);`
        // State 4
        -5, // on "(", reduce `Lit = StrLit => ActionFn(7);`
        0, // on ")", error
        0, // on ".", error
        -5, // on "and", reduce `Lit = StrLit => ActionFn(7);`
        -5, // on "false", reduce `Lit = StrLit => ActionFn(7);`
        -5, // on "or", reduce `Lit = StrLit => ActionFn(7);`
        -5, // on "true", reduce `Lit = StrLit => ActionFn(7);`
        -5, // on "xor", reduce `Lit = StrLit => ActionFn(7);`
        -5, // on r#"[0-9]+"#, reduce `Lit = StrLit => ActionFn(7);`
        0, // on r#"[\\\\λ]"#, error
        -5, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Lit = StrLit => ActionFn(7);`
        // State 5
        -2, // on "(", reduce `Application = Term => ActionFn(4);`
        0, // on ")", error
        0, // on ".", error
        -2, // on "and", reduce `Application = Term => ActionFn(4);`
        -2, // on "false", reduce `Application = Term => ActionFn(4);`
        -2, // on "or", reduce `Application = Term => ActionFn(4);`
        -2, // on "true", reduce `Application = Term => ActionFn(4);`
        -2, // on "xor", reduce `Application = Term => ActionFn(4);`
        -2, // on r#"[0-9]+"#, reduce `Application = Term => ActionFn(4);`
        0, // on r#"[\\\\λ]"#, error
        -2, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Application = Term => ActionFn(4);`
        // State 6
        22, // on "(", goto 21
        0, // on ")", error
        0, // on ".", error
        23, // on "and", goto 22
        24, // on "false", goto 23
        25, // on "or", goto 24
        26, // on "true", goto 25
        27, // on "xor", goto 26
        28, // on r#"[0-9]+"#, goto 27
        29, // on r#"[\\\\λ]"#, goto 28
        30, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, goto 29
        // State 7
        -10, // on "(", reduce `Lit = "and" => ActionFn(12);`
        0, // on ")", error
        0, // on ".", error
        -10, // on "and", reduce `Lit = "and" => ActionFn(12);`
        -10, // on "false", reduce `Lit = "and" => ActionFn(12);`
        -10, // on "or", reduce `Lit = "and" => ActionFn(12);`
        -10, // on "true", reduce `Lit = "and" => ActionFn(12);`
        -10, // on "xor", reduce `Lit = "and" => ActionFn(12);`
        -10, // on r#"[0-9]+"#, reduce `Lit = "and" => ActionFn(12);`
        0, // on r#"[\\\\λ]"#, error
        -10, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Lit = "and" => ActionFn(12);`
        // State 8
        -8, // on "(", reduce `Lit = "false" => ActionFn(10);`
        0, // on ")", error
        0, // on ".", error
        -8, // on "and", reduce `Lit = "false" => ActionFn(10);`
        -8, // on "false", reduce `Lit = "false" => ActionFn(10);`
        -8, // on "or", reduce `Lit = "false" => ActionFn(10);`
        -8, // on "true", reduce `Lit = "false" => ActionFn(10);`
        -8, // on "xor", reduce `Lit = "false" => ActionFn(10);`
        -8, // on r#"[0-9]+"#, reduce `Lit = "false" => ActionFn(10);`
        0, // on r#"[\\\\λ]"#, error
        -8, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Lit = "false" => ActionFn(10);`
        // State 9
        -9, // on "(", reduce `Lit = "or" => ActionFn(11);`
        0, // on ")", error
        0, // on ".", error
        -9, // on "and", reduce `Lit = "or" => ActionFn(11);`
        -9, // on "false", reduce `Lit = "or" => ActionFn(11);`
        -9, // on "or", reduce `Lit = "or" => ActionFn(11);`
        -9, // on "true", reduce `Lit = "or" => ActionFn(11);`
        -9, // on "xor", reduce `Lit = "or" => ActionFn(11);`
        -9, // on r#"[0-9]+"#, reduce `Lit = "or" => ActionFn(11);`
        0, // on r#"[\\\\λ]"#, error
        -9, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Lit = "or" => ActionFn(11);`
        // State 10
        -7, // on "(", reduce `Lit = "true" => ActionFn(9);`
        0, // on ")", error
        0, // on ".", error
        -7, // on "and", reduce `Lit = "true" => ActionFn(9);`
        -7, // on "false", reduce `Lit = "true" => ActionFn(9);`
        -7, // on "or", reduce `Lit = "true" => ActionFn(9);`
        -7, // on "true", reduce `Lit = "true" => ActionFn(9);`
        -7, // on "xor", reduce `Lit = "true" => ActionFn(9);`
        -7, // on r#"[0-9]+"#, reduce `Lit = "true" => ActionFn(9);`
        0, // on r#"[\\\\λ]"#, error
        -7, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Lit = "true" => ActionFn(9);`
        // State 11
        -11, // on "(", reduce `Lit = "xor" => ActionFn(13);`
        0, // on ")", error
        0, // on ".", error
        -11, // on "and", reduce `Lit = "xor" => ActionFn(13);`
        -11, // on "false", reduce `Lit = "xor" => ActionFn(13);`
        -11, // on "or", reduce `Lit = "xor" => ActionFn(13);`
        -11, // on "true", reduce `Lit = "xor" => ActionFn(13);`
        -11, // on "xor", reduce `Lit = "xor" => ActionFn(13);`
        -11, // on r#"[0-9]+"#, reduce `Lit = "xor" => ActionFn(13);`
        0, // on r#"[\\\\λ]"#, error
        -11, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Lit = "xor" => ActionFn(13);`
        // State 12
        -6, // on "(", reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        0, // on ")", error
        0, // on ".", error
        -6, // on "and", reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        -6, // on "false", reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        -6, // on "or", reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        -6, // on "true", reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        -6, // on "xor", reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        -6, // on r#"[0-9]+"#, reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        0, // on r#"[\\\\λ]"#, error
        -6, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        // State 13
        0, // on "(", error
        0, // on ")", error
        0, // on ".", error
        0, // on "and", error
        0, // on "false", error
        0, // on "or", error
        0, // on "true", error
        0, // on "xor", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[\\\\λ]"#, error
        32, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, goto 31
        // State 14
        -12, // on "(", reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        0, // on ")", error
        0, // on ".", error
        -12, // on "and", reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        -12, // on "false", reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        -12, // on "or", reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        -12, // on "true", reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        -12, // on "xor", reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        -12, // on r#"[0-9]+"#, reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        0, // on r#"[\\\\λ]"#, error
        -12, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        // State 15
        -1, // on "(", reduce `Application = Application, Term => ActionFn(3);`
        0, // on ")", error
        0, // on ".", error
        -1, // on "and", reduce `Application = Application, Term => ActionFn(3);`
        -1, // on "false", reduce `Application = Application, Term => ActionFn(3);`
        -1, // on "or", reduce `Application = Application, Term => ActionFn(3);`
        -1, // on "true", reduce `Application = Application, Term => ActionFn(3);`
        -1, // on "xor", reduce `Application = Application, Term => ActionFn(3);`
        -1, // on r#"[0-9]+"#, reduce `Application = Application, Term => ActionFn(3);`
        0, // on r#"[\\\\λ]"#, error
        -1, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Application = Application, Term => ActionFn(3);`
        // State 16
        22, // on "(", goto 21
        -4, // on ")", reduce `Expr = Application => ActionFn(2);`
        0, // on ".", error
        23, // on "and", goto 22
        24, // on "false", goto 23
        25, // on "or", goto 24
        26, // on "true", goto 25
        27, // on "xor", goto 26
        28, // on r#"[0-9]+"#, goto 27
        0, // on r#"[\\\\λ]"#, error
        30, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, goto 29
        // State 17
        0, // on "(", error
        34, // on ")", goto 33
        0, // on ".", error
        0, // on "and", error
        0, // on "false", error
        0, // on "or", error
        0, // on "true", error
        0, // on "xor", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[\\\\λ]"#, error
        0, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, error
        // State 18
        -13, // on "(", reduce `Term = Lit => ActionFn(5);`
        -13, // on ")", reduce `Term = Lit => ActionFn(5);`
        0, // on ".", error
        -13, // on "and", reduce `Term = Lit => ActionFn(5);`
        -13, // on "false", reduce `Term = Lit => ActionFn(5);`
        -13, // on "or", reduce `Term = Lit => ActionFn(5);`
        -13, // on "true", reduce `Term = Lit => ActionFn(5);`
        -13, // on "xor", reduce `Term = Lit => ActionFn(5);`
        -13, // on r#"[0-9]+"#, reduce `Term = Lit => ActionFn(5);`
        0, // on r#"[\\\\λ]"#, error
        -13, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Term = Lit => ActionFn(5);`
        // State 19
        -5, // on "(", reduce `Lit = StrLit => ActionFn(7);`
        -5, // on ")", reduce `Lit = StrLit => ActionFn(7);`
        0, // on ".", error
        -5, // on "and", reduce `Lit = StrLit => ActionFn(7);`
        -5, // on "false", reduce `Lit = StrLit => ActionFn(7);`
        -5, // on "or", reduce `Lit = StrLit => ActionFn(7);`
        -5, // on "true", reduce `Lit = StrLit => ActionFn(7);`
        -5, // on "xor", reduce `Lit = StrLit => ActionFn(7);`
        -5, // on r#"[0-9]+"#, reduce `Lit = StrLit => ActionFn(7);`
        0, // on r#"[\\\\λ]"#, error
        -5, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Lit = StrLit => ActionFn(7);`
        // State 20
        -2, // on "(", reduce `Application = Term => ActionFn(4);`
        -2, // on ")", reduce `Application = Term => ActionFn(4);`
        0, // on ".", error
        -2, // on "and", reduce `Application = Term => ActionFn(4);`
        -2, // on "false", reduce `Application = Term => ActionFn(4);`
        -2, // on "or", reduce `Application = Term => ActionFn(4);`
        -2, // on "true", reduce `Application = Term => ActionFn(4);`
        -2, // on "xor", reduce `Application = Term => ActionFn(4);`
        -2, // on r#"[0-9]+"#, reduce `Application = Term => ActionFn(4);`
        0, // on r#"[\\\\λ]"#, error
        -2, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Application = Term => ActionFn(4);`
        // State 21
        22, // on "(", goto 21
        0, // on ")", error
        0, // on ".", error
        23, // on "and", goto 22
        24, // on "false", goto 23
        25, // on "or", goto 24
        26, // on "true", goto 25
        27, // on "xor", goto 26
        28, // on r#"[0-9]+"#, goto 27
        29, // on r#"[\\\\λ]"#, goto 28
        30, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, goto 29
        // State 22
        -10, // on "(", reduce `Lit = "and" => ActionFn(12);`
        -10, // on ")", reduce `Lit = "and" => ActionFn(12);`
        0, // on ".", error
        -10, // on "and", reduce `Lit = "and" => ActionFn(12);`
        -10, // on "false", reduce `Lit = "and" => ActionFn(12);`
        -10, // on "or", reduce `Lit = "and" => ActionFn(12);`
        -10, // on "true", reduce `Lit = "and" => ActionFn(12);`
        -10, // on "xor", reduce `Lit = "and" => ActionFn(12);`
        -10, // on r#"[0-9]+"#, reduce `Lit = "and" => ActionFn(12);`
        0, // on r#"[\\\\λ]"#, error
        -10, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Lit = "and" => ActionFn(12);`
        // State 23
        -8, // on "(", reduce `Lit = "false" => ActionFn(10);`
        -8, // on ")", reduce `Lit = "false" => ActionFn(10);`
        0, // on ".", error
        -8, // on "and", reduce `Lit = "false" => ActionFn(10);`
        -8, // on "false", reduce `Lit = "false" => ActionFn(10);`
        -8, // on "or", reduce `Lit = "false" => ActionFn(10);`
        -8, // on "true", reduce `Lit = "false" => ActionFn(10);`
        -8, // on "xor", reduce `Lit = "false" => ActionFn(10);`
        -8, // on r#"[0-9]+"#, reduce `Lit = "false" => ActionFn(10);`
        0, // on r#"[\\\\λ]"#, error
        -8, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Lit = "false" => ActionFn(10);`
        // State 24
        -9, // on "(", reduce `Lit = "or" => ActionFn(11);`
        -9, // on ")", reduce `Lit = "or" => ActionFn(11);`
        0, // on ".", error
        -9, // on "and", reduce `Lit = "or" => ActionFn(11);`
        -9, // on "false", reduce `Lit = "or" => ActionFn(11);`
        -9, // on "or", reduce `Lit = "or" => ActionFn(11);`
        -9, // on "true", reduce `Lit = "or" => ActionFn(11);`
        -9, // on "xor", reduce `Lit = "or" => ActionFn(11);`
        -9, // on r#"[0-9]+"#, reduce `Lit = "or" => ActionFn(11);`
        0, // on r#"[\\\\λ]"#, error
        -9, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Lit = "or" => ActionFn(11);`
        // State 25
        -7, // on "(", reduce `Lit = "true" => ActionFn(9);`
        -7, // on ")", reduce `Lit = "true" => ActionFn(9);`
        0, // on ".", error
        -7, // on "and", reduce `Lit = "true" => ActionFn(9);`
        -7, // on "false", reduce `Lit = "true" => ActionFn(9);`
        -7, // on "or", reduce `Lit = "true" => ActionFn(9);`
        -7, // on "true", reduce `Lit = "true" => ActionFn(9);`
        -7, // on "xor", reduce `Lit = "true" => ActionFn(9);`
        -7, // on r#"[0-9]+"#, reduce `Lit = "true" => ActionFn(9);`
        0, // on r#"[\\\\λ]"#, error
        -7, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Lit = "true" => ActionFn(9);`
        // State 26
        -11, // on "(", reduce `Lit = "xor" => ActionFn(13);`
        -11, // on ")", reduce `Lit = "xor" => ActionFn(13);`
        0, // on ".", error
        -11, // on "and", reduce `Lit = "xor" => ActionFn(13);`
        -11, // on "false", reduce `Lit = "xor" => ActionFn(13);`
        -11, // on "or", reduce `Lit = "xor" => ActionFn(13);`
        -11, // on "true", reduce `Lit = "xor" => ActionFn(13);`
        -11, // on "xor", reduce `Lit = "xor" => ActionFn(13);`
        -11, // on r#"[0-9]+"#, reduce `Lit = "xor" => ActionFn(13);`
        0, // on r#"[\\\\λ]"#, error
        -11, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Lit = "xor" => ActionFn(13);`
        // State 27
        -6, // on "(", reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        -6, // on ")", reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        0, // on ".", error
        -6, // on "and", reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        -6, // on "false", reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        -6, // on "or", reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        -6, // on "true", reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        -6, // on "xor", reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        -6, // on r#"[0-9]+"#, reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        0, // on r#"[\\\\λ]"#, error
        -6, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        // State 28
        0, // on "(", error
        0, // on ")", error
        0, // on ".", error
        0, // on "and", error
        0, // on "false", error
        0, // on "or", error
        0, // on "true", error
        0, // on "xor", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[\\\\λ]"#, error
        32, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, goto 31
        // State 29
        -12, // on "(", reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        -12, // on ")", reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        0, // on ".", error
        -12, // on "and", reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        -12, // on "false", reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        -12, // on "or", reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        -12, // on "true", reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        -12, // on "xor", reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        -12, // on r#"[0-9]+"#, reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        0, // on r#"[\\\\λ]"#, error
        -12, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        // State 30
        0, // on "(", error
        0, // on ")", error
        37, // on ".", goto 36
        0, // on "and", error
        0, // on "false", error
        0, // on "or", error
        0, // on "true", error
        0, // on "xor", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[\\\\λ]"#, error
        0, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, error
        // State 31
        0, // on "(", error
        0, // on ")", error
        -12, // on ".", reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        0, // on "and", error
        0, // on "false", error
        0, // on "or", error
        0, // on "true", error
        0, // on "xor", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[\\\\λ]"#, error
        0, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, error
        // State 32
        -1, // on "(", reduce `Application = Application, Term => ActionFn(3);`
        -1, // on ")", reduce `Application = Application, Term => ActionFn(3);`
        0, // on ".", error
        -1, // on "and", reduce `Application = Application, Term => ActionFn(3);`
        -1, // on "false", reduce `Application = Application, Term => ActionFn(3);`
        -1, // on "or", reduce `Application = Application, Term => ActionFn(3);`
        -1, // on "true", reduce `Application = Application, Term => ActionFn(3);`
        -1, // on "xor", reduce `Application = Application, Term => ActionFn(3);`
        -1, // on r#"[0-9]+"#, reduce `Application = Application, Term => ActionFn(3);`
        0, // on r#"[\\\\λ]"#, error
        -1, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Application = Application, Term => ActionFn(3);`
        // State 33
        -14, // on "(", reduce `Term = "(", Expr, ")" => ActionFn(6);`
        0, // on ")", error
        0, // on ".", error
        -14, // on "and", reduce `Term = "(", Expr, ")" => ActionFn(6);`
        -14, // on "false", reduce `Term = "(", Expr, ")" => ActionFn(6);`
        -14, // on "or", reduce `Term = "(", Expr, ")" => ActionFn(6);`
        -14, // on "true", reduce `Term = "(", Expr, ")" => ActionFn(6);`
        -14, // on "xor", reduce `Term = "(", Expr, ")" => ActionFn(6);`
        -14, // on r#"[0-9]+"#, reduce `Term = "(", Expr, ")" => ActionFn(6);`
        0, // on r#"[\\\\λ]"#, error
        -14, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Term = "(", Expr, ")" => ActionFn(6);`
        // State 34
        0, // on "(", error
        38, // on ")", goto 37
        0, // on ".", error
        0, // on "and", error
        0, // on "false", error
        0, // on "or", error
        0, // on "true", error
        0, // on "xor", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[\\\\λ]"#, error
        0, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, error
        // State 35
        0, // on "(", error
        0, // on ")", error
        39, // on ".", goto 38
        0, // on "and", error
        0, // on "false", error
        0, // on "or", error
        0, // on "true", error
        0, // on "xor", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[\\\\λ]"#, error
        0, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, error
        // State 36
        7, // on "(", goto 6
        0, // on ")", error
        0, // on ".", error
        8, // on "and", goto 7
        9, // on "false", goto 8
        10, // on "or", goto 9
        11, // on "true", goto 10
        12, // on "xor", goto 11
        13, // on r#"[0-9]+"#, goto 12
        14, // on r#"[\\\\λ]"#, goto 13
        15, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, goto 14
        // State 37
        -14, // on "(", reduce `Term = "(", Expr, ")" => ActionFn(6);`
        -14, // on ")", reduce `Term = "(", Expr, ")" => ActionFn(6);`
        0, // on ".", error
        -14, // on "and", reduce `Term = "(", Expr, ")" => ActionFn(6);`
        -14, // on "false", reduce `Term = "(", Expr, ")" => ActionFn(6);`
        -14, // on "or", reduce `Term = "(", Expr, ")" => ActionFn(6);`
        -14, // on "true", reduce `Term = "(", Expr, ")" => ActionFn(6);`
        -14, // on "xor", reduce `Term = "(", Expr, ")" => ActionFn(6);`
        -14, // on r#"[0-9]+"#, reduce `Term = "(", Expr, ")" => ActionFn(6);`
        0, // on r#"[\\\\λ]"#, error
        -14, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, reduce `Term = "(", Expr, ")" => ActionFn(6);`
        // State 38
        22, // on "(", goto 21
        0, // on ")", error
        0, // on ".", error
        23, // on "and", goto 22
        24, // on "false", goto 23
        25, // on "or", goto 24
        26, // on "true", goto 25
        27, // on "xor", goto 26
        28, // on r#"[0-9]+"#, goto 27
        29, // on r#"[\\\\λ]"#, goto 28
        30, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, goto 29
        // State 39
        0, // on "(", error
        0, // on ")", error
        0, // on ".", error
        0, // on "and", error
        0, // on "false", error
        0, // on "or", error
        0, // on "true", error
        0, // on "xor", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[\\\\λ]"#, error
        0, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, error
        // State 40
        0, // on "(", error
        -3, // on ")", reduce `Expr = r#"[\\\\λ]"#, StrLit, ".", Expr => ActionFn(1);`
        0, // on ".", error
        0, // on "and", error
        0, // on "false", error
        0, // on "or", error
        0, // on "true", error
        0, // on "xor", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[\\\\λ]"#, error
        0, // on r#"[a-zA-Z]+[a-zA-Z0-9]*"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -4, // on EOF, reduce `Expr = Application => ActionFn(2);`
        -15, // on EOF, reduce `__Expr = Expr => ActionFn(0);`
        -13, // on EOF, reduce `Term = Lit => ActionFn(5);`
        -5, // on EOF, reduce `Lit = StrLit => ActionFn(7);`
        -2, // on EOF, reduce `Application = Term => ActionFn(4);`
        0, // on EOF, error
        -10, // on EOF, reduce `Lit = "and" => ActionFn(12);`
        -8, // on EOF, reduce `Lit = "false" => ActionFn(10);`
        -9, // on EOF, reduce `Lit = "or" => ActionFn(11);`
        -7, // on EOF, reduce `Lit = "true" => ActionFn(9);`
        -11, // on EOF, reduce `Lit = "xor" => ActionFn(13);`
        -6, // on EOF, reduce `Lit = r#"[0-9]+"# => ActionFn(8);`
        0, // on EOF, error
        -12, // on EOF, reduce `StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);`
        -1, // on EOF, reduce `Application = Application, Term => ActionFn(3);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -14, // on EOF, reduce `Term = "(", Expr, ")" => ActionFn(6);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -3, // on EOF, reduce `Expr = r#"[\\\\λ]"#, StrLit, ".", Expr => ActionFn(1);`
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Application, goto 1
        3, // on Expr, goto 2
        4, // on Lit, goto 3
        5, // on StrLit, goto 4
        6, // on Term, goto 5
        0, // on __Expr, error
        // State 1
        0, // on Application, error
        0, // on Expr, error
        4, // on Lit, goto 3
        5, // on StrLit, goto 4
        16, // on Term, goto 15
        0, // on __Expr, error
        // State 2
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 3
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 4
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 5
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 6
        17, // on Application, goto 16
        18, // on Expr, goto 17
        19, // on Lit, goto 18
        20, // on StrLit, goto 19
        21, // on Term, goto 20
        0, // on __Expr, error
        // State 7
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 8
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 9
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 10
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 11
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 12
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 13
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        31, // on StrLit, goto 30
        0, // on Term, error
        0, // on __Expr, error
        // State 14
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 15
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 16
        0, // on Application, error
        0, // on Expr, error
        19, // on Lit, goto 18
        20, // on StrLit, goto 19
        33, // on Term, goto 32
        0, // on __Expr, error
        // State 17
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 18
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 19
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 20
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 21
        17, // on Application, goto 16
        35, // on Expr, goto 34
        19, // on Lit, goto 18
        20, // on StrLit, goto 19
        21, // on Term, goto 20
        0, // on __Expr, error
        // State 22
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 23
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 24
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 25
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 26
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 27
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 28
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        36, // on StrLit, goto 35
        0, // on Term, error
        0, // on __Expr, error
        // State 29
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 30
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 31
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 32
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 33
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 34
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 35
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 36
        2, // on Application, goto 1
        40, // on Expr, goto 39
        4, // on Lit, goto 3
        5, // on StrLit, goto 4
        6, // on Term, goto 5
        0, // on __Expr, error
        // State 37
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 38
        17, // on Application, goto 16
        41, // on Expr, goto 40
        19, // on Lit, goto 18
        20, // on StrLit, goto 19
        21, // on Term, goto 20
        0, // on __Expr, error
        // State 39
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
        // State 40
        0, // on Application, error
        0, // on Expr, error
        0, // on Lit, error
        0, // on StrLit, error
        0, // on Term, error
        0, // on __Expr, error
    ];
    pub fn parse_Expr<
        'input,
    >(
        input: &'input str,
    ) -> Result<Token, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                (_, (8, _), _) if true => 8,
                (_, (9, _), _) if true => 9,
                (_, (10, _), _) if true => 10,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 11 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_2e_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22and_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22false_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22or_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22true_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22xor_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Termr_23_22_5b_5c_5c_5c_5c_3bb_5d_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_2b_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Token,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Application = Application, Term => ActionFn(3);
                let __sym1 = __pop_NtTerm(__symbols);
                let __sym0 = __pop_NtApplication(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtApplication(__nt), __end));
                0
            }
            2 => {
                // Application = Term => ActionFn(4);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtApplication(__nt), __end));
                0
            }
            3 => {
                // Expr = r#"[\\\\λ]"#, StrLit, ".", Expr => ActionFn(1);
                let __sym3 = __pop_NtExpr(__symbols);
                let __sym2 = __pop_Term_22_2e_22(__symbols);
                let __sym1 = __pop_NtStrLit(__symbols);
                let __sym0 = __pop_Termr_23_22_5b_5c_5c_5c_5c_3bb_5d_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            4 => {
                // Expr = Application => ActionFn(2);
                let __sym0 = __pop_NtApplication(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                1
            }
            5 => {
                // Lit = StrLit => ActionFn(7);
                let __sym0 = __pop_NtStrLit(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLit(__nt), __end));
                2
            }
            6 => {
                // Lit = r#"[0-9]+"# => ActionFn(8);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLit(__nt), __end));
                2
            }
            7 => {
                // Lit = "true" => ActionFn(9);
                let __sym0 = __pop_Term_22true_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLit(__nt), __end));
                2
            }
            8 => {
                // Lit = "false" => ActionFn(10);
                let __sym0 = __pop_Term_22false_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLit(__nt), __end));
                2
            }
            9 => {
                // Lit = "or" => ActionFn(11);
                let __sym0 = __pop_Term_22or_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLit(__nt), __end));
                2
            }
            10 => {
                // Lit = "and" => ActionFn(12);
                let __sym0 = __pop_Term_22and_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLit(__nt), __end));
                2
            }
            11 => {
                // Lit = "xor" => ActionFn(13);
                let __sym0 = __pop_Term_22xor_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLit(__nt), __end));
                2
            }
            12 => {
                // StrLit = r#"[a-zA-Z]+[a-zA-Z0-9]*"# => ActionFn(14);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_2b_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStrLit(__nt), __end));
                3
            }
            13 => {
                // Term = Lit => ActionFn(5);
                let __sym0 = __pop_NtLit(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                4
            }
            14 => {
                // Term = "(", Expr, ")" => ActionFn(6);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                4
            }
            15 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 6 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22and_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22and_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22false_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22false_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22or_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22or_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22true_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22true_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22xor_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22xor_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_5c_5c_5c_5c_3bb_5d_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_5c_5c_5c_5c_3bb_5d_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_2b_5ba_2dzA_2dZ0_2d9_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_2b_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtApplication<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtApplication(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLit<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLit(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtStrLit<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStrLit(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Expr::parse_Expr;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        46 => /* '.' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((8, __index + __ch.len_utf8()));
                            __current_state = 4;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        92 => /* '\\' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        98 ... 101 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        102 => /* 'f' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        103 ... 110 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        112 ... 115 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        117 ... 119 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        120 => /* 'x' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        121 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        955 => /* 'λ' */ {
                            __current_match = Some((9, __index + 2));
                            __current_state = 6;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((8, __index + __ch.len_utf8()));
                            __current_state = 13;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((8, __index + __ch.len_utf8()));
                            __current_state = 13;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 99 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        101 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 116 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        118 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Token, usize),
) -> Token
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Token, usize),
) -> Token
{
    Token::Lambda(v, Box::new(e))
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Token, usize),
) -> Token
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, t1, _): (usize, Token, usize),
    (_, t2, _): (usize, Token, usize),
) -> Token
{
    Token::Application(Box::new(t1), Box::new(t2))
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Token, usize),
) -> Token
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Token, usize),
) -> Token
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Token, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Token
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> Token
{
    Token::Variable(__0)
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Token
{
    helpers::i(__0.parse().unwrap())
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Token
{
    helpers::b(true)
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Token
{
    helpers::b(false)
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Token
{
    helpers::or_()
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Token
{
    helpers::and_()
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Token
{
    helpers::xor_()
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
