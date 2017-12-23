use EditorFileImpl;
use fall_tree::{File, TextUnit, TextEdit};
use fall_tree::test_util;


pub const DEFAULT_ACTIONS: &[(&str, fn(&File, TextUnit, bool) -> Option<ActionResult>)] = &[

];

pub enum ActionResult {
    Available,
    Applied(TextEdit),
}

impl ActionResult {
    pub fn into_edit(self) -> TextEdit {
        match self {
            ActionResult::Available =>
                panic!("Context action should provide edit when apply is set to true"),
            ActionResult::Applied(edit) => edit,
        }
    }
}


pub fn check_context_action<E: EditorFileImpl>(
    action_id: &str,
    before: &str,
    after: &str
) {
    let (before, range) = test_util::extract_range(before, "^");
    let file = E::parse(&before);
    let actions = file.context_actions(range);
    if !actions.contains(&action_id) {
        panic!("Action `{}` is not avialable", action_id);
    }
    match file.apply_context_action(range, action_id) {
        None => panic!("Failed to apply `{}` action", action_id),
        Some(edit) => {
            let actual = edit.apply(file.file().text());
            test_util::report_diff(after.trim(), actual.as_text().to_cow().trim())
        }
    }
}

pub fn check_no_context_action<E: EditorFileImpl>(
    action_id: &str,
    text: &str,
) {
    let (before, range) = test_util::extract_range(text, "^");
    let file = E::parse(&before);
    let actions = file.context_actions(range);
    if actions.contains(&action_id) {
        panic!("Action `{}` is avialable", action_id);
    }
}
