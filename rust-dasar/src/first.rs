pub fn say_hello() {
    println!("Hello from first module");
}

pub mod second {
    pub mod third {
        pub fn say_hello() {
            // Opsi pertama jika ingin mengakses melalui main nya
            crate::first::say_hello();

            // Opsi kedua, super pertama akan naik ke mod second -> super kedua akan naik ke mod first -> akses say_hello()
            super::super::say_hello();
        }
    }
}