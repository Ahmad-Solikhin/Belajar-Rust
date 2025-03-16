use std::rc::Rc;

/**
Multiple Ownership
Pada umumnya owner sebuah value hanya dimiliki oleh 1 varaible saja
Jika ingin sebuah value bisa dimiliki oleh beberapa owner bisa menggunakan Rc<T> (Reference Counted)
Jadi nanti akan dihitung beberapa jumlah ownernya, jika sudah tidak ada maka akan dihapus
Rc<T> ini juga merupakan smart pointer
**/

#[derive(Debug)]
enum Category{
    Of(String, Rc<Category>),
    End
}

#[test]
fn test_multiple_ownership() {
    let asis = Rc::new(Category::Of("Asis".to_string(), Rc::new(Category::End)));
    println!("Count owner {}", Rc::strong_count(&asis));
    let laptop = Category::Of("Laptop".to_string(), Rc::clone(&asis));
    println!("Count owner {}", Rc::strong_count(&asis));
    let handphone = Category::Of("handphone".to_string(), Rc::clone(&asis));
    println!("Count owner {}", Rc::strong_count(&asis));
}