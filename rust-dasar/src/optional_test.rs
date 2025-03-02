/**
Optional Values
Pada bahasa pemrograman lain ada null atau undefined, namun di rust tidak mengenal data seperti ini
Untuk membuat variable yang memang datanya bisa ada dan tidak bisa menggunakan Option Enum
Ada 2 opsi
- None : Untuk yang tidak ada isinya
- Some(T) : Untuk yang ada isinya
*/

fn double(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i * 2)
    }
}

// Ini versi lebih simplenya, kayaknya baru dah
fn double_with_map(x: Option<i32>) -> Option<i32> {
    x.map(|i| i * 2)
}

#[test]
fn test_options() {
    let result = double(Some(10));
    println!("{:?}", result);

    let result = double(None);
    println!("{:?}", result);
}