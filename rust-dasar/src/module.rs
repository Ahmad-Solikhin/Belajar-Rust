/**
Module
Ini adalah cara untuk organisir kode2
Untuk membuat module bisa menggunakan kata kunci mod lalu diikuti nama modulnya
Secara default visibility dari code block yang ada di dalam module hanya bisa diakses oleh module tersebut saja
Jika ingin mengakses isi dari modulenya dari tempat lain, bisa mengubah aksesnya dari private menjadi public dengan kata kunci pub baik di type, function maupun method
Untuk mengaksesnya harus menggunakan nama modulenya terlebih dahulu lalu diikuti oleh ::

Use Keyword
Kadang sulit juga jika harus menulis tertus menerus modulnya, use bisa digunakan agar kode yang dibuat masuk ke dalam scope module
Dengan demikian bisa mengakses tanpa menyebutkan modulenya lagi
Jika terdapat member yang sama dari module yang di use bisa menggunakan kata kunci as agar nama member tidak bentrok

Module FIle Terpisah
Biasanya module akan dibuat dalam di file terpisah agar kode program tidak terlalu panjang dan mudah digunakan
Dan otomatis nama filenya agan digunakan sebagai nama modulnya
Kecuali jika ingin menggunakna sub module bisa deklarasikan namanya secara terpisah
Secara default kode pada file terpisah tidak akan diinclude dalam programnya

Use Lainnya
Kadang kode program sudah banyak dan melakukan use terlalu banyak member di satu module, ada cara lain menggunakan use
Jika ingin mengambil semua member bisa menggunakan tanda bintang (*) : use module::*
Atau jika ingin mengambil beberapa saja bisa sebutkan dalam tanda kurung kurawal {} : use module::{A, B, C}

*/

mod model {
    pub struct User {
        pub name: String,
        pub username: String,
        pub email: String,
        pub age: u8,
    }

    impl User {
        pub fn say_hello(&self, name: &str) {
            println!("Hello {name}, my name is {}", self.name);
        }
    }
}

#[test]
fn test_access_module() {
    let user = model::User {
        name: String::from("Gayuh"),
        username: String::from("asgr"),
        email: String::from("test@gmail.com"),
        age: 23,
    };

    user.say_hello("Ahmad");
}



