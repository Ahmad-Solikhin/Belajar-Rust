/**
Perulangan dalam rust bisa menggunakan loop, tetep ada konsep break dan continue juga
Loop juga bisa mengembalikan nilai kayak di if

Loop Label
Jika membuat loop dalam loop dan ingin menghentikan loop yang berada diatasnya bisa menggunakan labelnya
Untuk memberi label pada loop bisa menggunakan contoh : 'outer: loop

While Loop
Perulangan yang menggunakan kondisi sebagai penentu apakah loop lanjut atau tidak, ini juga bisa make break dan continue

For Loop
Loop ini bisa digunakan untuk mengambil semua data satu2 dari arraynya

Range
Di rust ada tipe data range yang isinya adalah jarak antara start dan end
Range ini tipenya juga collection sama seperti array
Cara membuat range bisa menggunakan (start..end) contohnya 0..5 artinya start dari 0 dan diakhiri di 4

Range Inclusive
Jika range sebelumnya end nya tidak benar2 end mealinkan (n-1), jika ingin tetap mengikuti akhrannya bisa menggunakan yang namanya range inclusive
Cara penggunaannya (start..=end) contoh 0..=4, ini bakal ngambil sampe 4
*/

#[test]
fn loop_expression() {
    let mut counter = 0;

    loop {
        counter += 1;
        
        if counter % 2 == 0 { continue; } else if counter > 10 { break; }

        println!("Counter : {}", counter);

    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    
    println!("Counter : {}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }
            
            println!("{} * {} = {}", number, i, number * i);
            
            i += 1;
            if i > 10 {
                println!();
                break;
            }
        }
        
        number += 1;
    }
}

#[test]
fn test_while_loop() {
    let mut counter = 1;

    while counter <= 10 {
        println!("{}", counter);
        
        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    //Cara manual
    while index < array.len() {
        println!("{}", array[index]);
        
        index += 1;
    }
    
    println!();
    // Menggunakan for loop
    for value in array {
        println!("Value array : {}", value);
    }
}

#[test]
fn test_range() {
    let array = ["A", "B", "C", "D", "E"];
    let range = 0..5; //Walaupun sampe 5, tapi nanti berakhir di 4 (n - 1)
    println!("Start : {}", range.start);
    println!("End : {}", range.end);
    
    for i in range {
        println!("{}", i);
    }
    
    println!();
    
    for i in 0..array.len() {
        println!("{}", array[i]);
    }
}

#[test]
fn range_inclusive() {
    let range = 0..=5;
    println!("Start : {}", range.start());
    println!("End : {}", range.end());
    
    for i in range {
        println!("{}", i);
    }
}