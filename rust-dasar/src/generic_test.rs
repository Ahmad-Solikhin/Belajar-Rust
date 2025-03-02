use crate::generic_test::Value::VALUE;

/**
Generic
Merupakan fitur bisa membuat function, struct, enum, method atau trait yang tipe datanya bisa diubah, ini sama aja kayak generic di java

Generic di Struct
Bisa menambahkan tipe data generic setelah nama struct menggunakan <> dan diisi dengan tipe data generic yang ingin digunakan
Tipe datanya bisa lebih dari 1 dan tinggal menggunakan koma ",", biasanya generic akan menggunakan 1 huruf kapitgal aja

Generic di Enum
Ini sama aja cara buatnya kayak yang di Struct

Generic Type Bond
Saat membuat generic bis diberikan batasan juga jenis tipe data apa saja, ini caranya menggunakan titik dua ":" setelah tipe genericnya dan diikuti dengan Trait nya
Jika ingin menggunakan multiple Trait bisa menggunakan +

Generic di Function
Generic bisa digunakan juga di function, bisa diterapkan pada parameter dan return value

Generic di Method
Saat buat generic di method, generic bisa ditambahkan setelah kata kunci impl agar bisa digunakan semua method
Atau bisa juga khusus untuk method tertentu dengan cara menambahkan generic seperti saat membuat function

Generic di Trait
Saat membuat generic di trait, akan memaksa implementasi trait tersebut menggunakan type generic juga

Where Clause
Ada cara lain untuk menggunakan type bound tanpa titik dua, yaitu menggunakan where clause denganb kata kunci where
Ini akan lebih mudah dibaca saat memiliki type bound yang banyak

Default Generic Type
Generic type di rust bis diberikan default nya menggunakan = (sama dengan)


*/

struct Point<T = i32> {
    x: T,
    y: T,
}

struct TwoPoint<T, K> {
    x: T,
    y: K,
}

#[test]
fn test_generic_struct() {
    let integer = Point::<i32> {x: 5, y: 10};
    let integer = Point::<f64> {x: 5.0, y: 10.0};
    let boolean = Point::<bool> {x: true, y: false};
    
    let string_integer = TwoPoint::<String, i32> {x: "Gayuh".to_string(), y: 24};
    
    println!("{}", string_integer.x);
    
}

enum Value<T> {
    NONE, 
    VALUE(T)
}

#[test]
fn test_generic_enum() {
    let value = Value::<i32>::VALUE(10);
    
    match value {
        Value::NONE => {
            println!("none");
        }
        VALUE(value) => {
            println!("Value {}", value);
        }
    }
}

trait CanSayGoodBye {
    fn good_bye(&self) -> String;
    fn good_bye_to(&self, name: &str) -> String;
}

struct Person {
    name: String,
    age: i8,
}

impl CanSayGoodBye for Person {
    fn good_bye(&self) -> String {
        format!("Goodbye from {}", self.name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, from {}", name, self.name)
    }
}

struct Hi<T: CanSayGoodBye> {
    value: T
}

#[test]
fn test_generic_bound() {
    let hi = Hi::<Person> {
        value: Person {name: "Gayuh".to_string(), age: 24}
    };
    
    println!("{}", hi.value.name);
}

fn min<T: PartialOrd>(value1: T, value2: T) -> T {
    if value1 < value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn test_generic_function() {
    // Bisa seperti ini
    let result = min::<i32>(1, 10);
    
    // Atau bisa seperti ini juga
    let result = min(1, 10);


    print!("Result {}", result);
}

impl<T> Point<T> {
    fn get_x(&self) -> &T { 
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_method() {
    let point = Point{x: 10, y: 20};
    
    let x_point = point.get_x();
    
    print!("{}", point.get_y());
}

trait GetValue<T> where T: PartialOrd {
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T> where T: PartialOrd {
    fn get_value(&self) -> &T {
        &self.x
    }
}