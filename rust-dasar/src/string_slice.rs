/**
String Slice
&str ini berarti adalah reference ke sebagian atau seluruh data str
Dengan menggunakan &str berarti bisa mengambil sebagian atau keseluruhan dari String
Dan karena ini tipe reference maka juga berarti tidak ada ownernya



*/

#[test]
fn string_slice() {
    let name: String = String::from("Ahmad Solikhin Gayuh Raharjo");
    
    let first_name: &str = &name[0..5];
    println!("{}", first_name);
    
    let last_name = &name[21..];
    println!("{}", last_name);
}