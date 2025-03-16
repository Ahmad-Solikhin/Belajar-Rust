use std::ops::Deref;

/**
Dereference
Kadang saat mengggunakan reference namun ingin mengakses langsung datanya bisa dilakukan dereference
Untuk menggunakannya adalah dengan menggunakan symbol bintang (*)

Deref Trait
Saat melakukan reference atau Box bisa menggunakan * untuk dereference nya
Bagaimana jika menggunakan tipe data lain misal struct, nah hal ini gabisa dilakukan dereference
Untuk melakukannya bisa menggunakan Deref Trait, jika valuenya mutable bisa menggunakan DerefMut
**/

#[test]
fn test_dereference() {
    let value1 = Box::new(10);
    let value2 = Box::new(20);

    let result = *value1 * *value2;
    println!("Result is {}", result);
}

#[derive(Debug)]
struct MyValue<T> {
    value: T
}

impl<T> Deref for MyValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[test]
fn test_deref() {
    let value = MyValue{ value: 20};
    let real_value = *value;
    println!("Value is {}", real_value);
}

fn say_hello(name: &String) {
    println!("Hello {}", name);
}

#[test]
fn test_deref_reference() {
    let name = MyValue {
        value: "Gayuh".to_string()
    };

    say_hello(&name);
}