/**
Macro
Macro bukanlah function, ini merupakan fitur rust untuk membuat kode lainnya, di bahasa pemrograman lain biasa disebut meta programming
Kekurangan makro ini adalah implementasinya lebih kompleks dibanding dengan membuat function biasa

Declarative Macro
Ini adalah salah satu cara membuat makro yang paling mudah
Untuk membuat macro ini menggunakan macro_rules!, isinya berupa (pattern) => [expansion]
Pattern adalah kondisi yang diinginkan dan expansion adalah kode yang akan dibuat oleh macro
**/

macro_rules! hi {
    () => {
        println!("Hi")
    };
}

#[test]
fn test_macro() {
    println!("Hi Bang {}", "Gayuh");

    hi!();
}