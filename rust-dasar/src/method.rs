/**
Method
Method sebenarnya mirip dengan function, bedanya tidak bisa berdiri sendiri dan harus menempel di struct, enum atau trait
Pada method parameter pertama biasanya adalah self, ini representasi dari instance dimana method itu dipanggil
Dan biasanya selfnya akan dibuat dengan reference agar ownershipnya tidak berpindah ke method nya

Associated Functions
Jika function dengan parameter pertama self maka disebut dengan method
Namun jika tidak menggunkana self maka methodnya tidak terhubung dengan istance nya dan disebut dengan Associated Functions

*/

// Cara membuat method dalam struct

struct Person{
    name: String,
    age: u8
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.name);
    }
    
    // Ini jatohnya kayak static function
    fn new(name: String) -> Person {
        Person{name, age: 0}
    }
}

#[test]
fn test_method() {
    let person = Person{name: String::from("Gayuh"), age: 23};
    person.say_hello("Jono");
}

#[test]
fn test_associated_functions() {
    let person = Person::new(String::from("Ahmad"));
}