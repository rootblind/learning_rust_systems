pub fn reverse_text(text: String) -> String {
    text.chars().rev().collect()
}

pub fn is_palindrome(mut text: String) -> bool {
    text.retain(|c| !c.is_whitespace());
    text = text.to_lowercase();
    let copy = text.clone();
    let reverse = reverse_text(copy);
    if text == reverse { true } else { false }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_text_works() {
        let text = "1234".to_string();
        assert_eq!(reverse_text(text), "4321");
    }
    #[test]
    fn is_palindrome_works() {
        assert_eq!(is_palindrome(String::from("Never odd or even")), true);
        assert_eq!(is_palindrome(String::from("Rust is great")), false);
    }
}
