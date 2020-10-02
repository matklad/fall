use fall_tree::{File, Node, Text};
use fall_tree::visitor::{visitor, process_subtree_bottom_up};
use crate::syntax::{NameOwner, FnDef, StructDef, EnumDef, TypeDef, TraitDef, ModDef};

pub fn process_symbols<'f>(file: &'f File, f: &mut dyn FnMut(Text<'f>, Node<'f>)) {
    fn p<'f, T: NameOwner<'f>>(n: T, f: &mut dyn FnMut(Text<'f>, Node<'f>)) {
        if let Some(name) = n.name() {
            f(name, n.node())
        }
    }
    process_subtree_bottom_up(
        file.root(),
        visitor(f)
            .visit::<FnDef, _>(|def, f| p(def, f))
            .visit::<StructDef, _>(|def, f| p(def, f))
            .visit::<EnumDef, _>(|def, f| p(def, f))
            .visit::<TypeDef, _>(|def, f| p(def, f))
            .visit::<TraitDef, _>(|def, f| p(def, f))
            .visit::<ModDef, _>(|def, f| p(def, f)),
    );
}