/**
Clean Up

Drop Trait
Ini adalah trait yang bisa dogunakan untuk melakukan eksekusi function sebelum value tersebut dilakukan drop atau dihapus dari memory

**/

// #[derive(Debug)]
#[derive(Debug)]
struct Book {
    title: String
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Dropping book : {}", self.title);
    }
}

#[test]
fn test_clean_up() {
    let book = Book {
        title: "ABC".to_string()
    };

    println!("{:?}", book);
}