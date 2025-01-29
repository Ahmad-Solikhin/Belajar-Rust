/**
Type Alias
Digunakan untuk membuat tipe data alias dari tipe data yang udah ada sebelumnya
Sebagai contoh umur biasanya integer bisa dibuat type aliasnya menjadi age


*/

type Age = u8;

struct Customer {
    name: String,
    age: Age,
}

#[test]
fn test_type_alias() {
    let customer = Customer { name: String::from("Gayuh"), age: 23 };

    println!("Name {} with age {}", customer.name, customer.age);
}