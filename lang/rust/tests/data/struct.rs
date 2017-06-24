struct A {}
struct B { f: A }
struct C { f: A, }
struct D { f: A, pub j: B }
struct E(u32);
struct F(u32,);
struct G(u32, String);
struct H;

