mod variable;
mod data_types;
mod loop_test;
mod if_expression;
mod function;
mod references_borrowing;
mod slice;
mod string_slice;
mod struct_data;
mod method;
mod enum_test;
mod pattern_matching;
mod type_alias;
mod module;
mod first;
mod second;
mod crate_test;
mod third;
mod trait_test;

fn main() {
    println!("Hello, world!");
}

// cargo test hello_test -- --exact --nocapture
#[test]
fn hello_test() {
    println!("Haiyaaa loooo");
}

use first::say_hello;
use second::say_hello as say_hello_second;

#[test]
fn test_use() {
    first::say_hello();
    second::say_hello();

    say_hello();
    say_hello_second();

    // Lalu jika ingin akses sub mod yang ada di mod lain bisa seperti ini
    first::second::third::say_hello();
}
