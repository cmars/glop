use std::str::FromStr;
use ast;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Glop {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_21_3d_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3d_3d_22(&'input str),
        Term_22_5c_5c_5c_22_22(&'input str),
        Term_22acknowledge_22(&'input str),
        Term_22isset_22(&'input str),
        Term_22match_22(&'input str),
        Term_22message_22(&'input str),
        Term_22set_22(&'input str),
        Term_22shell_22(&'input str),
        Term_22unset_22(&'input str),
        Term_22_7b_22(&'input str),
        Term_22_7d_22(&'input str),
        Termr_23_22_5ba_2dz_5d_5ba_2dz0_2d9___5d_2b_22_23(&'input str),
        Termr_23_22_5c_5cd_2b_22_23(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
        NtAction(Box<ast::Action>),
        NtActions(Vec<Box<ast::Action>>),
        NtCmpOp(ast::CmpOpcode),
        NtCondition(Box<ast::Condition>),
        NtConditions(Vec<Box<ast::Condition>>),
        NtGlop(Vec<Box<ast::Match>>),
        NtIdentifier(()),
        NtMatch(Box<ast::Match>),
        NtUnaryFunction(Box<ast::Condition>),
        NtValue(()),
        Nt____Glop(Vec<Box<ast::Match>>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 12, 0, 0, 0, 0, 0, 13, 0, 0,
        // State 6
        0, 0, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 14, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        17, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, -10, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0,
        // State 12
        -15, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 12, 0, 0, 0, 0, 0, 13, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0,
        // State 18
        0, 0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, -15, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 29, 30, 31, 0, 0, 0, 0, 0,
        // State 22
        0, 0, -11, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, -9, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0,
        // State 27
        0, 0, 0, 0, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 41, 42, 43, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0,
        // State 36
        0, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0,
        // State 37
        0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0,
        // State 43
        0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0,
        // State 44
        0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0,
        // State 50
        0, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0,
        // State 53
        0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -20,
        -14,
        0,
        -13,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -16,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 7, 8, 0, 9, 0, 10, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 23, 0, 0, 9, 0, 10, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        26, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 34, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0,
        // State 31
        39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 51, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    pub fn parse_Glop<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<Box<ast::Match>>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
                (13, _) if true => 13,
                (14, _) if true => 14,
                (15, _) if true => 15,
                (16, _) if true => 16,
                (17, _) if true => 17,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_21_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2c_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_3b_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_3d_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5c_5c_5c_22_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22acknowledge_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22isset_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22match_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22message_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22set_22(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22shell_22(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22unset_22(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_7b_22(__tok0),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22_7d_22(__tok0),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Termr_23_22_5ba_2dz_5d_5ba_2dz0_2d9___5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Termr_23_22_5c_5cd_2b_22_23(__tok0),
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
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                };
                return Err(__error);
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
    ) -> Option<Result<Vec<Box<ast::Match>>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Action = "set", Identifier, Value => ActionFn(16);
                let __sym2 = __pop_NtValue(__symbols);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22set_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAction(__nt), __end));
                0
            }
            2 => {
                // Action = "unset", Identifier => ActionFn(17);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22unset_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAction(__nt), __end));
                0
            }
            3 => {
                // Action = "acknowledge" => ActionFn(18);
                let __sym0 = __pop_Term_22acknowledge_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAction(__nt), __end));
                0
            }
            4 => {
                // Action = "shell", "\\\"", Value, "\\\"" => ActionFn(19);
                let __sym3 = __pop_Term_22_5c_5c_5c_22_22(__symbols);
                let __sym2 = __pop_NtValue(__symbols);
                let __sym1 = __pop_Term_22_5c_5c_5c_22_22(__symbols);
                let __sym0 = __pop_Term_22shell_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action19::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtAction(__nt), __end));
                0
            }
            5 => {
                // Actions = Actions, ";", Action, ";" => ActionFn(14);
                let __sym3 = __pop_Term_22_3b_22(__symbols);
                let __sym2 = __pop_NtAction(__symbols);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtActions(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtActions(__nt), __end));
                1
            }
            6 => {
                // Actions = Action => ActionFn(15);
                let __sym0 = __pop_NtAction(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtActions(__nt), __end));
                1
            }
            7 => {
                // CmpOp = "==" => ActionFn(10);
                let __sym0 = __pop_Term_22_3d_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCmpOp(__nt), __end));
                2
            }
            8 => {
                // CmpOp = "!=" => ActionFn(11);
                let __sym0 = __pop_Term_22_21_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCmpOp(__nt), __end));
                2
            }
            9 => {
                // Condition = Identifier, CmpOp, Value => ActionFn(6);
                let __sym2 = __pop_NtValue(__symbols);
                let __sym1 = __pop_NtCmpOp(__symbols);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtCondition(__nt), __end));
                3
            }
            10 => {
                // Condition = UnaryFunction => ActionFn(7);
                let __sym0 = __pop_NtUnaryFunction(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCondition(__nt), __end));
                3
            }
            11 => {
                // Conditions = Conditions, ",", Condition => ActionFn(4);
                let __sym2 = __pop_NtCondition(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtConditions(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtConditions(__nt), __end));
                4
            }
            12 => {
                // Conditions = Condition => ActionFn(5);
                let __sym0 = __pop_NtCondition(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtConditions(__nt), __end));
                4
            }
            13 => {
                // Glop = Glop, Match => ActionFn(1);
                let __sym1 = __pop_NtMatch(__symbols);
                let __sym0 = __pop_NtGlop(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action1::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtGlop(__nt), __end));
                5
            }
            14 => {
                // Glop = Match => ActionFn(2);
                let __sym0 = __pop_NtMatch(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtGlop(__nt), __end));
                5
            }
            15 => {
                // Identifier = r#"[a-z][a-z0-9_]+"# => ActionFn(8);
                let __sym0 = __pop_Termr_23_22_5ba_2dz_5d_5ba_2dz0_2d9___5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdentifier(__nt), __end));
                6
            }
            16 => {
                // Match = "match", "(", Conditions, ")", "{", Actions, "}" => ActionFn(3);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_NtActions(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtConditions(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22match_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtMatch(__nt), __end));
                7
            }
            17 => {
                // UnaryFunction = "message", Identifier => ActionFn(12);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22message_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtUnaryFunction(__nt), __end));
                8
            }
            18 => {
                // UnaryFunction = "isset", Identifier => ActionFn(13);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22isset_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtUnaryFunction(__nt), __end));
                8
            }
            19 => {
                // Value = r#"\\d+"# => ActionFn(9);
                let __sym0 = __pop_Termr_23_22_5c_5cd_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValue(__nt), __end));
                9
            }
            20 => {
                // __Glop = Glop => ActionFn(0);
                let __sym0 = __pop_NtGlop(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 11 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_21_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_21_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_5c_5c_22_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_5c_5c_22_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22acknowledge_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22acknowledge_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22isset_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22isset_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22match_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22match_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22message_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22message_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22set_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22set_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22shell_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22shell_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22unset_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22unset_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dz_5d_5ba_2dz0_2d9___5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dz_5d_5ba_2dz0_2d9___5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5cd_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5cd_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAction<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<ast::Action>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAction(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtActions<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<ast::Action>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtActions(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCmpOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::CmpOpcode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCmpOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCondition<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<ast::Condition>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCondition(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtConditions<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<ast::Condition>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtConditions(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtGlop<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<ast::Match>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtGlop(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdentifier<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdentifier(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMatch<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<ast::Match>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMatch(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtUnaryFunction<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<ast::Condition>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtUnaryFunction(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtValue<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtValue(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Glop<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<ast::Match>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Glop(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Glop::parse_Glop;
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
                        33 => /* '!' */ {
                            __current_state = 1;
                            continue;
                        }
                        40 => /* '(' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        44 => /* ',' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        59 => /* ';' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        61 => /* '=' */ {
                            __current_state = 7;
                            continue;
                        }
                        92 => /* '\\' */ {
                            __current_state = 8;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_state = 9;
                            continue;
                        }
                        98 ... 104 => {
                            __current_state = 10;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_state = 11;
                            continue;
                        }
                        106 ... 108 => {
                            __current_state = 10;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_state = 12;
                            continue;
                        }
                        110 ... 114 => {
                            __current_state = 10;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_state = 13;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_state = 10;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_state = 14;
                            continue;
                        }
                        118 ... 122 => {
                            __current_state = 10;
                            continue;
                        }
                        123 => /* '{' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        125 => /* '}' */ {
                            __current_match = Some((15, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        61 => /* '=' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 18;
                            continue;
                        }
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
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
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
                        61 => /* '=' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 20;
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
                        34 => /* '\"' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 21;
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
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 98 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        100 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        98 ... 100 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        102 ... 103 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        104 => /* 'h' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        105 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 19;
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
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
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 106 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        107 => /* 'k' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 31;
                            continue;
                        }
                        108 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
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
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
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
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 33;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
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
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
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
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 35;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
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
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 36;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 37;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
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
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 38;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
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
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 39;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 98 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        100 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
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
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 41;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 42;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                37 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 43;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                38 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 44;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                39 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 45;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                40 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 103 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        104 => /* 'h' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        105 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                41 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 47;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                42 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 48;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                43 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 49;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                44 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 118 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        119 => /* 'w' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 50;
                            continue;
                        }
                        120 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                45 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                46 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                47 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 102 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        103 => /* 'g' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 51;
                            continue;
                        }
                        104 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                48 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                49 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                50 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 52;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                51 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 53;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                52 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 54;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                53 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                54 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 99 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 55;
                            continue;
                        }
                        101 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                55 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 102 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        103 => /* 'g' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 56;
                            continue;
                        }
                        104 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                56 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 57;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                57 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((16, __index + __ch.len_utf8()));
                            __current_state = 30;
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
    (_, __0, _): (usize, Vec<Box<ast::Match>>, usize),
) -> Vec<Box<ast::Match>>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Box<ast::Match>>, usize),
    (_, __1, _): (usize, Box<ast::Match>, usize),
) -> Vec<Box<ast::Match>>
{
    ...
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<ast::Match>, usize),
) -> Vec<Box<ast::Match>>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, Vec<Box<ast::Condition>>, usize),
    (_, __3, _): (usize, &'input str, usize),
    (_, __4, _): (usize, &'input str, usize),
    (_, __5, _): (usize, Vec<Box<ast::Action>>, usize),
    (_, __6, _): (usize, &'input str, usize),
) -> Box<ast::Match>
{
    (__0, __1, __2, __3, __4, __5, __6)
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Box<ast::Condition>>, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, Box<ast::Condition>, usize),
) -> Vec<Box<ast::Condition>>
{
    ...
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<ast::Condition>, usize),
) -> Vec<Box<ast::Condition>>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (), usize),
    (_, __1, _): (usize, ast::CmpOpcode, usize),
    (_, __2, _): (usize, (), usize),
) -> Box<ast::Condition>
{
    Box::new(ast::Condition::Cmp(__0, __1, __2))
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<ast::Condition>, usize),
) -> Box<ast::Condition>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::CmpOpcode
{
    ast::CmpOpcode::Equal
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::CmpOpcode
{
    ast::CmpOpcode::NotEqual
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, (), usize),
) -> Box<ast::Condition>
{
    Box::new(ast::Condition::Message(__0, __1))
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, (), usize),
) -> Box<ast::Condition>
{
    Box::new(ast::Condition::Defined(__0, __1))
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Box<ast::Action>>, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, Box<ast::Action>, usize),
    (_, __3, _): (usize, &'input str, usize),
) -> Vec<Box<ast::Action>>
{
    ...
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<ast::Action>, usize),
) -> Vec<Box<ast::Action>>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, (), usize),
    (_, __2, _): (usize, (), usize),
) -> Box<ast::Action>
{
    Box::new(ast::Action::SetVar(__0, __1, __2))
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, (), usize),
) -> Box<ast::Action>
{
    Box::new(ast::Action::UnsetVar(__0, __1))
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Box<ast::Action>
{
    Box::new(ast::Action::Acknowledge)
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, (), usize),
    (_, __3, _): (usize, &'input str, usize),
) -> Box<ast::Action>
{
    Box::new(ast::Action::Shell(__0, __1, __2, __3))
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
