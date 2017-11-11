use fall_tree::{File, TextRange, FileEdit, TextEdit};

mod swap_alternatives;
mod extract_rule;

pub fn context_actions(file: &File, range: TextRange) -> Vec<&'static str> {
    ACTIONS.iter()
        .filter(|action| action.apply(file, range).is_some())
        .map(|action| action.id())
        .collect()
}

pub fn apply_context_action(file: &File, range: TextRange, action_id: &str) -> TextEdit {
    let action = ACTIONS.iter().find(|action| action.id() == action_id).unwrap();
    action.apply(file, range).unwrap().into_text_edit()
}

const ACTIONS: &[&ContextAction] = &[
    &self::swap_alternatives::SwapAlternatives,
    &self::extract_rule::ExtractRule,
];

pub trait ContextAction {
    fn id(&self) -> &'static str;
    fn apply<'f>(&self, file: &'f File, range: TextRange) -> Option<FileEdit<'f>>;
}

#[cfg(test)]
fn check_context_action(
    available: &str,
    execute: &str,
    before: &str,
    after: &str
) {
    let (file, range) = ::test_util::parse_with_range(before);
    let actions = context_actions(&file, range);
    assert_eq!(
        format!("{:?}", actions),
        available
    );
    let edit = apply_context_action(&file, range, execute);
    let actual = edit.apply(file.text());
    ::fall_tree::test_util::report_diff(after.trim(), actual.as_slice().to_cow().trim())
}
