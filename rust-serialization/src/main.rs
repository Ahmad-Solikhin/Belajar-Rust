use chrono::{DateTime, Utc};
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Formatter;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Serialize, Deserialize)]
struct UserLoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AddressRequest {
    street: String,
    city: String,
    state: String,
    zip: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    username: String,
    password: String,
    #[serde(rename = "alamat")]
    address: AddressRequest,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))] // Ini digunakan jika ingin keseluruhan
struct User {
    username: String,
    email: String,
    hobbies: Vec<String>,
    phone_number: Option<String>,
    gender: Gender,
    payment: Payment,
}

#[derive(Debug, Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Payment {
    CreditCard {
        card_number: String,
        expiration: String,
    },
    BankAccount {
        account_number: String,
        bank_name: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    id: String,
    name: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    created_at: DateTime<Utc>,
}

#[test]
fn test_create_json_for_user_login_request() {
    let login_request = UserLoginRequest {
        username: String::from("user"),
        password: String::from("rahasia"),
    };

    let json = serde_json::to_string(&login_request).unwrap();
    println!("{}", json);

    let login_result: UserLoginRequest = serde_json::from_str(&json).unwrap();
    println!("{:?}", login_result);
}

#[test]
fn test_create_json_for_create_user_request() {
    let create_user_request = CreateUserRequest {
        username: String::from("user"),
        password: String::from("rahasia"),
        address: AddressRequest {
            street: "Jalan".to_string(),
            city: "Kota".to_string(),
            zip: 17511,
            state: "Provinsi".to_string(),
        },
    };

    let json = serde_json::to_string(&create_user_request).unwrap();
    println!("{}", json);

    let login_result: CreateUserRequest = serde_json::from_str(&json).unwrap();
    println!("{:?}", login_result);
}

#[test]
fn test_create_json_for_user() {
    let user = User {
        username: String::from("user"),
        email: String::from("test@gmail.com"),
        hobbies: vec![String::from("Rebahan"), String::from("Turu")],
        phone_number: Some("81371837".to_string()),
        gender: Gender::Male,
        payment: Payment::CreditCard {
            card_number: "783187187".to_string(),
            expiration: "2025".to_string(),
        },
    };

    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);

    let login_result: User = serde_json::from_str(&json).unwrap();
    println!("{:?}", login_result);
}

#[test]
fn test_map() {
    let mut values = HashMap::<String, i32>::new();
    values.insert("one".to_string(), 1);
    values.insert("two".to_string(), 2);
    values.insert("three".to_string(), 3);

    let json = serde_json::to_string(&values).unwrap();
    println!("{}", json);

    let login_result: HashMap<String, i32> = serde_json::from_str(&json).unwrap();
    println!("{:?}", login_result);
}

#[test]
fn test_chrono() {
    let category = Category {
        id: "1".to_string(),
        name: "Tech".to_string(),
        created_at: Utc::now(),
    };

    let json = serde_json::to_string(&category).unwrap();
    println!("{}", json);

    let login_result: Category = serde_json::from_str(&json).unwrap();
    println!("{:?}", login_result);
}

// Start Custom
#[derive(Debug, Serialize, Deserialize)]
struct Admin {
    id: String,
    name: Name,
}

#[derive(Debug)]
struct Name {
    first: String,
    last: String,
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{} {}", self.first, self.last).as_str())
    }
}

#[test]
fn test_custom_serialize() {
    let admin = Admin {
        id: "Admin".to_string(),
        name: Name {
            first: "Ahmad".to_string(),
            last: "Solikhin".to_string(),
        },
    };

    let json = serde_json::to_string(&admin).unwrap();
    println!("{}", json);

    let result: Admin = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
}

struct NameVisitor;

impl<'de> Visitor<'de> for NameVisitor {
    type Value = Name;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Expecting name as string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let result: Vec<&str> = v.split(" ").collect();
        if result.len() != 2 {
            return Err(Error::custom("Expecting first and last name"));
        }

        Ok(Name {
            first: result[0].to_string(),
            last: result[0].to_string(),
        })
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(NameVisitor)
    }
}
