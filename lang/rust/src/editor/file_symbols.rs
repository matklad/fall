use fall_tree::{File, Node, Text};
use fall_tree::visitor::{visitor, process_subtree_bottom_up};
use {NameOwner, FnDef, StructDef, EnumDef, TypeDef, TraitDef};

pub fn process_symbols<'f>(file: &'f File, f: &mut FnMut(Text<'f>, Node<'f>)) {
    fn p<'f, T: NameOwner<'f>>(n: T, f: &mut FnMut(Text<'f>, Node<'f>)) {
        if let Some(name) = n.name() {
            f(name, n.node())
        }
    }
    process_subtree_bottom_up(
        file.root(),
        visitor(f)
            .visit::<FnDef, _>(|f, def| p(def, f))
            .visit::<StructDef, _>(|f, def| p(def, f))
            .visit::<EnumDef, _>(|f, def| p(def, f))
            .visit::<TypeDef, _>(|f, def| p(def, f))
            .visit::<TraitDef, _>(|f, def| p(def, f)),
    );
}