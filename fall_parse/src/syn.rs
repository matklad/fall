use fall_tree::{NodeType, ERROR};
use TreeBuilder;

pub struct Parser<'r> {
    rules: &'r [SynRule],
}

pub struct SynRule {
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
    Rep(Alt, Option<&'static [NodeType]>, Option<&'static [NodeType]>),
    Opt(Alt)
}

impl<'r> Parser<'r> {
    pub fn new(rules: &[SynRule]) -> Parser {
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
            if !self.parse_part(p, b) {
                if i < commit {
                    return false;
                } else {
                    b.error();
                    break
                }
            }
        }
        true
    }

    fn parse_part(&self, part: &Part, b: &mut TreeBuilder) -> bool {
        match *part {
            Part::Token(ty) => b.try_eat(ty),
            Part::Rule(id) => self.parse_rule(&self.rules[id], b),
            Part::Rep(ref a, skip_until, _) => {
                'outer: loop {
                    let mut skipped = false;
                    'inner: loop {
                        let current = match b.current() {
                            None => {
                                if skipped {
                                    b.finish(Some(ERROR))
                                }
                                break 'outer
                            },
                            Some(c) => c,
                        };
                        let skip = match skip_until {
                            None => {
                                if skipped {
                                    b.finish(Some(ERROR))
                                }
                                break 'inner
                            },
                            Some(s) => s,
                        };
                        if skip.iter().any(|&it| it == current.ty) {
                            if skipped {
                                b.finish(Some(ERROR))
                            }
                            break 'inner
                        } else {
                            if !skipped {
                                b.start(Some(ERROR))
                            }
                            skipped = true;
                            b.bump();
                        }
                    }
                    if !self.parse_alt(a, b) {
                        break 'outer;
                    }
                }
                true
            }
            Part::Opt(ref a) => {
                self.parse_alt(a, b);
                true
            }
        }
    }

    fn parse_rule(&self, rule: &SynRule, b: &mut TreeBuilder) -> bool {
        b.start(rule.ty);

        for alt in rule.alts {
            if self.parse_alt(alt, b) {
                b.finish(rule.ty);
                return true;
            }
        }

        b.rollback(rule.ty);
        false
    }
}

