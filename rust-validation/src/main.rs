use serde::Serialize;
use validator::{Validate, ValidateArgs};

pub mod asgr {
    pub mod validator {
        use crate::{DatabaseContext, RegisterRequest};
        use std::borrow::Cow;
        use validator::ValidationError;

        pub fn not_blank(value: &str) -> Result<(), ValidationError> {
            if value.trim().is_empty() {
                return Err(ValidationError::new("not_blank")
                    .with_message(Cow::from("value cannot be blank")));
            }

            Ok(())
        }

        pub fn password_match(request: &RegisterRequest) -> Result<(), ValidationError> {
            if request.password != request.confirm_password {
                return Err(ValidationError::new("password_match")
                    .with_message(Cow::from("password and confirm password must be the same")));
            }

            Ok(())
        }

        pub fn can_register(
            request: &RegisterRequest,
            context: &DatabaseContext,
        ) -> Result<(), ValidationError> {
            if context.total >= context.max_data {
                return Err(
                    ValidationError::new("can_register").with_message(Cow::from(format!(
                        "cannot register user {}, database is full",
                        request.username
                    ))),
                );
            }

            Ok(())
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Validate)]
struct LoginRequest {
    #[validate(length(min = 3, max = 20, message = "Minimal 3 and max 20"))]
    username: String,

    #[validate(length(min = 3, max = 20))]
    password: String,
}

#[test]
fn test_validate_success() {
    let request = LoginRequest {
        username: "asgr".to_string(),
        password: "rahasia".to_string(),
    };

    assert!(request.validate().is_ok());
}

#[test]
fn test_validate_failed() {
    let request = LoginRequest {
        username: "as".to_string(),
        password: "ra".to_string(),
    };

    assert!(request.validate().is_err());

    let errors = request.validate().err().unwrap();
    println!("{:?}", errors.errors());
}

#[derive(Debug, Validate)]
#[validate(context = DatabaseContext,
    schema(
        function = "crate::asgr::validator::password_match",
        skip_on_field_errors = false,
        code = "testing"
    ),
    schema(
        function = "crate::asgr::validator::can_register",
        skip_on_field_errors = false,
        code = "testing",
        use_context
    )
)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20))]
    username: String,
    #[validate(length(min = 3, max = 20))]
    password: String,
    #[validate(length(min = 3, max = 20))]
    confirm_password: String,
    #[validate(length(min = 3, max = 20))]
    name: String,
    #[validate(nested)]
    address: AddressRequest,
}

#[derive(Debug, Validate)]
struct AddressRequest {
    #[validate(length(min = 3, max = 20))]
    street: String,
    #[validate(length(min = 3, max = 20))]
    city: String,
    #[validate(length(min = 3, max = 20))]
    country: String,
}

#[test]
fn test_nested_validation_success() {
    let register = RegisterRequest {
        username: "Gayuh".to_string(),
        password: "rahasia".to_string(),
        confirm_password: "rahasia".to_string(),
        name: "Gayuh".to_string(),
        address: AddressRequest {
            street: "Jalan".to_string(),
            city: "Bekasi".to_string(),
            country: "Indonesia".to_string(),
        },
    };

    assert!(
        register
            .validate_with_args(&DatabaseContext {
                total: 1,
                max_data: 100
            })
            .is_ok()
    );
}

#[test]
fn test_nested_validation_error() {
    let register = RegisterRequest {
        username: "Gayuh".to_string(),
        password: "rahasia".to_string(),
        confirm_password: "salah".to_string(),
        name: "Gayuh".to_string(),
        address: AddressRequest {
            street: "".to_string(),
            city: "Bekasi".to_string(),
            country: "Indonesia".to_string(),
        },
    };

    assert!(
        register
            .validate_with_args(&DatabaseContext {
                total: 101,
                max_data: 100
            })
            .is_err()
    );
    let errors = register
        .validate_with_args(&DatabaseContext {
            total: 101,
            max_data: 100,
        })
        .err()
        .unwrap();
    println!("{:?}", errors.errors());
}

#[derive(Debug, Validate)]
struct Product {
    id: String,
    name: String,
    #[validate(nested, length(min = 1))]
    variants: Vec<ProductVariant>,
}

#[derive(Debug, Validate, Serialize)]
struct ProductVariant {
    #[validate(length(min = 3, max = 20))]
    name: String,
    #[validate(range(min = 3, max = 1000000000))]
    price: i32,
}

#[test]
fn test_validate_vector_success() {
    let request = Product {
        id: "1".to_string(),
        name: "Test 1".to_string(),
        variants: vec![ProductVariant {
            name: "Variant 1".to_string(),
            price: 10000,
        }],
    };

    assert!(request.validate().is_ok());
}

#[test]
fn test_validate_vector_failed() {
    let request = Product {
        id: "1".to_string(),
        name: "Test 1".to_string(),
        variants: vec![ProductVariant {
            name: "".to_string(),
            price: -1,
        }],
    };

    assert!(request.validate().is_err());
    let errors = request.validate().err().unwrap();
    println!("{:?}", errors.errors());
}

#[derive(Debug, Validate, Serialize)]
struct CreateCategoryRequest {
    #[validate(custom(function = "crate::asgr::validator::not_blank"))]
    id: String,
    #[validate(custom(function = "crate::asgr::validator::not_blank"))]
    name: String,
}

#[test]
fn test_custom_validation() {
    let category = CreateCategoryRequest {
        id: "   ".to_string(),
        name: "".to_string(),
    };

    let errors = category.validate().err().unwrap();
    println!("{:?}", errors.errors());
}

pub struct DatabaseContext {
    total: i32,
    max_data: i32,
}
