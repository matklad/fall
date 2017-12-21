use fall_tree::File;
use fall_tree::visitor::{visitor, process_subtree_bottom_up};
use {NameOwner, FnDef, StructDef, EnumDef, TypeDef, TraitDef};

pub fn process_symbols<'f>(file: &'f File, f: &mut FnMut(&NameOwner<'f>)) {
    process_subtree_bottom_up(
        file.root(),
        visitor(f)
            .visit::<FnDef, _>(|f, def| f(&def))
            .visit::<StructDef, _>(|f, def| f(&def))
            .visit::<EnumDef, _>(|f, def| f(&def))
            .visit::<TypeDef, _>(|f, def| f(&def))
            .visit::<TraitDef, _>(|f, def| f(&def)),
    )
}