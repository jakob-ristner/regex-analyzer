#[derive(Debug, Clone)]
pub enum RegexAst {
    Epsilon,
    Any,
    CharClass(bool, Vec<char>),
    Literal(char),
    Concat(Box<RegexAst>, Box<RegexAst>),
    Or(Box<RegexAst>, Box<RegexAst>),
    Star(Box<RegexAst>),
}
