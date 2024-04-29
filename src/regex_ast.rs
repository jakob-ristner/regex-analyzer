#[derive(Debug, Clone)]
pub enum RegexAst {
    Epsilon,
    Any,
    EOL,
    BOL,
    CharClass(CharSet),
    Literal(String),
    Concat(Box<RegexAst>, Box<RegexAst>),
    Or(Box<RegexAst>, Box<RegexAst>),
    Star(Box<RegexAst>),
}

#[derive(Debug, Clone)]
pub struct CharSet {
    pub chars: Vec<char>,
    pub complement: bool,
}

impl CharSet {
    pub fn add(&mut self, c: char) {
        self.chars.push(c);
    }

    pub fn add_range(&mut self, start: char, end: char) {
        for c in start..=end {
            self.chars.push(c);
        }
    }

    pub fn contains(&self, c: char) -> bool {
        self.chars.contains(&c) ^ self.complement
    }

    pub fn complement(&self) -> CharSet {
        CharSet {
            complement: !self.complement.clone(),
            chars: self.chars.clone(),
        }
    }
}
