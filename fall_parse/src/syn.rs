pub struct Rule {
    pub name: &'static str,
    pub alts: &'static [Alt],
}

pub struct Alt {
    pub parts: &'static [Part],
    pub commit: Option<usize>,
}

pub enum Part {
    Rule(&'static str),
    Rep(Alt)
}