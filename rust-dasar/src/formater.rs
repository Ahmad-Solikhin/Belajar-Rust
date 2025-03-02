use std::fmt::{Debug, Formatter};

struct Category {
    id: String,
    name: String,
}

// Ini sama kayak implement to string
impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

#[test]
fn test_format() {
    let category = Category { id: "hp".to_string(), name: "Xiomay".to_string() };
    println!("{:?}", category);
}