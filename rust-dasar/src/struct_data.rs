/**
Struct 
Tipe data mirip tuple yang bisa menampung beberapa tipe data yang berbeda
Bedanya di struct bisa diberikan nama untuk setiap datanya, dan nantinya bisa diakses menggunakan namanya bnukan menggunakan index saja
Ini mirip kayak object atau struct di golang

Cara membuat instance dari struct nya wajib menentukan semua value untuk field di struct nya

Struct bisa digunakan sebagai parameter maupun return value dari sebuah function

Init Shorthand, ini konsepnya kayak kasih key dan value di js, jadi langsung kasih nama variablenya, namun ini akan memindahkan ownershipnya

Struct update syntax
Defaultnya instance struct adalah immutable, jika ingin mengupdatenya bisa diubah menggunakan mut atau mengunkana destructuring kayak di js
Tapi saat menggunkana struct update syntax harus hati2 dengan value yang berada di heap, karena nanti ownernya akan berpindah ke field instance yang baru
Untuk menghindari hal ini bisa manual satu2 di clone pada tipe data yang di heap

Tuple Struct
Ini jatuhnya sama seperti tuple namun tidak menggunakna nama fieldnya, untuk aksesnya sama seperti tuple menggunakan indexnya

Struct tanpa Field
Dalam membuat struct bisa dibuat tanpa adanya field, untuk sekarang masih belum berguna namun nanti kegunaannya akan terlihat saat menggunakna Trait

Reference Filed Struct
Tipe data dalam struct bisa berbentuk dalam reference, namun untuk melakukannya harus menggunakan Lifetime, nanti akan di bahas

*/


struct Person {
    name: String,
    age: u8,
    gender: String,
    active: bool
}

fn print_person(person: &Person) {
    println!("{}", person.name)
}


#[test]
fn test_struct_person() {
    let name: String = String::from("Ahmad");
    
    
    let person1 = Person {
        age: 23,
        name: String::from("Gayuh"),
        gender: String::from("L"),
        active: true
    };

    // Nah ini mirip kayak di js, cuma nanti variable namenya udah gabisa dipake lagi karena ownernya dah dipindahin
    let person2 = Person {
        age: 23,
        name,
        gender: String::from("L"),
        active: true
    };
    
    let person3 = Person {name: person1.name.clone(), gender: person1.gender.clone(), ..person1};
    
    print_person(&person1);
    print_person(&person2);
    print_person(&person3);
    
}

// Kalau tidak menggunakna nama fieldnya harus menggunakna () bukan {}
struct GeoPoint(f64, f64);

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(10.672, 7.9760);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}


// Struct tanpa field sama sekali
struct Nothing;

#[test]
fn test_nothing() {
    let _nothing1: Nothing = Nothing;
    let _nothing2: Nothing = Nothing{};
}



