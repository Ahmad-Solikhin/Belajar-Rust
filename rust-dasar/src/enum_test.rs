/**
Enum
Merupakan tipe data yang digunakan untuk menyimpan beberapa kemungkinan tipe data Level, sebagai contoh bisa menyimpan Reguler dan Premium
Dengan menggunakan enum ini tipe data tersebut dibatasi dengan value yang sudah dideklarasikan

Enum Data
Enum juga bisa digunakna untuk menyimpan data kayak tuple
Untuk mengakses datanya perlu dilakukan pattern matching pada materi selanjutnya

Enum Method
Cara nambahinnya sama kayak cara nambahin di struct
*/

enum Level {
    Regular,
    Premium,
    Platinum
}

#[test]
fn test_enum() {
    let _level = Level::Regular;
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String)
}

impl Payment {
    fn pay(&self, amount: u32) {
        println!("Paying amount {}", amount);
    }
}

#[test]
fn test_payment() {
    let _payment1 = Payment::CreditCard(String::from("098139813978"));
    let _payment2 = Payment::BankTransfer(String::from("BCA"), String::from("8918914891"));
    let _payment3 = Payment::EWallet(String::from("DANA"), String::from("098139813978"));
}