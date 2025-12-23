// src/services.rs
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// tests/services_test.rs
#[cfg(test)]
mod tests {
    use crate::services;

    #[test]
    fn test_greet() {
        let result = services::greet("Alice");
        assert_eq!(result, "Hello, Alice!");
    }
}
