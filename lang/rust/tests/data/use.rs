extern crate spam;
extern crate eggs as yolk;

use foo::bar;
use ::foo::bar::baz;
use {};
use ::*;
use *;
use ::{};
use foo as bar;
use foo::{bar, baz as quux};
use foo::{bar,};

