struct Foo<> {}
struct Bar<U, V>(U, V);
fn baz<X,>() {}

struct A<A,>();
struct B<'a,>();
struct C<'a, A,>();

