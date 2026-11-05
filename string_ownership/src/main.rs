use std::io;

fn string_len(text: &String) {
    println!("Length: {}", text.len());
}

fn substring_lookup(text: &String, substring: &String) {
    if text.contains(substring) {
        println!("{} was found.", substring);
    } else {
        println!("No match found.");
    }
}

fn main() {
    let mut input = String::new();
    let substring = String::from("cal");
    println!("Text: ");
    io::stdin().read_line(&mut input).expect("No input given.");
    input = input.trim().to_string();

    string_len(&input);
    substring_lookup(&input, &substring);

    println!("Final print: {}", input);
}
