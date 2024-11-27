/**
Tipe data di rust dibagi jadi 2:
1. scalar : nilai tunggal (single value)
2. compound : nilai bisa lebih dari 1 value (tuple, array)

Scalar
- Integer : bilangan bulat
- Float : bilangan desimal
- Boolean : true or false
- Char : karakter

Compound
- Tuple : kumpulan data yang tipe datanya bisa berbeda
- Array : kumpulan data dengan tipe data yang sama

Tipe data bisa disebutkan secara explicit dengan menggunakan : {tipe_data} setelah nama variable

Konversi tipe data number dari ukuran kecil ke lebih besar atau sebaliknya, tapi kalo lebih besar ke lebih kecil harus diperhatikan besarnya, jika tidak bisa ditampung bisa terjadi overflow dan nilainya jadi 0

Char harus menggunakan petik satu dan hanya bisa 1 karakter

Tuple
Jumlah data di dalam tuple itu udah final tidak bisa dikurangi atau ditambah lagi
Untuk bikin tuple bisa pake () tanda kurung
Untuk akses data di tuple menggunakan indexnya bisa menggunakna . (titik)
Tuple bisa dilakukan destructuring sama kayak di js, jika ada data yang tidak ingin digunakan bisa menggunakan _ (garis bawah)
Jika membuat variable tuple dalam bentuk mutable maka isinya dapat dirubah lagi, caranya pakai nomor indexnya dan diassign saja pake =

Unit
Ini adalah tuple kosong, maksudnya gada isinya sama sekali, unit ini biasanya digunakan dalam function yang tidak mengembalikan data

Array
Merupakan kumpulan 1 tipe data, misal kalau integer ya integer semua dan sebagainya
Untuk membuat array menggunakan [] (kurung siku)
Array di rust ukurannya fix dan tidak bisa dirubah, dikurang maupun ditambah
Cara akses array juga pake index tapi ini pake kurung siku kek di bahasa lainnya
Ini sama kayak variable lainnya, kalo datanya mau diubah harus dibuat mutable
Di array bisa dihitung panjangnya berapa, bisa menggunakan function bawaan len()

Constant
Ini adalah vbariable immutable yang tidak bisa diubah sama sekali, perbedaannya dengan let adalah constant tidak memiliki opsi mutable
Untuk mengunakna constant harus langsung dideklarasikan, dan tipe datanya harus disebutkan secara eksplist
Best practice nya constant ini penamaannya menggunakan huruf kapital dan tanda hubung _ jika lebih dari 1 kata

Stack
Ini menyimpan data yang fix ukurannya, contoh number array dan tuple

Heap
Ini digunakan untuk menyimpan data yang ukurannya tidak fix, contohnya string

String
DI rust ada 2 macam string
- &str : (string slice) ukuran datanya fix dan akan disimpan di satck
- String : ukuran datanya bisa mengembang dan disimpan di heap

&str
Jika membuat variable mutable yang menggunakan &str jika diubah valunya maka yang berubah ada isinya buakn di update isi dari string sebelumnya
&str memiliki beberapa method untuk memanipulasi datanya namun akan selalu membuat data baru
Ada beberapa method juga yang mengembalikan String bukan &str




*/

#[test]
fn explicit() {
    let age: i32 = 20;

    println!("My age is {}", age);
}

#[test]
fn konversi() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 1000000000;
    let e: i8 = d as i8;
    println!("{}", e);
}

#[test]
fn tuple() {
    let data: (i32, &str, bool) = (10, "Gayuh", true);
    println!("{:?}", data);

    let data2 = (10, "Gayuh", true);
    println!("{:?}", data2);

    println!("Data pertama {}", data.0);
    println!("Data pertama {}", data.1);
    println!("Data pertama {}", data.2);
}

#[test]
fn destructuring() {
    let data: (i32, &str, bool) = (10, "Gayuh", true);

    let (a, b, c) = data;

    println!("{}, {}, {}", a, b, c);
}

#[test]
fn mutable_tuple() {
    let mut data: (i32, &str, bool) = (10, "Gayuh", true);

    let (a, b, c) = data;

    println!("{}, {}, {}", a, b, c);

    data.1 = "Ahmad";

    println!("{:?}", data);
}

fn unit() {
    println!("Hello")
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result)
}

#[test]
fn test_array() {
    let array = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    //Bisa juga dibuat kayak gini
    let array: [&str; 2] = ["Ahmad", "Solikhin"];
    println!("{:?}", array);

    println!("{}", array[1]);

    let mut array = ["Ahmad", "Gayuh"];
    println!("{:?}", array);

    array[0] = "Raharjo";
    println!("{:?}", array);

    println!("{}", array.len());
}

#[test]
fn test_two_dimension_array() {
    let matrix = [[1, 2], [2, 3]];

    println!("{:?}", matrix);
}

#[test]
fn test_constant() {
    const MINIMUM: i32 = 10;
    println!("{}", MINIMUM);
}

#[test]
fn test_stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Gayuh");

    println!("{}, {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Ahmad");

    println!("{}, {}", a, b);
}

#[test]
fn test_string() {
    let name = " Gayuh ";
    let trim = name.trim();

    println!("{}", trim);
}
