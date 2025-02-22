fn say_hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    println!("{}", say_hello("World"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello() {
        assert_eq!(say_hello("World"), "Hello, World!");
        assert_eq!(say_hello("Rust"), "Hello, Rust!");
    }
}
