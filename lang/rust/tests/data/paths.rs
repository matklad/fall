use foo::bar::baz;
use self::foo;
use self::super::super::foo;


fn foo() {
    let x = ::Vec::<Vec<i32>>::new;
}
