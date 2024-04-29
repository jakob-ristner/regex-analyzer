mod nfa;
mod regex_ast;
use lalrpop_util::lalrpop_mod;
use nfa::*;

lalrpop_mod!(pub regex_parser, "/regex_parser.rs");

fn main() {
    let mut vec = vec!['a', 'b', 'c', 'd'];
    let range = 'a'..='d';
    vec.extend(range);
    let a: &str = "(ab)? b* [a-d]+ ";
    let re = regex_parser::RegexParser::new().parse(a).unwrap();
    dbg!(re);
}
