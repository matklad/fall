#[derive(Debug)]
pub struct Grammar {
    pub node_types: Vec<String>,
    pub lex_rules: Vec<LexRule>,
    pub syn_rules: Vec<SynRule>,
}

#[derive(Debug)]
pub struct LexRule {
    pub ty: String,
    pub re: String,
    pub f: Option<String>,
}

#[derive(Debug)]
pub struct SynRule {
    pub name: String,
    pub alts: Vec<Alt>,
}

#[derive(Debug)]
pub struct Alt {
    pub parts: Vec<Part>,
    pub commit: Option<usize>,
}

#[derive(Debug)]
pub enum Part {
    Rule(String),
    Rep(Alt)
}
