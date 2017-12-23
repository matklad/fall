use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

#[macro_export]
macro_rules! checkpoint {
    ($name:ident) => {
        pub static $name: $crate::Checkpoint = $crate::Checkpoint {
            flag: ::std::sync::atomic::ATOMIC_BOOL_INIT,
            name: stringify!($name),
        };
    }
}

pub struct Checkpoint{
    pub flag: AtomicBool,
    pub name: &'static str,
}

impl Checkpoint {
    pub fn pass(&self) {
        self.flag.store(true, SeqCst)
    }

    pub fn check<F: FnOnce()>(&self, f: F) {
        self.flag.store(false, SeqCst);
        f();
        if !self.flag.load(SeqCst) {
            panic!("Checkpoint {} was not hit", self.name)
        }
    }
}

#[test]
fn test_hit() {
    checkpoint!(FOO);
    FOO.check(|| {
        FOO.pass();
    })
}

#[test]
#[should_panic]
fn test_not_hit() {
    checkpoint!(FOO);
    FOO.check(|| {

    })
}