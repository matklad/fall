fn a() {}

fn b() { 1 }

fn c() { 1; }

fn d() { {} }

fn precedence() {
    let x = 1 + 2 * 3 % 4 - 5 / 6;
    1 + 2 * 3;
    1 << 2 + 3;
    1 & 2 >> 3;
    1 ^ 2 & 3;
    1 | 2 ^ 3;
    1 == 2 | 3;
    1 && 2 == 3;
    1 || 2 && 2;
    1 .. 2 || 3;
    1 = 2 .. 3;
    ---&*1 - --2 * 9;
}

fn angle_ambiguity() {
    foo::<X<Y>> >> 1 << 2;
}

fn blockish() {
    {};
    if 1 {} else { if 2 {} };
    1 + if 2 { 3 } else { 4 } + 5;
}

fn struct_cond_ambiguity() {
    if foo {}
    if (Foo {}) {}
    if foo(Foo {}) {}
    if {Foo {}} {}
}

fn blocklike_expressions() {
    { 92; } -1;
    if foo {}
    if bar {}
    1 + {1} + 2;
    let _ = {1} + 1;
    ({1} + 1);
    {
        let x = {1} + 1;
    }
    {92}
}

fn misc() {
    let a = 1;
    let b = a::b::<c>;
    3;
    (4);
    foo {};
    foo {a: 1};
    foo {a: 2,};
    foo::bar {a: 1, b: C {}};
    foo(1, 2, bar(3,));
    foo.bar.baz;
}
