/**
Attributes
Ini adalah cara untuk menambahkan metadata, ini mirip annotation di java
Attribute di rust sudah disediakan value-valuenya

Derive Attribute
Ini digunakna untuk melakukan implementasi trait secara otomatis, dan tidak bisa semua trait bisa dilakukan implementasi secara otomatis, hanya beberapa saja
Sebagai contoh trait Eq, PartialEq, Ord, PartialOrd
**/

#[derive(Debug, PartialOrd, PartialEq)]
struct Company {
    name: String,
    location: String,
    website: String,
}

#[test]
fn test_attribute_derive() {
    let company = Company {
        name: "Gayuh Enterprise".to_string(),
        location: "Singapore".to_string(),
        website: "https://agsr.com".to_string(),
    };

    let company2 = Company {
        name: "Gayuh Enterprise".to_string(),
        location: "Singapore".to_string(),
        website: "https://agsr.com".to_string(),
    };

    println!("{:?}", company);

    let result = company == company2;
    println!("{}", result);

    let result = company > company2;
    println!("{}", result);
}
