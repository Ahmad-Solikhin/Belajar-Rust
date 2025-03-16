/**
Smart Pointer
Pointer merupakan hal umum yang berisikan alamat ke lokasi di memory
Samrt pointer ini adalah pointer yang memiliki metadata, di rust ada konsep ownership dan borrowing yang berarti reference ini hanya meminjamkan data
Sedangkan smart pointer ini adalah pemilik dair data yang ditunjuk, jadi seakan-akan dia pointer tapi dia sebenarnya adalah pemiliknya
Box<T> adalah cara membuat smart pointer dan akan membuat data disimpan di Heap sedangkan pointernya berada di Stack, keuntungannya agar data tidak akan dicopy saat dikirimkan, sehingga lebih irit memory

Recursive Data Type
Single data dari Box mungkin tidak menarik, namun akan berguna ketika menemui tipe data yang recursive
Misal ada tipe data Category yang di dalamnya bisa ada Category lagi

**/

#[test]
fn test_box() {
    //Jadi walaupun i32 disimpan dalam stack tapi karena menggunakan Box, maka datanya akan disimpan di Heap
    let value  = Box::new(32);
    println!("{}", value);

    display_number(*value); //* adalah dereference yang berarti mengirimkan data asilnya
    display_number_reference(&value);
}

fn display_number(value: i32) {
    println!("Display number {}", value);
}

fn display_number_reference(value: &i32) {
    println!("Display number reference {}", value);
}

//Tanpa Smart Pointer
#[derive(Debug)]
enum Category{
    Of(String, Box<Category>),
    End
}


#[test]
fn test_box_enum() {
    let category = Category::Of(
        "Hanphone".to_string(),
        Box::new(Category::Of(
            "Sungsang".to_string(),
            Box::new(Category::End)))
    );

    println!("{:?}", category);
}