use fall_tree::NodeType;
use TreeBuilder;

pub struct Parser<'r> {
    rules: &'r [Rule],
}

pub struct Rule {
    pub ty: Option<NodeType>,
    pub alts: &'static [Alt],
}

pub struct Alt {
    pub parts: &'static [Part],
    pub commit: Option<usize>,
}

pub enum Part {
    Rule(usize),
    Token(NodeType),
    Rep(Alt)
}

impl<'r> Parser<'r> {
    pub fn new(rules: &[Rule]) -> Parser {
        Parser { rules: rules }
    }

    pub fn parse(&self, b: &mut TreeBuilder) {
        let main_rule = &self.rules[0];
        for alt in main_rule.alts {
            if self.parse_alt(alt, b) {
                return
            }
        }
    }

    fn parse_alt(&self, alt: &Alt, b: &mut TreeBuilder) -> bool {
        let commit = alt.commit.unwrap_or(alt.parts.len());
        for (i, p) in alt.parts.iter().enumerate() {
            if !self.parse_part(p, b) && i < commit {
                return false;
            }
        }
        true
    }

    fn parse_part(&self, part: &Part, b: &mut TreeBuilder) -> bool {
        match *part {
            Part::Token(ty) => b.try_eat(ty),
            Part::Rule(id) => self.parse_rule(&self.rules[id], b),
            Part::Rep(ref a) => {
                while self.parse_alt(a, b) {}
                true
            }
        }
    }

    fn parse_rule(&self, rule: &Rule, b: &mut TreeBuilder) -> bool {
        if let Some(ty) = rule.ty {
            b.start(ty)
        }

        for alt in rule.alts {
            if self.parse_alt(alt, b) {
                if let Some(ty) = rule.ty {
                    b.finish(ty)
                }
                return true;
            }
        }

        if let Some(ty) = rule.ty {
            b.rollback(ty)
        }
        false
    }
}

