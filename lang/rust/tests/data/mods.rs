mod foo;
pub mod pfoo;

mod bar {}
pub mod pbar {}

mod baz {
    fn spam() {}
    mod quux {
        struct Eggs;
    }
}
