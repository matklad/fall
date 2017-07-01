fn a() {}

fn b() { 1 }

fn c() { 1; }

fn d() { {} }

fn precedence() {
    let x = 1 + 2 * 3 % 4 - 5 / 6;
}

fn blockish() {
    {};
    if 1 {} else { if 2 {} };
    1 + if 2 { 3 } else { 4 } + 5;
}

fn struct_cond_ambiguity() {
    if foo {}
}

fn misc() {
    let a = 1;
    let b = a::b::<c>;
    3;
    foo {};
    foo {a: 1};
    foo {a: 2,};
    foo::bar {a: 1, b: C {}};
}
