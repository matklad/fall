use fall_tree::{File, TextRange};
use fall_editor::actions::ActionResult;

mod swap_alternatives;
mod extract_rule;

pub const ACTIONS: &[(&str, fn(&File, TextRange, bool) -> Option<ActionResult>)] = &[
    ("Swap alternatives", |file, range, apply| swap_alternatives::swap_alternatives(file, range.start(), apply)),
    ("Extract rule", extract_rule::extract_rule),
];

