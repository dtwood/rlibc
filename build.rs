extern crate gcc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    gcc::compile_library("libstart.a", &[&format!("src/crt/{}/start.s", target)]);
}
