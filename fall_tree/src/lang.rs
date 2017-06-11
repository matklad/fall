use std::sync::Arc;
use {File, NodeType, NodeTypeInfo, FileStats, INode, TextRange, TextUnit, Edit};

#[derive(Clone)]
pub struct Language {
    imp: Arc<LanguageImpl>
}

impl Language {
    pub fn new<I: LanguageImpl>(imp: I) -> Language {
        Language { imp: Arc::new(imp) }
    }

    pub fn parse(&self, text: String) -> File {
        let (stats, inode) = self.imp.parse(&text);
        File::new(self.clone(), text, stats, inode)
    }

    pub fn reparse(&self, file: &File, edit: &Edit) -> File {
        let before = file.text().slice(TextRange::from_to(TextUnit::zero(), edit.delete.start()));
        let after = file.text().slice(TextRange::from_to(edit.delete.end(), file.text().len()));
        let new_text = before.to_string() + &edit.insert + &after.to_string();
        let (stats, inode) = go(self, &new_text, TextUnit::zero(), &file.inode(), edit).unwrap_or_else(|| {
            self.imp.parse(&new_text)
        });

        let result = File::new(self.clone(), new_text, stats, inode);
        return result;

        fn go(lang: &Language, text: &str, node_offset: TextUnit, node: &INode, edit: &Edit) -> Option<(FileStats, INode)> {
            let node_range = TextRange::from_len(node_offset, node.len());
            if !edit.delete.is_subrange_of(node_range) {
                return None;
            }
            if let Some(region) = node.reparse_region() {
                let region_range = region.range.shift_right(node_offset);
                let range_after_edit = TextRange::from_len(
                    region_range.start(),
                    region_range.len() - edit.delete.len() + TextUnit::from_usize(edit.insert.len())
                );
                if edit.delete.is_subrange_of(region_range) {
                    let text = &text[range_after_edit];
                    if let Some((mut stats, inodes)) = lang.imp.reparse(text, region.parser_id) {
                        let mut old_children = Vec::new();
                        let mut result = node.clone();
                        ::std::mem::swap(&mut old_children, result.children_mut());
                        let mut child_offset = TextUnit::zero();
                        let mut old_children = old_children.into_iter();
                        while child_offset < region.range.start() {
                            let child = old_children.next().unwrap();
                            child_offset += child.len();
                            result.children_mut().push(child)
                        }
                        result.children_mut().extend(inodes);
                        while child_offset < region.range.end() {
                            let child = old_children.next().unwrap();
                            child_offset += child.len();
                        }
                        result.children_mut().extend(old_children);
                        let mut new_region = region;
                        new_region.range = TextRange::from_len(
                            region.range.start(),
                            region.range.len() - edit.delete.len() + TextUnit::from_usize(edit.insert.len())
                        );
                        result.set_reparse_region(new_region);
                        stats.reparsed_region = range_after_edit;
                        return Some((stats, result))
                    }
                }
            }
            let mut child_offset = node_offset;
            for (i, child) in node.children().iter().enumerate() {
                if let Some((stats, new_child)) = go(lang, text, child_offset, child, edit) {
                    let mut result = node.clone();
                    result.children_mut()[i] = new_child;
                    return Some((stats, result))
                }
                child_offset += child.len()
            }

            None
        }
    }

    pub fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
        self.imp.node_type_info(ty)
    }
}

pub trait LanguageImpl: 'static + Send + Sync {
    fn parse(&self, text: &str) -> (FileStats, INode);
    fn reparse(&self, text: &str, parser_id: u32) -> Option<(FileStats, Vec<INode>)>;
    fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo;
}
