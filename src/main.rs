mod nfa;
mod regex_ast;
use lalrpop_util::lalrpop_mod;
use nfa::*;
use std::collections::HashMap;

lalrpop_mod!(pub regex_parser, "/regex_parser.rs");

fn main() {
    let a: &str = r"";
    let re = regex_parser::RegexParser::new().parse(a).unwrap();
    dbg!(&re);

    let test_state = State::new(false);
    let test_state2 = State::new(false);
    let test_state3 = State::new(false);

    test_state
        .borrow_mut()
        .add_transition(vec!['a'], &test_state2);

    test_state2
        .borrow_mut()
        .add_transition(vec!['a'], &test_state);

    test_state
        .borrow_mut()
        .add_transition(vec!['a'], &test_state3);

    test_state3
        .borrow_mut()
        .add_transition(vec!['a'], &test_state);

    let loops = find_loops(
        test_state.clone(),
        test_state.clone(),
        &mut HashMap::new(),
        Vec::new(),
        Vec::new(),
    );
    dbg!(loops);
}

#[test]
pub fn test_regex_parser() {
    let examples = fill_nfas();
    for (original_regex, nfa, strings) in examples {
        for (string, expected) in strings {
            assert_eq!(
                nfa.run(&string),
                expected,
                "Regex: {}, String: {}, Expected: {}",
                original_regex,
                string,
                expected
            );
        }
    }
}

fn fill_nfas() -> Vec<(String, NFA, Vec<(String, bool)>)> {
    let mut nfas = Vec::new();

    let re = "a".to_string();
    let nfa = nfa_from_string(&re);
    let strings = vec![
        ("a".to_string(), true),
        ("b".to_string(), false),
        ("aa".to_string(), false),
    ];
    nfas.push((re, nfa, strings));

    //generate more tests
    let re = "a|b".to_string();
    let nfa = nfa_from_string(&re);
    let strings = vec![
        ("a".to_string(), true),
        ("b".to_string(), true),
        ("c".to_string(), false),
        ("ab".to_string(), false),
    ];
    nfas.push((re, nfa, strings));

    let re = "a*".to_string();
    let nfa = nfa_from_string(&re);
    let strings = vec![
        ("a".to_string(), true),
        ("aa".to_string(), true),
        ("b".to_string(), false),
        ("ab".to_string(), false),
    ];
    nfas.push((re, nfa, strings));

    let re = "a*|b".to_string();
    let nfa = nfa_from_string(&re);
    let strings = vec![
        ("a".to_string(), true),
        ("aa".to_string(), true),
        ("b".to_string(), true),
        ("ab".to_string(), false),
    ];
    nfas.push((re, nfa, strings));

    let re = "(ab)*".to_string();
    let nfa = nfa_from_string(&re);
    let strings = vec![
        ("ab".to_string(), true),
        ("abab".to_string(), true),
        ("a".to_string(), false),
        ("b".to_string(), false),
        ("".to_string(), true),
    ];
    nfas.push((re, nfa, strings));

    let re = "(ab)+".to_string();
    let nfa = nfa_from_string(&re);
    let strings = vec![
        ("ab".to_string(), true),
        ("abab".to_string(), true),
        ("a".to_string(), false),
        ("b".to_string(), false),
        ("".to_string(), false),
    ];
    nfas.push((re, nfa, strings));

    let re = "a?".to_string();
    let nfa = nfa_from_string(&re);
    let strings = vec![
        ("a".to_string(), true),
        ("".to_string(), true),
        ("aa".to_string(), false),
    ];
    nfas.push((re, nfa, strings));

    let re = "(a*b?)+".to_string();
    let nfa = nfa_from_string(&re);
    let strings = vec![
        ("a".to_string(), true),
        ("b".to_string(), true),
        ("ab".to_string(), true),
        ("ba".to_string(), true),
        ("aa".to_string(), true),
        ("bb".to_string(), true),
        ("aba".to_string(), true),
        ("bab".to_string(), true),
        ("a".to_string(), true),
        ("b".to_string(), true),
        ("".to_string(), true),
    ];
    nfas.push((re, nfa, strings));

    let re = "[a-v]".to_string();
    let nfa = nfa_from_string(&re);
    let strings = vec![
        ("a".to_string(), true),
        ("b".to_string(), true),
        ("v".to_string(), true),
        ("w".to_string(), false),
        ("z".to_string(), false),
    ];

    let re = "[^a-cA-C]".to_string();
    let nfa = nfa_from_string(&re);
    let strings = vec![
        ("a".to_string(), false),
        ("b".to_string(), false),
        ("c".to_string(), false),
        ("A".to_string(), false),
        ("B".to_string(), false),
        ("C".to_string(), false),
        ("d".to_string(), true),
        ("e".to_string(), true),
        ("f".to_string(), true),
        ("D".to_string(), true),
        ("E".to_string(), true),
        ("F".to_string(), true),
    ];
    nfas.push((re, nfa, strings));

    let re = "ab.+c".to_string();
    let nfa = nfa_from_string(&re);
    let strings = vec![
        ("abbbc".to_string(), true),
        ("abbc".to_string(), true),
        ("abalskjdcc".to_string(), true),
        ("abcalksjdcc".to_string(), true),
        ("ablkjlkj".to_string(), false),
        ("ac".to_string(), false),
        ("abc".to_string(), false),
    ];
    nfas.push((re, nfa, strings));

    nfas
}

fn nfa_from_string(string: &str) -> NFA {
    let re = regex_parser::RegexParser::new().parse(string).unwrap();
    let nfa = create_nfa(&re);
    nfa
}
