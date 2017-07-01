fn a() {}

fn b() { 1 }

fn c() { 1; }

fn d() { {} }

fn statements() {
    let a = 1;
    let b = a::b::<c>;
    3;
}

fn precedence() {
    let x = 1 + 2 * 3 % 4 - 5 / 6;
}

fn blockish() {
    {};
    if 1 {} else { if 2 {} };
    1 + if 2 { 3 } else { 4 } + 5;
}
