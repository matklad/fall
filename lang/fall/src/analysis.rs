use std::collections::HashSet;

use lazycell::LazyCell;

use fall_tree::{Node, AstNode};
use fall_tree::search::child_of_type;
use fall_tree::search::ast;

use {FallFile, SynRule, RefExpr, RefKind, CallKind, CallExpr, SYN_RULE};

pub struct Analysis<'f> {
    file: FallFile<'f>,

    used_rules: LazyCell<HashSet<Node<'f>>>
}

impl<'f> Analysis<'f> {
    pub fn new(file: FallFile) -> Analysis {
        Analysis {
            file,
            used_rules: LazyCell::new(),
        }
    }

    pub fn is_unused(&self, rule: SynRule<'f>) -> bool {
        self.unused_rules().contains(&rule.node())
    }

    fn unused_rules(&self) -> &HashSet<Node<'f>> {
        self.used_rules.borrow_with(|| {
            self.calculate_unused_rules()
        })
    }

    fn calculate_unused_rules(&self) -> HashSet<Node<'f>> {
        ast::descendants_of_type::<RefExpr>(self.file.node())
            .into_iter()
            .filter_map(|node| node.resolve())
            .filter_map(|r| match r {
                RefKind::RuleReference(rule) => Some(rule.node()),
                _ => None
            })
            .chain(
                ast::descendants_of_type::<CallExpr>(self.file.node())
                    .into_iter()
                    .filter_map(|call| call.kind().ok())
                    .filter_map(|kind| match kind {
                        CallKind::RuleCall(rule, ..) => Some(rule.node()),
                        _ => None,
                    })
            )
            .chain(child_of_type(self.file.node(), SYN_RULE).into_iter())
            .collect()
    }
}

