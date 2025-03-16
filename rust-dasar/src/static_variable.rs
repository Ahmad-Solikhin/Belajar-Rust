/**
Static
Caranya mirip menggunakan constant, cuma harus menggunakan kata kunci static
Perbedaannya dengan constant:
- Bisa diubah dengan menggunakan mutable static
- Ada kemungkinan tidak aman jika dilakukan mutable karena bisa diupdate oleh banyak tempat
- Untuk mengubah static harus menggunakan unsafe block atau unsafe function
**/

static APPLICATION: &str = "myapp";

#[test]
fn test_static() {
    println!("{}", APPLICATION);
}

static mut COUNTER: i32 = 0;

unsafe fn increment() {
    COUNTER += 1;
}

#[test]
fn test_static_mut() {
    unsafe {
        increment();
        COUNTER += 1;

        println!("{}", COUNTER);
    }
}