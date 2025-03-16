use std::cell::RefCell;

/**
Interior Mutability
Ini adalah desing pattern dalam rust untuk mengubah data walaupun datanya reference nya immutable
Untuk melakukan ini bisa menggunakan type RefCell<T>, RefCell ini single ownership
Aturan dari RefCell:
- Bnayak immutable borrow diperbolehkan
- Satu mutable borrow diperbolehkan
- Banyak mutable borrow tidak diperbolehkan
- Sekaligus mutable dan immutable borrow tidak diperbolehkan
**/

#[derive(Debug)]
struct Seller {
    name: RefCell<String>,
    active: RefCell<bool>
}

#[test]
fn test_ref_cell() {
    let seller = Seller {
        name: RefCell::new("Gayuh".to_string()),
        active: RefCell::new(true)
    };
    println!("{:?}", seller);

    {
        let mut name = seller.name.borrow_mut();
        *name = "Raharjo".to_string();
    }

    println!("{:?}", seller);

    let mut seller2 = Seller {
        name: RefCell::new("Ahmad".to_string()),
        active: RefCell::new(true)
    };

    println!("{:?}", seller2);

    seller2.name = RefCell::new("Solikhin".to_string());

    println!("{:?}", seller2);
}