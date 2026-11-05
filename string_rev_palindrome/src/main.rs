use std::io;

use string_rev_palindrome::is_palindrome;
fn main() {
    println!("Enter a text: ");
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read the text input.");

    text = text.trim().to_string();
    match is_palindrome(text) {
        true => println!("The input is palindrome"),
        false => println!("The input is not palindrome"),
    }
}
