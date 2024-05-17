mod nfa;
mod regex_ast;
use anyhow::Result;
use lalrpop_util::lalrpop_mod;
use nfa::*;
use std::env;
use thiserror::Error;

lalrpop_mod!(pub regex_parser, "/regex_parser.rs");

fn main() {
    let re = "(b|b)*";
    let nfa = nfa_from_string(&re).unwrap();
    println!("{}", nfa);
}

fn check_ambig(nfa: &NFA) -> bool {
    let mut exp = false;

    dbg!(nfa.all_loops());
    for (state, input_loops) in nfa.all_loops() {
        for (input, cycle) in input_loops {
            if cycle.len() > 1 {
                println!(
                    "Cycle of length {} found for sub-input {:?} in state {}",
                    cycle.len(),
                    input,
                    state,
                );
                exp = true;
            }
        }
    }
    if exp {
        println!("Potential expontential ambiguity found.");
    }
    exp
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
    let mut nfas: Vec<(String, NFA, Vec<(String, bool)>)> = Vec::new();

    let re = "a".to_string();
    let nfa = nfa_from_string(&re).unwrap();
    let strings = vec![
        ("a".to_string(), true),
        ("b".to_string(), false),
        ("aa".to_string(), false),
    ];
    nfas.push((re, nfa, strings));

    //generate more tests
    let re = "a|b".to_string();
    let nfa = nfa_from_string(&re).unwrap();
    let strings = vec![
        ("a".to_string(), true),
        ("b".to_string(), true),
        ("c".to_string(), false),
        ("ab".to_string(), false),
    ];
    nfas.push((re, nfa, strings));

    let re = "a*".to_string();
    let nfa = nfa_from_string(&re).unwrap();
    let strings = vec![
        ("a".to_string(), true),
        ("aa".to_string(), true),
        ("b".to_string(), false),
        ("ab".to_string(), false),
    ];
    nfas.push((re, nfa, strings));

    let re = "a*|b".to_string();
    let nfa = nfa_from_string(&re).unwrap();
    let strings = vec![
        ("a".to_string(), true),
        ("aa".to_string(), true),
        ("b".to_string(), true),
        ("ab".to_string(), false),
    ];
    nfas.push((re, nfa, strings));

    let re = "(ab)*".to_string();
    let nfa = nfa_from_string(&re).unwrap();
    let strings = vec![
        ("ab".to_string(), true),
        ("abab".to_string(), true),
        ("a".to_string(), false),
        ("b".to_string(), false),
        ("".to_string(), true),
    ];
    nfas.push((re, nfa, strings));

    let re = "(ab)+".to_string();
    let nfa = nfa_from_string(&re).unwrap();
    let strings = vec![
        ("ab".to_string(), true),
        ("abab".to_string(), true),
        ("a".to_string(), false),
        ("b".to_string(), false),
        ("".to_string(), false),
    ];
    nfas.push((re, nfa, strings));

    let re = "a?".to_string();
    let nfa = nfa_from_string(&re).unwrap();
    let strings = vec![
        ("a".to_string(), true),
        ("".to_string(), true),
        ("aa".to_string(), false),
    ];
    nfas.push((re, nfa, strings));

    let re = "(a*b?)+".to_string();
    let nfa = nfa_from_string(&re).unwrap();
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
    let nfa = nfa_from_string(&re).unwrap();
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
    let nfa = nfa_from_string(&re).unwrap();
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

fn nfa_from_string(string: &str) -> Result<NFA> {
    let re = regex_parser::RegexParser::new()
        .parse(string)
        .map_err(|_| ParseError::InvalidRegex)?;
    let nfa = create_nfa(&re);
    Ok(nfa)
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Invalid regex")]
    InvalidRegex,
}
