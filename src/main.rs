mod nfa;
mod regex_ast;
use lalrpop_util::lalrpop_mod;
use nfa::*;

lalrpop_mod!(pub regex_parser, "/regex_parser.rs");

fn main() {
    let a: &str = r"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let re = regex_parser::RegexParser::new().parse(a).unwrap();
    dbg!(&re);
}
