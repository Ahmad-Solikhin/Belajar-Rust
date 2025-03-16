/**
Lifetime
Tiap data memiliki lifetime berdasarkan scope variablenya, hal ini untuk mencegah juga yang namanya dangling reference

Lifetime di Function
Hal membingungkan lainnya adalah ketika menggunakan reference di parameter dan di return juga
Misal dengan case function dengan 2 parameter reference dan membandingkan kedua nilainya lalu melakukan return salah satu parameter sebagai reference
Di case ini rust akan bingung kearena harus melakukan borrow parameter pertama atau kedua karena kondisinya berbeda

Lifetime Annotation Syntax
Fitur ini untuk menentukan parameter mana yang akan di borrow, cara menambahkannya sama seperti generic cuma diawali dengan petik satu
Lifetime tiodak menambahkan waktu hidup dari sebuah variable, ini cuma sebagai tanda di rust value mana yang bisa dijadikan return
Annotation ini bisa juga digunakan pada struct dan method
Lifetime ini juga bis digabungkan dengan generic, perbedaannya untuk lifetime annotation ditandai dengan petik satu

**/


#[test]
fn test_dangling_reference() {
    let r: &i32;
    {
        let x = 5;
        //r = &x; //Akan terjadi error disini, karena saat keluar dari scope varibale x akan dihapus dari memory
    }
    r = &60;
    println!("R is : {}", r);
}

//Jadi dengan menggunakan annotation ini maka returnnya bisa mengambil type annotation 'a, yang berarti returnnya bisa value1 atau value2
fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str {
    if value1.len() > value2.len() {
        return value1
    }

    value2
}

#[test]
fn test_lifetime_annotation() {
    let value1 = "Ahmad";
    let value2 = "Gayuh";
    let result = longest(value1, value2);
    println!("Result is : {}", result);
}

struct Student<'a, 'b> {
    name: &'a str,
    last_name: &'b str
}

impl<'a, 'b> Student<'a, 'b> {
    fn longest_name(&self, student: &Student<'a, 'b>) -> &'a str {
        if self.name.len() > student.name.len() {
            return self.name;
        }

        student.name
    }
}

fn longest_student_name<'a, 'b>(student1: &Student<'a, 'b>, student2: &Student<'a, 'b>) -> &'a str {
    if student1.name.len() > student2.name.len() {
        return student1.name
    }

    student2.name
}

#[test]
fn test_student() {
    let student1 = Student {
        name: "Gayuh",
        last_name: "Raharjo"
    };
    let student2 = Student {
        name: "Adi",
        last_name: "Suradi"
    };

    let mut result = longest_student_name(&student1, &student2);
    println!("Student with longest name is {}", result);

    result = student1.longest_name(&student2);
    println!("Student with longest name is {}", result);
}