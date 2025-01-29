/**
Trait
Definisi kontrak untuk tipe data lainnya
Trait ini mirip konsepnya sama interface di java, jadi kontrak ini harus diimplementasikan
Trait berisi definisi method tanpa implementasinya, untuk membuatnya bisa menggunakan kata kunci trait dan diikuti dengan namanya
Untuk naming conventionnya mirip struct pake PascalCase
Trait ini bisa digunakan sebagai tipe data namun tetep harus ada implementasinya
Untuk implementasinya bisa menggunakan impl NamaTrait for TipeData
Trait tidak bisa dibuat instance nya atau object nya, untuk membuat instance dari trait harus menggunakan implementasinya (sama kayak java juga gini)
Trait sebenarnya juga bisa juga digunakan untuk Method dengan implementasi konkrit atau default implementation dan secara otomatis Type yang nanti melakukan implementasi akan mendapatkan default implementation dari trait tersebut jika tidak dilakukan impelementasi ulang

Trait Parameter
Trait juga bisa digunakan sebagai parameter, jadi bisa mengirimkan tipe data apapun yang penting tipe data tersebut mengimplementasikan trait tersebut
Untuk penggunakannya harus ada kata kunci "impl NamaTrait" pada parameternya, dan jika tipe datanya reference bisa tinggal ditambah &

Multiple Trait
Type bisa implementasi lebih dari satu trait, konsepnya sama kayak interface di java bisa implementasi lebih dari 1 interface
Parameter di function bisa merupakan kombinasi dari banyak trait

Return Trait
Selain untuk parameter, trait juga bisa digunakan sebagai return value
Karen trait tidak bisa dibuat instance nya, return valuenya sudah harus dalam implementasinya
Untuk membuatnya sebagai return value, perlu disebutkan seperti di parameter yaitu impl NamaTrait
Sebaiknya return langsung valuenya bukan reference, karena nanti bisa menyebabkan dangling reference
Kembalian dari function return trait tidak bisa berbeda implementasi atau struct nya, ini nanti bisa ditangani menggunakan generic

Conflict Method Name
Kadang nama trait itu konflik dengan nama function lainnya, rust tidak akan menjadikan hal ini error dan akan ketahuan saat memanggilnya
Ada cara untuk menentukan method mana yang akan dipanggil menggunakan Type::nama_method(instance)

Super Trait
Trait bisa digabungkan dengan konsep pewarisan dimana suatu Trait bisa memiliki beberapa Trait di bawahnya
Trait yang ada di atasnya disebut sebagai Super Trait
Misal dibuat Trait A, Trait B, dan Trait C, lalu Trait A menjadi super Trait B dan Trait C
Sehinggal Trait A menjadi Super Trait nya, Trait juga bisa memiliki lebih dari 1 Super Trait dengan menggunakan +



*/

trait CanSayHello {
    fn hello(&self) -> String {
        "Hello".to_string()
    }
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

trait CanSayGoodBye {
    fn good_bye(&self) -> String;
    fn good_bye_to(&self, name: &str) -> String;
}

struct Person {
    name: String,
    age: i8,
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {} from Method impl", name, self.name);
    }
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {} from Trait", self.name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.name)
    }

    // Function yang sebelumnya diberikan defaut method bisa dilakukan override jika mau
    fn hello(&self) -> String {
        "Ini kalo mau di override".to_string()
    }
}

impl CanSayGoodBye for Person {
    fn good_bye(&self) -> String {
        format!("Goodbye from {}", self.name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, from {}", name, self.name)
    }
}

#[test]
fn test_trait() {
    let person = Person {
        name: "Gayuh".to_string(),
        age: 23,
    };

    // println!("{}", person.say_hello()); // Ini bakal error karena struct Person juga punya Method say_hello
    println!("{}", person.say_hello_to("Amanda"));
    println!("{}", person.hello());

    say_hello_trait(&person);

    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Ahmad"));

    say_hello_and_goodbye_trait(&person);
}

// Agar bisa akses say_hello yang ada di dalam trait bisa menggunakan function dengan parameter trait nya
fn say_hello_trait(value: &impl CanSayHello) {
    println!("{}", value.say_hello());
}

fn say_hello_and_goodbye_trait(value: &(impl CanSayHello + CanSayGoodBye)) {
    println!("{}. {}", value.say_hello(), value.good_bye());
}

struct SimplePerson {
    name: String,
}

impl CanSayGoodBye for SimplePerson {
    fn good_bye(&self) -> String {
        format!("Goodbye from {}", self.name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, from {}", name, self.name)
    }
}

impl SimplePerson {
    fn create_simple_person(name: &str) -> impl CanSayGoodBye {
        SimplePerson { name: name.to_string() }
    }
}

// Hal ini gabisa dilakukan di rust walaupun structnya sama-sama mengimplementasi Trait yang sama, ini bisa dilakukan di Generic nanti
// fn create_person(name: String) -> impl CanSayGoodBye {
//     if name == "Gayuh".to_string() {
//         SimplePerson {name}
//     } else {
//         Person {
//             name,
//             age: 24
//         }
//     }
// }

#[test]
fn test_create_simple_person() {
    let simple_person = SimplePerson::create_simple_person("Gayuh");
    println!("{}", simple_person.good_bye());
}

#[test]
fn test_say_hello_multiple_method() {
    let person = Person {
        name: "Ahmad".to_string(),
        age: 24
    };

    // Memanggil method yang ada di trait
    println!("{}", CanSayHello::say_hello(&person));

    // Memanggil method yang ada di instance nya
    Person::say_hello(&person, "Gayuh");
}

// Trait CanSay memiliki 2 Super Trait, yaitu CanSayHello dan CanSayGoodBye
trait CanSay: CanSayHello + CanSayGoodBye {
    // Kelebihan dari implementasi trait lain, jadi trait ini bisa mengakses method yang ada di super trait nya
    fn say(&self) {
        println!("{}", self.say_hello());
    }
}

struct VerySimplePerson {
    name: String
}

// Walaupun hanya impelentasi ke CanSay, tapi tetep di implement satu-satu ke CanSayHello dan CanSayGoodBye
impl CanSay for VerySimplePerson {
}

impl CanSayHello for VerySimplePerson {
    fn say_hello(&self) -> String {
        todo!()
    }

    fn say_hello_to(&self, name: &str) -> String {
        todo!()
    }
}

impl CanSayGoodBye for VerySimplePerson {
    fn good_bye(&self) -> String {
        todo!()
    }

    fn good_bye_to(&self, name: &str) -> String {
        todo!()
    }
}