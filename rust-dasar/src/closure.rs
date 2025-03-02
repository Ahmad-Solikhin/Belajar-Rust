/**
Closure
Ini bisa dibilang juga sebagai anonymous function
Untuk tipe datanya adalah fn(paramType) -> returnType

Closure sebagai Parameter
Dengan menggunakan closure, bisa dioper kedalam parameter function kayak callback function

Closure Scope
Dengan menggunakan closure, closure dapat menggunakan data dalam scope yang sama, namun ini bisa membingungkan jika sering digunakan pada project besar

*/

#[test]
fn test_closure() {
    let sum = |value1: i32, value2: i32| -> i32 {
        value1 + value2
    };

    let result = sum(1, 2);
    println!("Result is {}", result);
}

fn print_with_filter(value: String, filter: fn(String) -> String) {
    let result = filter(value);
    println!("{}", result);
}

#[test]
fn test_closure_as_parameter() {
    print_with_filter(
        "Gayuh".to_string(),
        |value: String| -> String { value.to_uppercase() },
    )
}

#[test]
fn test_closure_scope() {
    let mut counter = 0;

    let mut increment = || {
        counter += 1;
        println!("Increment");
    };

    increment();
    increment();
    increment();

    println!("Counter is : {}", counter);
}

// Untuk mengatasi masalah closure scope bisa membuat struct untuk counter

struct Counter {
    counter: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.counter += 1;
        println!("Increment");
    }
}

#[test]
fn test_closure_scope_with_struct() {
    let mut counter = Counter { counter: 0 };

    counter.increment();
    counter.increment();
    counter.increment();

    println!("Counter : {}", counter.counter);
}