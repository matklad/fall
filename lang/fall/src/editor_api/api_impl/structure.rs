use fall_tree::File;
use fall_tree::visitor::{Visitor, NodeVisitor};

use ::*;
use editor_api::FileStructureNode;


pub fn structure(file: &File) -> Vec<FileStructureNode> {
    Visitor(Vec::new())
        .visit::<SynRule, _>(|nodes, rule| {
            if let Some(name) = rule.name() {
                nodes.push(FileStructureNode {
                    name: name.to_string(),
                    range: rule.node().range(),
                    children: Vec::new(),
                })
            }
        })
        .walk_recursively_children_first(file.root())
}


#[test]
fn test_structure() {
    let file = parse(r#"
tokenizer { number r"\d+"}
pub rule foo { bar }
rule bar { number }
"#);
    let s = structure(&file);

    assert_eq!(
        format!("{:?}", s),
        r#"[FileStructureNode { name: "foo", range: [28; 48), children: [] }, FileStructureNode { name: "bar", range: [49; 68), children: [] }]"#
    );
}
