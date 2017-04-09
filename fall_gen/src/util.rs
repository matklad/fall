use std::ascii::AsciiExt;

pub fn scream(word: &str) -> String {
    word.chars().map(|c| c.to_ascii_uppercase()).collect()
}

pub fn snake(word: &str) -> String {
    word.split("_")
        .map(|w| w[..1].to_ascii_uppercase() + &w[1..])
        .collect()
}


macro_rules! ln {
    ($name:ident, $($e:expr),*) => { $name.line(&format!($($e),*)) };
}

pub struct Buff {
    inner: String,
    indent: usize
}

impl Buff {
    pub fn new() -> Buff {
        Buff { inner: String::new(), indent: 0 }
    }

    pub fn line(&mut self, line: &str) {
        for _ in 0..self.indent {
            self.inner.push_str("    ");
        }
        self.inner.push_str(line);
        self.inner.push('\n');
    }

    pub fn block(&mut self, block: &str) {
        self.inner.push_str(block)
    }

    pub fn blank_line(&mut self) {
        self.inner.push('\n');
    }

    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn dedent(&mut self) {
        self.indent -= 1;
    }

    pub fn done(self) -> String {
        self.inner
    }
}
