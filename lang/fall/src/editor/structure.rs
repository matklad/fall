use fall_tree::{File, AstNode};
use fall_tree::visitor::{visitor, process_subtree_bottom_up};
use fall_editor::FileStructureNode;

use syntax::{SynRule, TokenizerDef, AstDef};


pub(crate) fn structure(file: &File) -> Vec<FileStructureNode> {
    process_subtree_bottom_up(
        file.root(),
        visitor(Vec::new())
            .visit::<SynRule, _>(|rule, nodes| {
                if let Some(name) = rule.name() {
                    nodes.push(FileStructureNode {
                        name: name.to_string(),
                        range: rule.node().range(),
                        children: Vec::new(),
                    })
                }
            })
            .visit::<TokenizerDef, _>(|tokenizer, nodes| {
                nodes.push(FileStructureNode {
                    name: "tokenizer".to_owned(),
                    range: tokenizer.node().range(),
                    children: Vec::new(),
                })
            })
            .visit::<AstDef, _>(|ast, nodes| {
                nodes.push(FileStructureNode {
                    name: "ast".to_owned(),
                    range: ast.node().range(),
                    children: Vec::new(),
                })
            }),
    )
}


#[test]
fn test_structure() {
    let file = ::parse(r#"
tokenizer { number r"\d+"}
pub rule foo { bar }
rule bar { number }
ast {
  node foo { }
}
"#);
    let s = structure(&file);

    assert_eq!(
        format!("{:?}", s),
        r#"[FileStructureNode { name: "tokenizer", range: [1; 27), children: [] }, FileStructureNode { name: "foo", range: [28; 48), children: [] }, FileStructureNode { name: "bar", range: [49; 68), children: [] }, FileStructureNode { name: "ast", range: [69; 91), children: [] }]"#
    );
}
