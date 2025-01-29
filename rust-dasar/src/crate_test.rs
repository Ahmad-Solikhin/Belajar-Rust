/**
Crate
Kode yang dijalankan oleh rust compiler untuk membuat aplikasi atau library
Contoh kode sebelumnya adalah aplikasi dan cratenya ada di src/main.rs
Di dalam crate harus didefinisikan file-file yang digunakan sebagai module
Jika ingin use module lain diluar file main harus menggunakan prefix crate:: untu mengacu ke main.rs
Jika dilakukan di main file maka tidak perlu lagi menggunakna crate::

Saat membuat nested module dan ingin mengakses module yang ada diatasnya bisa menggunakna super, bisa juga menyebutkan namanya dari awal menggunakan crate

*/

use crate::first::say_hello as say_hello_first;
use crate::second::say_hello as say_hello_second;
use crate::third::say_hello as say_hello_third;

#[test]
fn test_crate() {
    say_hello_first();
    say_hello_second();
    say_hello_third();
}


