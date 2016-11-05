use std::str::FromStr;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_5e_22(&'input str),
        Term_22cos_28_22(&'input str),
        Term_22hypot_28_22(&'input str),
        Term_22sin_28_22(&'input str),
        Term_22sqrt_28_22(&'input str),
        Term_22tan_28_22(&'input str),
        Termr_23_22_28_28_5c_5cd_2b_28_5c_5c_2e_5c_5cd_2a_29_3f_29_7c_28_5c_5c_2e_5c_5cd_2b_29_29_22_23(&'input str),
        NtExpr(f64),
        NtFactor(f64),
        NtNum(f64),
        NtPower(f64),
        NtTerm(f64),
        Nt____Expr(f64),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        7, // on "(", goto 6
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        8, // on "cos(", goto 7
        9, // on "hypot(", goto 8
        10, // on "sin(", goto 9
        11, // on "sqrt(", goto 10
        12, // on "tan(", goto 11
        13, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 12
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        14, // on "+", goto 13
        0, // on ",", error
        15, // on "-", goto 14
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        16, // on "*", goto 15
        -3, // on "+", reduce `Expr = Factor => ActionFn(3);`
        0, // on ",", error
        -3, // on "-", reduce `Expr = Factor => ActionFn(3);`
        17, // on "/", goto 16
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 3
        0, // on "(", error
        0, // on ")", error
        -10, // on "*", reduce `Term = Num => ActionFn(9);`
        -10, // on "+", reduce `Term = Num => ActionFn(9);`
        0, // on ",", error
        -10, // on "-", reduce `Term = Num => ActionFn(9);`
        -10, // on "/", reduce `Term = Num => ActionFn(9);`
        -10, // on "^", reduce `Term = Num => ActionFn(9);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 4
        0, // on "(", error
        0, // on ")", error
        -6, // on "*", reduce `Factor = Power => ActionFn(6);`
        -6, // on "+", reduce `Factor = Power => ActionFn(6);`
        0, // on ",", error
        -6, // on "-", reduce `Factor = Power => ActionFn(6);`
        -6, // on "/", reduce `Factor = Power => ActionFn(6);`
        18, // on "^", goto 17
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 5
        0, // on "(", error
        0, // on ")", error
        -9, // on "*", reduce `Power = Term => ActionFn(8);`
        -9, // on "+", reduce `Power = Term => ActionFn(8);`
        0, // on ",", error
        -9, // on "-", reduce `Power = Term => ActionFn(8);`
        -9, // on "/", reduce `Power = Term => ActionFn(8);`
        -9, // on "^", reduce `Power = Term => ActionFn(8);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 6
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 7
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 8
        37, // on "(", goto 36
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        38, // on "cos(", goto 37
        39, // on "hypot(", goto 38
        40, // on "sin(", goto 39
        41, // on "sqrt(", goto 40
        42, // on "tan(", goto 41
        43, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 42
        // State 9
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 10
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 11
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 12
        0, // on "(", error
        0, // on ")", error
        -7, // on "*", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        -7, // on "+", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        0, // on ",", error
        -7, // on "-", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        -7, // on "/", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        -7, // on "^", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 13
        7, // on "(", goto 6
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        8, // on "cos(", goto 7
        9, // on "hypot(", goto 8
        10, // on "sin(", goto 9
        11, // on "sqrt(", goto 10
        12, // on "tan(", goto 11
        13, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 12
        // State 14
        7, // on "(", goto 6
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        8, // on "cos(", goto 7
        9, // on "hypot(", goto 8
        10, // on "sin(", goto 9
        11, // on "sqrt(", goto 10
        12, // on "tan(", goto 11
        13, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 12
        // State 15
        7, // on "(", goto 6
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        8, // on "cos(", goto 7
        9, // on "hypot(", goto 8
        10, // on "sin(", goto 9
        11, // on "sqrt(", goto 10
        12, // on "tan(", goto 11
        13, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 12
        // State 16
        7, // on "(", goto 6
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        8, // on "cos(", goto 7
        9, // on "hypot(", goto 8
        10, // on "sin(", goto 9
        11, // on "sqrt(", goto 10
        12, // on "tan(", goto 11
        13, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 12
        // State 17
        7, // on "(", goto 6
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        8, // on "cos(", goto 7
        9, // on "hypot(", goto 8
        10, // on "sin(", goto 9
        11, // on "sqrt(", goto 10
        12, // on "tan(", goto 11
        13, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 12
        // State 18
        0, // on "(", error
        52, // on ")", goto 51
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 19
        0, // on "(", error
        -3, // on ")", reduce `Expr = Factor => ActionFn(3);`
        55, // on "*", goto 54
        -3, // on "+", reduce `Expr = Factor => ActionFn(3);`
        0, // on ",", error
        -3, // on "-", reduce `Expr = Factor => ActionFn(3);`
        56, // on "/", goto 55
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 20
        0, // on "(", error
        -10, // on ")", reduce `Term = Num => ActionFn(9);`
        -10, // on "*", reduce `Term = Num => ActionFn(9);`
        -10, // on "+", reduce `Term = Num => ActionFn(9);`
        0, // on ",", error
        -10, // on "-", reduce `Term = Num => ActionFn(9);`
        -10, // on "/", reduce `Term = Num => ActionFn(9);`
        -10, // on "^", reduce `Term = Num => ActionFn(9);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 21
        0, // on "(", error
        -6, // on ")", reduce `Factor = Power => ActionFn(6);`
        -6, // on "*", reduce `Factor = Power => ActionFn(6);`
        -6, // on "+", reduce `Factor = Power => ActionFn(6);`
        0, // on ",", error
        -6, // on "-", reduce `Factor = Power => ActionFn(6);`
        -6, // on "/", reduce `Factor = Power => ActionFn(6);`
        57, // on "^", goto 56
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 22
        0, // on "(", error
        -9, // on ")", reduce `Power = Term => ActionFn(8);`
        -9, // on "*", reduce `Power = Term => ActionFn(8);`
        -9, // on "+", reduce `Power = Term => ActionFn(8);`
        0, // on ",", error
        -9, // on "-", reduce `Power = Term => ActionFn(8);`
        -9, // on "/", reduce `Power = Term => ActionFn(8);`
        -9, // on "^", reduce `Power = Term => ActionFn(8);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 23
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 24
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 25
        37, // on "(", goto 36
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        38, // on "cos(", goto 37
        39, // on "hypot(", goto 38
        40, // on "sin(", goto 39
        41, // on "sqrt(", goto 40
        42, // on "tan(", goto 41
        43, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 42
        // State 26
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 27
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 28
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 29
        0, // on "(", error
        -7, // on ")", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        -7, // on "*", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        -7, // on "+", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        0, // on ",", error
        -7, // on "-", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        -7, // on "/", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        -7, // on "^", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 30
        0, // on "(", error
        64, // on ")", goto 63
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 31
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        65, // on "+", goto 64
        66, // on ",", goto 65
        67, // on "-", goto 66
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 32
        0, // on "(", error
        0, // on ")", error
        68, // on "*", goto 67
        -3, // on "+", reduce `Expr = Factor => ActionFn(3);`
        -3, // on ",", reduce `Expr = Factor => ActionFn(3);`
        -3, // on "-", reduce `Expr = Factor => ActionFn(3);`
        69, // on "/", goto 68
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 33
        0, // on "(", error
        0, // on ")", error
        -10, // on "*", reduce `Term = Num => ActionFn(9);`
        -10, // on "+", reduce `Term = Num => ActionFn(9);`
        -10, // on ",", reduce `Term = Num => ActionFn(9);`
        -10, // on "-", reduce `Term = Num => ActionFn(9);`
        -10, // on "/", reduce `Term = Num => ActionFn(9);`
        -10, // on "^", reduce `Term = Num => ActionFn(9);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 34
        0, // on "(", error
        0, // on ")", error
        -6, // on "*", reduce `Factor = Power => ActionFn(6);`
        -6, // on "+", reduce `Factor = Power => ActionFn(6);`
        -6, // on ",", reduce `Factor = Power => ActionFn(6);`
        -6, // on "-", reduce `Factor = Power => ActionFn(6);`
        -6, // on "/", reduce `Factor = Power => ActionFn(6);`
        70, // on "^", goto 69
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 35
        0, // on "(", error
        0, // on ")", error
        -9, // on "*", reduce `Power = Term => ActionFn(8);`
        -9, // on "+", reduce `Power = Term => ActionFn(8);`
        -9, // on ",", reduce `Power = Term => ActionFn(8);`
        -9, // on "-", reduce `Power = Term => ActionFn(8);`
        -9, // on "/", reduce `Power = Term => ActionFn(8);`
        -9, // on "^", reduce `Power = Term => ActionFn(8);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 36
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 37
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 38
        37, // on "(", goto 36
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        38, // on "cos(", goto 37
        39, // on "hypot(", goto 38
        40, // on "sin(", goto 39
        41, // on "sqrt(", goto 40
        42, // on "tan(", goto 41
        43, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 42
        // State 39
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 40
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 41
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 42
        0, // on "(", error
        0, // on ")", error
        -7, // on "*", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        -7, // on "+", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        -7, // on ",", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        -7, // on "-", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        -7, // on "/", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        -7, // on "^", reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 43
        0, // on "(", error
        77, // on ")", goto 76
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 44
        0, // on "(", error
        78, // on ")", goto 77
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 45
        0, // on "(", error
        79, // on ")", goto 78
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 46
        0, // on "(", error
        0, // on ")", error
        16, // on "*", goto 15
        -1, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(1);`
        0, // on ",", error
        -1, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(1);`
        17, // on "/", goto 16
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 47
        0, // on "(", error
        0, // on ")", error
        16, // on "*", goto 15
        -2, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(2);`
        0, // on ",", error
        -2, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(2);`
        17, // on "/", goto 16
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 48
        0, // on "(", error
        0, // on ")", error
        -4, // on "*", reduce `Factor = Factor, "*", Power => ActionFn(4);`
        -4, // on "+", reduce `Factor = Factor, "*", Power => ActionFn(4);`
        0, // on ",", error
        -4, // on "-", reduce `Factor = Factor, "*", Power => ActionFn(4);`
        -4, // on "/", reduce `Factor = Factor, "*", Power => ActionFn(4);`
        18, // on "^", goto 17
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 49
        0, // on "(", error
        0, // on ")", error
        -5, // on "*", reduce `Factor = Factor, "/", Power => ActionFn(5);`
        -5, // on "+", reduce `Factor = Factor, "/", Power => ActionFn(5);`
        0, // on ",", error
        -5, // on "-", reduce `Factor = Factor, "/", Power => ActionFn(5);`
        -5, // on "/", reduce `Factor = Factor, "/", Power => ActionFn(5);`
        18, // on "^", goto 17
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 50
        0, // on "(", error
        0, // on ")", error
        -8, // on "*", reduce `Power = Power, "^", Term => ActionFn(7);`
        -8, // on "+", reduce `Power = Power, "^", Term => ActionFn(7);`
        0, // on ",", error
        -8, // on "-", reduce `Power = Power, "^", Term => ActionFn(7);`
        -8, // on "/", reduce `Power = Power, "^", Term => ActionFn(7);`
        -8, // on "^", reduce `Power = Power, "^", Term => ActionFn(7);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 51
        0, // on "(", error
        0, // on ")", error
        -11, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        -11, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        0, // on ",", error
        -11, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        -11, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        -11, // on "^", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 52
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 53
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 54
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 55
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 56
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 57
        0, // on "(", error
        85, // on ")", goto 84
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 58
        0, // on "(", error
        86, // on ")", goto 85
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 59
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        65, // on "+", goto 64
        87, // on ",", goto 86
        67, // on "-", goto 66
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 60
        0, // on "(", error
        88, // on ")", goto 87
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 61
        0, // on "(", error
        89, // on ")", goto 88
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 62
        0, // on "(", error
        90, // on ")", goto 89
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 63
        0, // on "(", error
        0, // on ")", error
        -14, // on "*", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        -14, // on "+", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        0, // on ",", error
        -14, // on "-", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        -14, // on "/", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        -14, // on "^", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 64
        37, // on "(", goto 36
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        38, // on "cos(", goto 37
        39, // on "hypot(", goto 38
        40, // on "sin(", goto 39
        41, // on "sqrt(", goto 40
        42, // on "tan(", goto 41
        43, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 42
        // State 65
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 66
        37, // on "(", goto 36
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        38, // on "cos(", goto 37
        39, // on "hypot(", goto 38
        40, // on "sin(", goto 39
        41, // on "sqrt(", goto 40
        42, // on "tan(", goto 41
        43, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 42
        // State 67
        37, // on "(", goto 36
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        38, // on "cos(", goto 37
        39, // on "hypot(", goto 38
        40, // on "sin(", goto 39
        41, // on "sqrt(", goto 40
        42, // on "tan(", goto 41
        43, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 42
        // State 68
        37, // on "(", goto 36
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        38, // on "cos(", goto 37
        39, // on "hypot(", goto 38
        40, // on "sin(", goto 39
        41, // on "sqrt(", goto 40
        42, // on "tan(", goto 41
        43, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 42
        // State 69
        37, // on "(", goto 36
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        38, // on "cos(", goto 37
        39, // on "hypot(", goto 38
        40, // on "sin(", goto 39
        41, // on "sqrt(", goto 40
        42, // on "tan(", goto 41
        43, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 42
        // State 70
        0, // on "(", error
        97, // on ")", goto 96
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 71
        0, // on "(", error
        98, // on ")", goto 97
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 72
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        65, // on "+", goto 64
        99, // on ",", goto 98
        67, // on "-", goto 66
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 73
        0, // on "(", error
        100, // on ")", goto 99
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 74
        0, // on "(", error
        101, // on ")", goto 100
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 75
        0, // on "(", error
        102, // on ")", goto 101
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 76
        0, // on "(", error
        0, // on ")", error
        -13, // on "*", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        -13, // on "+", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        0, // on ",", error
        -13, // on "-", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        -13, // on "/", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        -13, // on "^", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 77
        0, // on "(", error
        0, // on ")", error
        -12, // on "*", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        -12, // on "+", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        0, // on ",", error
        -12, // on "-", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        -12, // on "/", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        -12, // on "^", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 78
        0, // on "(", error
        0, // on ")", error
        -15, // on "*", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        -15, // on "+", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        0, // on ",", error
        -15, // on "-", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        -15, // on "/", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        -15, // on "^", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 79
        0, // on "(", error
        -1, // on ")", reduce `Expr = Expr, "+", Factor => ActionFn(1);`
        55, // on "*", goto 54
        -1, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(1);`
        0, // on ",", error
        -1, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(1);`
        56, // on "/", goto 55
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 80
        0, // on "(", error
        -2, // on ")", reduce `Expr = Expr, "-", Factor => ActionFn(2);`
        55, // on "*", goto 54
        -2, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(2);`
        0, // on ",", error
        -2, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(2);`
        56, // on "/", goto 55
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 81
        0, // on "(", error
        -4, // on ")", reduce `Factor = Factor, "*", Power => ActionFn(4);`
        -4, // on "*", reduce `Factor = Factor, "*", Power => ActionFn(4);`
        -4, // on "+", reduce `Factor = Factor, "*", Power => ActionFn(4);`
        0, // on ",", error
        -4, // on "-", reduce `Factor = Factor, "*", Power => ActionFn(4);`
        -4, // on "/", reduce `Factor = Factor, "*", Power => ActionFn(4);`
        57, // on "^", goto 56
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 82
        0, // on "(", error
        -5, // on ")", reduce `Factor = Factor, "/", Power => ActionFn(5);`
        -5, // on "*", reduce `Factor = Factor, "/", Power => ActionFn(5);`
        -5, // on "+", reduce `Factor = Factor, "/", Power => ActionFn(5);`
        0, // on ",", error
        -5, // on "-", reduce `Factor = Factor, "/", Power => ActionFn(5);`
        -5, // on "/", reduce `Factor = Factor, "/", Power => ActionFn(5);`
        57, // on "^", goto 56
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 83
        0, // on "(", error
        -8, // on ")", reduce `Power = Power, "^", Term => ActionFn(7);`
        -8, // on "*", reduce `Power = Power, "^", Term => ActionFn(7);`
        -8, // on "+", reduce `Power = Power, "^", Term => ActionFn(7);`
        0, // on ",", error
        -8, // on "-", reduce `Power = Power, "^", Term => ActionFn(7);`
        -8, // on "/", reduce `Power = Power, "^", Term => ActionFn(7);`
        -8, // on "^", reduce `Power = Power, "^", Term => ActionFn(7);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 84
        0, // on "(", error
        -11, // on ")", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        -11, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        -11, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        0, // on ",", error
        -11, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        -11, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        -11, // on "^", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 85
        0, // on "(", error
        -14, // on ")", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        -14, // on "*", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        -14, // on "+", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        0, // on ",", error
        -14, // on "-", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        -14, // on "/", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        -14, // on "^", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 86
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 87
        0, // on "(", error
        -13, // on ")", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        -13, // on "*", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        -13, // on "+", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        0, // on ",", error
        -13, // on "-", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        -13, // on "/", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        -13, // on "^", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 88
        0, // on "(", error
        -12, // on ")", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        -12, // on "*", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        -12, // on "+", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        0, // on ",", error
        -12, // on "-", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        -12, // on "/", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        -12, // on "^", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 89
        0, // on "(", error
        -15, // on ")", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        -15, // on "*", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        -15, // on "+", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        0, // on ",", error
        -15, // on "-", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        -15, // on "/", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        -15, // on "^", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 90
        0, // on "(", error
        0, // on ")", error
        68, // on "*", goto 67
        -1, // on "+", reduce `Expr = Expr, "+", Factor => ActionFn(1);`
        -1, // on ",", reduce `Expr = Expr, "+", Factor => ActionFn(1);`
        -1, // on "-", reduce `Expr = Expr, "+", Factor => ActionFn(1);`
        69, // on "/", goto 68
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 91
        0, // on "(", error
        104, // on ")", goto 103
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 92
        0, // on "(", error
        0, // on ")", error
        68, // on "*", goto 67
        -2, // on "+", reduce `Expr = Expr, "-", Factor => ActionFn(2);`
        -2, // on ",", reduce `Expr = Expr, "-", Factor => ActionFn(2);`
        -2, // on "-", reduce `Expr = Expr, "-", Factor => ActionFn(2);`
        69, // on "/", goto 68
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 93
        0, // on "(", error
        0, // on ")", error
        -4, // on "*", reduce `Factor = Factor, "*", Power => ActionFn(4);`
        -4, // on "+", reduce `Factor = Factor, "*", Power => ActionFn(4);`
        -4, // on ",", reduce `Factor = Factor, "*", Power => ActionFn(4);`
        -4, // on "-", reduce `Factor = Factor, "*", Power => ActionFn(4);`
        -4, // on "/", reduce `Factor = Factor, "*", Power => ActionFn(4);`
        70, // on "^", goto 69
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 94
        0, // on "(", error
        0, // on ")", error
        -5, // on "*", reduce `Factor = Factor, "/", Power => ActionFn(5);`
        -5, // on "+", reduce `Factor = Factor, "/", Power => ActionFn(5);`
        -5, // on ",", reduce `Factor = Factor, "/", Power => ActionFn(5);`
        -5, // on "-", reduce `Factor = Factor, "/", Power => ActionFn(5);`
        -5, // on "/", reduce `Factor = Factor, "/", Power => ActionFn(5);`
        70, // on "^", goto 69
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 95
        0, // on "(", error
        0, // on ")", error
        -8, // on "*", reduce `Power = Power, "^", Term => ActionFn(7);`
        -8, // on "+", reduce `Power = Power, "^", Term => ActionFn(7);`
        -8, // on ",", reduce `Power = Power, "^", Term => ActionFn(7);`
        -8, // on "-", reduce `Power = Power, "^", Term => ActionFn(7);`
        -8, // on "/", reduce `Power = Power, "^", Term => ActionFn(7);`
        -8, // on "^", reduce `Power = Power, "^", Term => ActionFn(7);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 96
        0, // on "(", error
        0, // on ")", error
        -11, // on "*", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        -11, // on "+", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        -11, // on ",", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        -11, // on "-", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        -11, // on "/", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        -11, // on "^", reduce `Term = "(", Expr, ")" => ActionFn(10);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 97
        0, // on "(", error
        0, // on ")", error
        -14, // on "*", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        -14, // on "+", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        -14, // on ",", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        -14, // on "-", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        -14, // on "/", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        -14, // on "^", reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 98
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "^", error
        25, // on "cos(", goto 24
        26, // on "hypot(", goto 25
        27, // on "sin(", goto 26
        28, // on "sqrt(", goto 27
        29, // on "tan(", goto 28
        30, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, goto 29
        // State 99
        0, // on "(", error
        0, // on ")", error
        -13, // on "*", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        -13, // on "+", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        -13, // on ",", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        -13, // on "-", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        -13, // on "/", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        -13, // on "^", reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 100
        0, // on "(", error
        0, // on ")", error
        -12, // on "*", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        -12, // on "+", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        -12, // on ",", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        -12, // on "-", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        -12, // on "/", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        -12, // on "^", reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 101
        0, // on "(", error
        0, // on ")", error
        -15, // on "*", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        -15, // on "+", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        -15, // on ",", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        -15, // on "-", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        -15, // on "/", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        -15, // on "^", reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 102
        0, // on "(", error
        106, // on ")", goto 105
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 103
        0, // on "(", error
        0, // on ")", error
        -16, // on "*", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        -16, // on "+", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        0, // on ",", error
        -16, // on "-", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        -16, // on "/", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        -16, // on "^", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 104
        0, // on "(", error
        107, // on ")", goto 106
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "^", error
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 105
        0, // on "(", error
        -16, // on ")", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        -16, // on "*", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        -16, // on "+", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        0, // on ",", error
        -16, // on "-", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        -16, // on "/", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        -16, // on "^", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
        // State 106
        0, // on "(", error
        0, // on ")", error
        -16, // on "*", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        -16, // on "+", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        -16, // on ",", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        -16, // on "-", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        -16, // on "/", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        -16, // on "^", reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        0, // on "cos(", error
        0, // on "hypot(", error
        0, // on "sin(", error
        0, // on "sqrt(", error
        0, // on "tan(", error
        0, // on r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -17, // on EOF, reduce `__Expr = Expr => ActionFn(0);`
        -3, // on EOF, reduce `Expr = Factor => ActionFn(3);`
        -10, // on EOF, reduce `Term = Num => ActionFn(9);`
        -6, // on EOF, reduce `Factor = Power => ActionFn(6);`
        -9, // on EOF, reduce `Power = Term => ActionFn(8);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);`
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
        -1, // on EOF, reduce `Expr = Expr, "+", Factor => ActionFn(1);`
        -2, // on EOF, reduce `Expr = Expr, "-", Factor => ActionFn(2);`
        -4, // on EOF, reduce `Factor = Factor, "*", Power => ActionFn(4);`
        -5, // on EOF, reduce `Factor = Factor, "/", Power => ActionFn(5);`
        -8, // on EOF, reduce `Power = Power, "^", Term => ActionFn(7);`
        -11, // on EOF, reduce `Term = "(", Expr, ")" => ActionFn(10);`
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
        -14, // on EOF, reduce `Term = "cos(", Expr, ")" => ActionFn(13);`
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
        -13, // on EOF, reduce `Term = "sin(", Expr, ")" => ActionFn(12);`
        -12, // on EOF, reduce `Term = "sqrt(", Expr, ")" => ActionFn(11);`
        -15, // on EOF, reduce `Term = "tan(", Expr, ")" => ActionFn(14);`
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
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -16, // on EOF, reduce `Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Expr, goto 1
        3, // on Factor, goto 2
        4, // on Num, goto 3
        5, // on Power, goto 4
        6, // on Term, goto 5
        0, // on __Expr, error
        // State 1
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 2
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 3
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 4
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 5
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 6
        19, // on Expr, goto 18
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 7
        31, // on Expr, goto 30
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 8
        32, // on Expr, goto 31
        33, // on Factor, goto 32
        34, // on Num, goto 33
        35, // on Power, goto 34
        36, // on Term, goto 35
        0, // on __Expr, error
        // State 9
        44, // on Expr, goto 43
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 10
        45, // on Expr, goto 44
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 11
        46, // on Expr, goto 45
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 12
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 13
        0, // on Expr, error
        47, // on Factor, goto 46
        4, // on Num, goto 3
        5, // on Power, goto 4
        6, // on Term, goto 5
        0, // on __Expr, error
        // State 14
        0, // on Expr, error
        48, // on Factor, goto 47
        4, // on Num, goto 3
        5, // on Power, goto 4
        6, // on Term, goto 5
        0, // on __Expr, error
        // State 15
        0, // on Expr, error
        0, // on Factor, error
        4, // on Num, goto 3
        49, // on Power, goto 48
        6, // on Term, goto 5
        0, // on __Expr, error
        // State 16
        0, // on Expr, error
        0, // on Factor, error
        4, // on Num, goto 3
        50, // on Power, goto 49
        6, // on Term, goto 5
        0, // on __Expr, error
        // State 17
        0, // on Expr, error
        0, // on Factor, error
        4, // on Num, goto 3
        0, // on Power, error
        51, // on Term, goto 50
        0, // on __Expr, error
        // State 18
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 19
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 20
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 21
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 22
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 23
        58, // on Expr, goto 57
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 24
        59, // on Expr, goto 58
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 25
        60, // on Expr, goto 59
        33, // on Factor, goto 32
        34, // on Num, goto 33
        35, // on Power, goto 34
        36, // on Term, goto 35
        0, // on __Expr, error
        // State 26
        61, // on Expr, goto 60
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 27
        62, // on Expr, goto 61
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 28
        63, // on Expr, goto 62
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 29
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 30
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 31
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 32
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 33
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 34
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 35
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 36
        71, // on Expr, goto 70
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 37
        72, // on Expr, goto 71
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 38
        73, // on Expr, goto 72
        33, // on Factor, goto 32
        34, // on Num, goto 33
        35, // on Power, goto 34
        36, // on Term, goto 35
        0, // on __Expr, error
        // State 39
        74, // on Expr, goto 73
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 40
        75, // on Expr, goto 74
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 41
        76, // on Expr, goto 75
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 42
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 43
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 44
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 45
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 46
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 47
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 48
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 49
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 50
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 51
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 52
        0, // on Expr, error
        80, // on Factor, goto 79
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 53
        0, // on Expr, error
        81, // on Factor, goto 80
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 54
        0, // on Expr, error
        0, // on Factor, error
        21, // on Num, goto 20
        82, // on Power, goto 81
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 55
        0, // on Expr, error
        0, // on Factor, error
        21, // on Num, goto 20
        83, // on Power, goto 82
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 56
        0, // on Expr, error
        0, // on Factor, error
        21, // on Num, goto 20
        0, // on Power, error
        84, // on Term, goto 83
        0, // on __Expr, error
        // State 57
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 58
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 59
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 60
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 61
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 62
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 63
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 64
        0, // on Expr, error
        91, // on Factor, goto 90
        34, // on Num, goto 33
        35, // on Power, goto 34
        36, // on Term, goto 35
        0, // on __Expr, error
        // State 65
        92, // on Expr, goto 91
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 66
        0, // on Expr, error
        93, // on Factor, goto 92
        34, // on Num, goto 33
        35, // on Power, goto 34
        36, // on Term, goto 35
        0, // on __Expr, error
        // State 67
        0, // on Expr, error
        0, // on Factor, error
        34, // on Num, goto 33
        94, // on Power, goto 93
        36, // on Term, goto 35
        0, // on __Expr, error
        // State 68
        0, // on Expr, error
        0, // on Factor, error
        34, // on Num, goto 33
        95, // on Power, goto 94
        36, // on Term, goto 35
        0, // on __Expr, error
        // State 69
        0, // on Expr, error
        0, // on Factor, error
        34, // on Num, goto 33
        0, // on Power, error
        96, // on Term, goto 95
        0, // on __Expr, error
        // State 70
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 71
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 72
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 73
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 74
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 75
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 76
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 77
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 78
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 79
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 80
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 81
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 82
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 83
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 84
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 85
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 86
        103, // on Expr, goto 102
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 87
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 88
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 89
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 90
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 91
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 92
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 93
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 94
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 95
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 96
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 97
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 98
        105, // on Expr, goto 104
        20, // on Factor, goto 19
        21, // on Num, goto 20
        22, // on Power, goto 21
        23, // on Term, goto 22
        0, // on __Expr, error
        // State 99
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 100
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 101
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 102
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 103
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 104
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 105
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
        // State 106
        0, // on Expr, error
        0, // on Factor, error
        0, // on Num, error
        0, // on Power, error
        0, // on Term, error
        0, // on __Expr, error
    ];
    pub fn parse_Expr<
        'input,
    >(
        input: &'input str,
    ) -> Result<f64, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
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
                (_, (11, _), _) if true => 11,
                (_, (12, _), _) if true => 12,
                (_, (13, _), _) if true => 13,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 14 + __integer];
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
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2c_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5e_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22cos_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22hypot_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22sin_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22sqrt_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22tan_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Termr_23_22_28_28_5c_5cd_2b_28_5c_5c_2e_5c_5cd_2a_29_3f_29_7c_28_5c_5c_2e_5c_5cd_2b_29_29_22_23(__tok0),
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
    ) -> Option<Result<f64,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Expr = Expr, "+", Factor => ActionFn(1);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            2 => {
                // Expr = Expr, "-", Factor => ActionFn(2);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            3 => {
                // Expr = Factor => ActionFn(3);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            4 => {
                // Factor = Factor, "*", Power => ActionFn(4);
                let __sym2 = __pop_NtPower(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                1
            }
            5 => {
                // Factor = Factor, "/", Power => ActionFn(5);
                let __sym2 = __pop_NtPower(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                1
            }
            6 => {
                // Factor = Power => ActionFn(6);
                let __sym0 = __pop_NtPower(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                1
            }
            7 => {
                // Num = r#"((\\d+(\\.\\d*)?)|(\\.\\d+))"# => ActionFn(16);
                let __sym0 = __pop_Termr_23_22_28_28_5c_5cd_2b_28_5c_5c_2e_5c_5cd_2a_29_3f_29_7c_28_5c_5c_2e_5c_5cd_2b_29_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                2
            }
            8 => {
                // Power = Power, "^", Term => ActionFn(7);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_Term_22_5e_22(__symbols);
                let __sym0 = __pop_NtPower(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPower(__nt), __end));
                3
            }
            9 => {
                // Power = Term => ActionFn(8);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPower(__nt), __end));
                3
            }
            10 => {
                // Term = Num => ActionFn(9);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                4
            }
            11 => {
                // Term = "(", Expr, ")" => ActionFn(10);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                4
            }
            12 => {
                // Term = "sqrt(", Expr, ")" => ActionFn(11);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22sqrt_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                4
            }
            13 => {
                // Term = "sin(", Expr, ")" => ActionFn(12);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22sin_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                4
            }
            14 => {
                // Term = "cos(", Expr, ")" => ActionFn(13);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22cos_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                4
            }
            15 => {
                // Term = "tan(", Expr, ")" => ActionFn(14);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22tan_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                4
            }
            16 => {
                // Term = "hypot(", Expr, ",", Expr, ")" => ActionFn(15);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtExpr(__symbols);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22hypot_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action15::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                4
            }
            17 => {
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
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22cos_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22cos_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22hypot_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22hypot_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22sin_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22sin_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22sqrt_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22sqrt_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22tan_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22tan_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_28_5c_5cd_2b_28_5c_5c_2e_5c_5cd_2a_29_3f_29_7c_28_5c_5c_2e_5c_5cd_2b_29_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_28_5c_5cd_2b_28_5c_5c_2e_5c_5cd_2a_29_3f_29_7c_28_5c_5c_2e_5c_5cd_2b_29_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPower<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPower(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
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
                        42 => /* '*' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        44 => /* ',' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        46 => /* '.' */ {
                            __current_state = 7;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        94 => /* '^' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_state = 11;
                            continue;
                        }
                        104 => /* 'h' */ {
                            __current_state = 12;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_state = 13;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_state = 14;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
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
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        46 => /* '.' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 9;
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 18;
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
                        121 => /* 'y' */ {
                            __current_state = 19;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 20;
                            continue;
                        }
                        113 => /* 'q' */ {
                            __current_state = 21;
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
                        97 => /* 'a' */ {
                            __current_state = 22;
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
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
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 17;
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
                        115 => /* 's' */ {
                            __current_state = 23;
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
                        112 => /* 'p' */ {
                            __current_state = 24;
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
                        110 => /* 'n' */ {
                            __current_state = 25;
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
                        114 => /* 'r' */ {
                            __current_state = 26;
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
                        110 => /* 'n' */ {
                            __current_state = 27;
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
                        40 => /* '(' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 28;
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
                        111 => /* 'o' */ {
                            __current_state = 29;
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
                        40 => /* '(' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 30;
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
                        116 => /* 't' */ {
                            __current_state = 31;
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
                        40 => /* '(' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 33;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        40 => /* '(' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        40 => /* '(' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 35;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
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
    (_, __0, _): (usize, f64, usize),
) -> f64
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, f64, usize),
) -> f64
{
    l + r
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, f64, usize),
) -> f64
{
    l - r
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, f64, usize),
) -> f64
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, f64, usize),
) -> f64
{
    l * r
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, f64, usize),
) -> f64
{
    l / r
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, f64, usize),
) -> f64
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, b, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
) -> f64
{
    b.powf(e)
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, f64, usize),
) -> f64
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, f64, usize),
) -> f64
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    e.sqrt()
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    e.sin()
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    (f64::to_radians(90.0) - e).sin()
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    e.tan()
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, a, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, b, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    f64::hypot(a, b)
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> f64
{
    f64::from_str(__0).unwrap()
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
