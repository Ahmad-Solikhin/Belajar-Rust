/**
Function
Dibuat dengan fn, dan best practice untuk memberi nama function adalah menggunkana snake case contoh: fn test_hitung()
Function juga bisa ditambahkan parameter, parameter ini bisa atau lebih atau bahkan tanpa parameter sama sekali
Di parameter juga perlu ditambahkan tipe data

Return Value
Dalam membuat function kadang ingin mengembalikan nilai dari function tersebut, ini disebut return value
Untuk mendeklarasikan return value bisa mengunakan -> diikuti dengan tipe datanya
Buat return lebih awal ini sama aja kayak bahasa pemrograman lainnya

Recursive
Ini adalah function yang memanggil dirinya sendiri

Ownership
Di function juga terdapat aturan ownership pada variable
Tipe data heap yang dikirim sebagai parameter di function maka ownershipnya akan dipindah ke parameter function nya
Jika tipe datanya stack maka datanya akan di copy
Jadi untuk data heap yang dikirim sebagai parameter setelah functionnya selesai dieksekusi maka value dan ownershipnya akan dihapus

Return Value Ownership
Jika return valuenya adalah heap maka ownership akan dipindahkan ke yang memanggil functionya
Jika return valuenya adalah stack maka akan dicopy valuenya

Mengembalikan Ownership
Ownership bisa dikembalikan dengan menggunakan return value berupa tuple
Jadi konsepnya nanti parameternya dimasukkan juga ke return valuenya
Mengembalikan owenership dalam return value juga akan menyulitkan nantinya, karena semakin banyak parameter yang dikirim berarti semakin banyak juga return value yang harus diberikan
Solusi dari hal ini di rust menyediakan konsep dimana mengirim data heap tampa harus transfer ownership, hal ini disebut dengan reference



*/

fn say_hello() {
    println!("Hello ges");
}

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();
    say_hello();
    say_hello();
    say_hello();
}


fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {} {}", first_name, last_name);
}

#[test]
fn test_say_goodbye() {
    say_goodbye("Ahmad", "Solikhin");
    say_goodbye("Gayuh", "Raharjo");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn test_factorial() {
    println!("{}", factorial_loop(5));
    println!("{}", factorial_loop(3));
}

fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    }

    println!("{}", value);

    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("Gayuh"), 5);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("{}", result);
}


//Ini yang di stack
fn print_number(number: i32) {
    println!("Number {}", number);
}

#[test]
fn test_print_number() {
    let number = 10;
    print_number(number);
    
    //Number masih bisa diakses disini
    println!("Number from calling {}", number);
}

//Ini yang heap
fn hi(name: String) {
    println!("Name is {}", name);
}

#[test] 
fn test_hi() {
    let name = String::from("Gayuh");
    // hi(name); //Ini bakal mindahin ownershipnya
    hi(name.clone()); //Kalau mau dipake lagi dengan gaya seperti stack yang variablenya dicopy bisa pake clone ini
    
    //Ini udah gabisa lagi diakses variable namenya, karena ownershipnya udah pindah ke variable di parameter function dan udah dihapus juga
    println!("Hi {}", name);
}

fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name(){
    let first_name = String::from("Ahmad");
    let last_name = String::from("Solikhin");
    
    let full_name = full_name(first_name, last_name);
    
    println!("Full name is {}", full_name);
}

fn payback_ownership(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    (first_name, last_name, full_name)
}

#[test]
fn test_payback_ownership() {
    let first_name = String::from("Ahmad");
    let last_name = String::from("Solikhin");

    let (first_name, last_name, full_name) = payback_ownership(first_name, last_name);
    
    println!("{} + {} = {}",first_name, last_name, full_name);
}
