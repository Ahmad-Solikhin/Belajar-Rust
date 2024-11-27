/**
If Expression
Sama seperti dengan bahasa pemrograman lain, if digunakan untuk melakukan percabangan, cara membuat if nya mirip sepoerti golang


If di rust bisa mengembalikan nilai, karena if dalam rust itu termasuk expression
*/

#[test]
fn test_if() {
    let value = 8;

    if value == 10 {
        println!("Perfect");
    } else if value > 7 {
        println!("Good");
    } else {
        println!("Bad");
    }
}

#[test]
fn test_if_value() {
    let value = 8;
    let result: &str;

    // Ini adalah cara manual
    if value == 10 {
        result = "Perfect";
    } else if value > 7 {
        result = "Good";
    } else {
        result = "Bad";
    }

    println!("Result is {}", result);

    // Cara yang lebih mudah

    let actual_result = if value == 10 {
        "Perfect"
    } else if value > 7 {
        "Good"
    } else {
        "Bad"
    };

    println!("Actual result {}", actual_result);
}