use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

/**
Collection
Ini bisa menghandle tipe data array yang fix, collection bisa berkembang
Collection disimpannya dalam heap bukan di stack seperti array
String juga sebenarnya adalah tipe data collection (collection of char)
Perlu diingat, karena collection ini disimpan dalam heap, maka bisa terjadi perpindahan ownership saat dikirimkan ke function, loop, maupun method

Tipe Data Collection
- Sequences : Tipe data yang memiliki index
- Maps : Berupa key dan value
- Sets : Tipe data yang isinya unique dan tidak memiliki index

Sequences
TIpe data yang mirip seperti array karena memiliki index, namun panjangnya bisa berkembang
Dibagi menjadi Vec (Vectoer), VecDeque, dan LinkedList

Vec (Vector)
Sequence yang urutannya sesuai dengan yang diinginkan
Menambahkan data diurutan paling belakang
Cocok untuk LIFO

VecDeque
Mirip seperti vecotr, namun bisa ditamnbahkan data dari depan maupun belakang
Jadi bisa digunakan sebagai Queue FIFO

Linked List
Ini menggunakan struktur data linked list, ini bagus untuk menambahkan, mengurangi, atau mencar data ke list
Ini tidak bisa diaksez menggunakan index seperti vector

Maps
Isinya menggunakna key dan value, dan keynya adalah unique
Ada dua implementasi dari map :
- HashMap : Key nya tidak diurutkan, maka operasi menambahkan data akan menjadi cepat namun tidak ada jaminan urutan
- BTreeMap : Key nya akan diurutkan

Sets
Tipe data seperti vector namun isinya unique, data di set juga tidak bisa diakses menggunakan index, datanya juga tidak urut sesuai dengan waktu dimasukkan
Ada 2 implementasi :
- BTreeSet : Menjamin urutan data
- HashSet : Tidak menjamin urutan data

Iterators
Ini adalah module untuk melakukan mekanisme looping data
Semua yang ada di collection memiliki fitur iterator


*/

#[test]
fn test_vector() {
    // Karena names ini disimpan dalam heap, jadi ownershipnya masih dipegang oleh names
    let mut names = Vec::<String>::new();
    names.push("Gayuh".to_string());
    names.push("Raharjo".to_string());

    // Namun saat dilakukan for disini maka ownershipnya berpindah
    for name in names {
        println!("{}", name);
    }

    // Jadi udah gabisa dipake lagi disini
    // println!("{:?}", names);
}

#[test]
fn test_vector_deque() {
    let mut names = VecDeque::<String>::new();
    names.push_back("Gayuh".to_string());
    names.push_back("Raharjo".to_string());
    names.push_front("Solikhin".to_string());

    for name in &names {
        println!("{}", name);
    }

    println!("{:?}", names);
}

#[test]
fn test_vector_linked_list() {
    let mut names = LinkedList::<String>::new();
    names.push_back("Gayuh".to_string());
    names.push_back("Raharjo".to_string());
    names.push_front("Solikhin".to_string());

    for name in &names {
        println!("{}", name);
    }

    println!("{:?}", names);
}

#[test]
fn test_hash_map() {
    let mut map = HashMap::<String, String>::new();
    map.insert("name".to_string(), "Gayuh".to_string());
    map.insert("age".to_string(), "23".to_string());

    let name = map.get("name");
    let age = map.get("age");

    println!("Name : {}", name.unwrap());
    println!("Age : {}", age.unwrap());
}

#[test]
fn test_btree_map() {
    let mut map = BTreeMap::<String, String>::new();
    map.insert("name".to_string(), "Gayuh".to_string());
    map.insert("age".to_string(), "23".to_string());
    map.insert("country".to_string(), "Indonesia".to_string());

    for entry in &map {
        println!("{} : {}", entry.0, entry.1);
    }

    println!("{:?}", map);
}

#[test]
fn test_hash_set() {
    let mut set = HashSet::<String>::new();
    set.insert("Ahmad".to_string());
    set.insert("Ahmad".to_string());
    set.insert("Gayuh".to_string());
    set.insert("Solikhin".to_string());
    set.insert("Solikhin".to_string());
    set.insert("Solikhin".to_string());
    set.insert("Solikhin".to_string());

    for i in &set {
        println!("{i}");
    }
}

#[test]
fn test_btree_set() {
    let mut set = BTreeSet::<String>::new();
    set.insert("Ahmad".to_string());
    set.insert("Ahmad".to_string());
    set.insert("Gayuh".to_string());
    set.insert("Solikhin".to_string());
    set.insert("Solikhin".to_string());
    set.insert("Solikhin".to_string());
    set.insert("Solikhin".to_string());

    for i in &set {
        println!("{i}");
    }
}

#[test]
fn test_iterator() {
    let array = [1, 2, 3, 4, 5];
    let mut iterator = array.iter();

    while let Some(value) = iterator.next() {
        println!("{}", value);
    }

    for i in iterator {
        println!("{}", i);
    }
}

#[test]
fn test_iterator_method() {
    let vector = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    println!("{:?}", vector);

    let sum: i32 = vector.iter().sum();
    println!("{}", sum);

    let count = vector.iter().count();
    println!("{}", count);

    let doubled: Vec<i32> = vector.iter().map(|x| {
        x * 2
    }).collect();
    println!("{:?}", doubled);

    //Tanda * pada x merupakan dereference, artinya menggunakan value aslinya bukan referensinya
    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("{:?}", odd);
}