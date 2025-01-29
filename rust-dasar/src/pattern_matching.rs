/**
Pattern Matching
Selani menggunakan if dalam melakukan percabangan, di rust bisa digunakan pattern matching
Hal ini lumayan kompleks

Enum
Untuk melakukan kondisi dari enum bisa menggunakan pattern matching ini dan harus dibuat untuk semua kondisi yang ada di enum nya
Ini juga bisa digunakna untuk mengambil data yang terdapat pada enumnya disebut dengan destructuring

Value
Pattern matching juga bisa digunakan untuk value seperti string atau number
Untung value ada bagian elsenya yang akan diisi dengan nama variablenya

Multiple Pattern
Jika ingin menggunakan kondisi seperti or dalam pattern matching bis amenggunakan pipe (|)

Range Pattern
Ini bisa digunakan jika value yang dilakukan match adalah number, jadi cukup menggunakan tipe data range

Destructuring Struct Patterns
Selain destructure enum, bisa juga digunakan untuk struct filed
Nama dari variable harus sesui dengan fieldnya kecuali untuk tuple struct bis menggunakna nama variable lain

Match Expression
Selain digunakan untuk kondisi, match juga bisa digunakan untuk assign value menggunakna let

*/

enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_pattern_matching_enum() {
    let level = Level::Regular;

    match level {
        Level::Regular => {
            println!("Regular level")
        }
        Level::Premium => {
            println!("Premium level")
        }
        Level::Platinum => {
            println!("Platinum level")
        }
    }
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount)
            }
            Payment::BankTransfer(bank, number) => {
                println!("Paying with {} transfer {} amount {}", bank, number, amount)
            }
            Payment::EWallet(wallet, number) => {
                println!("Paying with {} wallet {} amount {}", wallet, number, amount)
            }
        }
    }
}

#[test]
fn test_payment() {
    let _payment1 = Payment::CreditCard(String::from("098139813978"));
    _payment1.pay(10000);

    let _payment2 = Payment::BankTransfer(String::from("BCA"), String::from("8918914891"));
    let _payment3 = Payment::EWallet(String::from("DANA"), String::from("098139813978"));
}

#[test]
fn test_match_value() {
    let name = "Gayuh";

    if name == "Gayuh" {
        println!("Holla");
    }

    match name {
        "Gayuh" => {
            print!("Hello Gayuh")
        }
        other => {
            println!("Hello {}", other)
        }
    }
}

#[test]
fn test_multiple_pattern() {
    let name = "Ahmad";

    match name {
        "Gayuh" | "Ahmad" => {
            print!("Hello Bang")
        }
        other => {
            println!("Hello {}", other)
        }
    }
}


#[test]
fn test_range_pattern() {
    let value = 100;

    match value {
        0..51 => {
            println!("Bad Bang");
        }
        51..76 => {
            println!("Good Bang");
        }
        76..101 => {
            println!("Very Good");
        }
        _other => {
            println!("Invalid value");
        }
    }
}


struct GeoPoint(f64, f64);

struct Person {
    name: String,
    age: u8,
    gender: String,
    active: bool,
}

#[test]
fn test_struct_pattern() {
    let point = GeoPoint(2.0, 1.0);

    match point {
        GeoPoint(long, 0.0) => {
            println!("Long {long}");
        }
        GeoPoint(0.0, lat) => {
            println!("Lat {lat}");
        }
        GeoPoint(long, lat) => {
            println!("Long {long}, Lat {lat}");
        }
    }

    let person = Person {
        name: String::from("Gayuh"),
        age: 23,
        gender: String::from("L"),
        active: true,
    };

    match person {
        Person { name, age, .. } => {
            println!("Name {name}, age {age}")
        }
    }
}

#[test]
fn test_match_expression() {
    let value = 2;

    let result = match value {
        0 => String::from("Nol"),
        1 => String::from("Satu"),
        _ => String::from("Invalid")
    };
    
    println!("Result is {result}")
}

