mod regex_ast;
use crate::regex_ast::CharSet;
use lalrpop_util::lalrpop_mod;
use regex_ast::*;

lalrpop_mod!(pub regex_parser, "/regex_parser.rs");

fn main() {
    let a: &str = r"go+gle";
    println!("{}", a);
    let re = regex_parser::RegexParser::new().parse(a).unwrap();
    dbg!(re);
}
