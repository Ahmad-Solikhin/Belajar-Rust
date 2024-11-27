/**
Variable di rust menggunakan kata kunci let
Sifat dari variable di rust defaultnya adalah immutable (tidak bisa diubah lagi datanya setelah diinisiasi)
*/

#[test]
fn test_variable() {
    let name = "Gayuh";

    println!("Hello {}", name);
}

/**
Jika ingin membuat variable yang bisa diubah lagi datanya bisa menggunakana keyword "mut"
*/

#[test]
fn test_mutable() {
    let mut name = "Gayuh";
    println!("Hello ges {}", name);

    name = "Raharjo";
    println!("Hello ges {}", name);

    let mut age = 10;
    println!("My age is {}", age);

    age = 20;
    println!("My age is {}", age);
}