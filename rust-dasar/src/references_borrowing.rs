/**
References
Reference ini adalah pointer (penunjuk) ke data di heap, owner datanya masih pemilik awalnya
Reference akan dijamin menunjuk ke value yang valid selama data ownernya masih ada
Untuk membuat referebce di rust bisa menggunkana tanda & sebelum tipe datanya
Reference bisa dibuat sebanyak mungkin tidak seperti ownership yang hanya boleh ada 1 dalam siklus yang ada

Borrowing
Saat membuat reference, kegiatan ini disebut dengan borrowing
Bisa dianalogikan kita bisa meminjam barang, dan kalau sudah selesai wajib mengembalikan ke ownernya
Tidak bisa juga memodifikasi value dari reference karena ini defaultnya bersifat immutable walaupun variable ownernya adalah mutable

Mutable Reference
Reference defaultnya memang immutable, namun ini bisa diubah menjadi mutable dengan cara menggunakan &mut
Ada ketentuan jika ingin membuat mutable reference, yaitu variable owner juga harus mutable
Selain itu, untuk menjamin keamanannya dalam satu waktu hanya boleh ada satu mutable refernce dan tidak ada reference lainnya (baik yang mutable maupun yang immutable)

Dangling Pointer
Ini adalah kondisi dimana pointer yang menunjuk ke value yang tidak ada di memory (heap)
Di rust hal ini tidak diperbolehkan, contohnya ketika mengembalikan reference, karena value akan otomatis dihapus keteku sudah keluar dari scope functionnya
Hal ini bisanya terjadi di golang yang sering membuat function dengan return value pointer, namun di rust tidak bisa
Solusinya adalah dengan value langsung bukan referencenya, dan nanti akan ownershipnya akan berpindah
Atau bisa juga keluarkan variable ownernya ke variable diluar function, agar variable tersebut masuk kedalam scope utama dan tidak akan dihapus setelah function selesai dieksekusi

*/

//Tanda & disini berarti parameter hanya meminjam value yang dikirim saat memanggil function full_name tampa memindahkan ownershipnya ke parameter yang ada di function
fn full_name(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name: String = String::from("Ahmad");
    let last_name: String = String::from("Solikhin");
    
    //Disini juga gabisa langsung asal lempar data string nya, karena konsepnya borrowing, jadi harus ditambah tanda & juga
    let full_name = full_name(&first_name, &last_name);
    
    //Karena first_name dan last_name dikirim ke function dengan konsep borrwing maka masih bisa diakses dibawah ini karena ownershipnya tidak berpindah
    println!("{} + {} = {}", first_name, last_name, full_name);
}

// fn cannot_change_value(value: &String) {
    
    //Hal ini tidak bisa dilakukan akrena data reference sifat defaultnya adalah immutable, konsepnya mirip minjem barang harus dibalikin dengan kondisi yang sama
    // value.push_str("Haiyaaa");
// }

fn can_change_value(value: &mut String) {
    value.push_str(" Raharjo");
}

#[test]
fn test_change_value() {
    let mut name = String::from("Gayuh");
    // cannot_change_value(&name);
    println!("{name}");
    
    //Walaupun ini dipanggil berkali kali datanya, bisa dilakukan karena setelah function dipanggil maka proses borrowing nya akan dihapus lagi
    can_change_value(&mut name);
    can_change_value(&mut name);
    can_change_value(&mut name);
    can_change_value(&mut name);
    println!("{name}");
    
    // Yang tidak bisa dilakukan adalah seperti ini, karena saat borrow 1 belum digunakan dilakukan borrow lagi untuk kedua kalinya
    let value_borrow1 = &mut name;
    // let value_borrow2 = &name; //Ini dicomment karena nanti bakal error
    
    println!("{value_borrow1}");
}

// fn dangling_full_name(first_name: &String, last_name: &String) -> &String {
//     let name = format!("{} {}", first_name, last_name);
//     
//     &name
// }

// #[test]
// fn test_dangling_full_name() {
//     let first_name: String = String::from("Ahmad");
//     let last_name: String = String::from("Solikhin");
// 
//     //Disini juga gabisa langsung asal lempar data string nya, karena konsepnya borrowing, jadi harus ditambah tanda & juga
//     let full_name = dangling_full_name(&first_name, &last_name);
// 
//     //Karena first_name dan last_name dikirim ke function dengan konsep borrwing maka masih bisa diakses dibawah ini karena ownershipnya tidak berpindah
//     println!("{} + {} = {}", first_name, last_name, full_name);
// }
