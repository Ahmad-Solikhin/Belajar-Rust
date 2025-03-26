fn main() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn start_application(host: &str, port: &str) {
    if host == "localhost" {
        panic!("You can't use localhost");
    } else {
        println!("Start {} on port {}", host, port);
    }
}

#[cfg(test)]
mod tests {
    use crate::{add, start_application};

    #[test]
    fn test_simple() {
        println!("Hello Test")
    }

    #[test]
    fn test_add() {
        let result = add(10, 2);
        assert_eq!(result, 12, "10 + 2 should be 12");
    }

    #[should_panic]
    #[test]
    fn start_application_panic() {
        start_application("localhost", "8080");
    }

    #[test]
    #[ignore]
    fn test_ignore() {
        println!("This test is ignored")
    }

    #[test]
    fn test_add_again() -> Result<(), String> {
        let result = add(1, 2);

        if result == 3 {
            Ok(())
        } else {
            Err("Haiyaa Loo".to_string())
        }

    }
}