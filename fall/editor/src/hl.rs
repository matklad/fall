use fall_tree::{TextRange, NodeType, Node};

use fall_tree::visitor::{Visit, VisitorBuilder};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct HlTag(pub &'static str);

pub const ERROR: HlTag = HlTag("error");
pub const COMMENT: HlTag = HlTag("comment");
pub const STRING: HlTag = HlTag("string");
pub const KEYWORD: HlTag = HlTag("keyword");
pub const FUNCTION: HlTag = HlTag("function");
pub const PARAMETER: HlTag = HlTag("parameter");
pub const BUILTIN: HlTag = HlTag("builtin");
pub const ATTRIBUTE: HlTag = HlTag("attribute");
pub const LITERAL: HlTag = HlTag("literal");


pub type Highlights = Vec<(TextRange, HlTag)>;
pub type HlMap = &'static [(HlTag, &'static [NodeType])];

pub fn visitor<'f>(mapping: HlMap) -> VisitorBuilder<'f, Highlights, HlVisitor> {
    VisitorBuilder::new(Vec::new(), HlVisitor(mapping))
}

pub struct HlVisitor(HlMap);

impl<'f> Visit<'f> for HlVisitor {
    type Context = Highlights;

    fn visit(&mut self, ctx: &mut Highlights, node: Node<'f>) {
        for &(tag, tys) in self.0.iter() {
            for &ty in tys.iter() {
                if node.ty() == ty {
                    hl(node, tag, ctx);
                    return;
                }
            }
        }
    }
}

pub fn hl(node: Node, tag: HlTag, highlights: &mut Highlights) {
    highlights.push((node.range(), tag))
}

