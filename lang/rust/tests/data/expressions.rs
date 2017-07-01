fn a() {}

fn b() { 1 }

fn c() { 1; }

fn d() { {} }

fn precedence() {
    let x = 1 + 2 * 3 % 4 - 5 / 6;
    1 + 2 * 3;
    1 & 2 + 3;
    1 | 2 & 3;

}

fn blockish() {
    {};
    if 1 {} else { if 2 {} };
    1 + if 2 { 3 } else { 4 } + 5;
}

fn struct_cond_ambiguity() {
    if foo {}
    if (foo {}) {}
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
}

fn main() {
    let and1 = 2 * 1 & 1;
    let and2 = (2 * 1) & 1;
    let and3 = 2 * (1 & 1);
    println!("{} {} {}", and1, and2, and3);

    let or1 = 2 * 0 | 1;
    let or2 = (2 * 0) | 1;
    let or3 = 2 * (0 | 1);
    println!("{} {} {}", or1, or2, or3);
}
