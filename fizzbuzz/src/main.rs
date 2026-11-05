fn fizz_buzz(number: u16) -> String {
    if number % 5 == 0 && number % 3 == 0 {
        String::from("FizzBuzz")
    } else if number % 3 == 0 {
        String::from("Fizz")
    } else if number % 5 == 0 {
        String::from("Buzz")
    } else {
        format!("{}", number)
    }
}

fn main() {
    for i in 1..100 {
        println!("{}", fizz_buzz(i));
    }
}

#[cfg(test)]
mod fizz_buzz {
    use super::*;

    #[test]
    fn fizz_buzz_test() {
        assert_eq!(fizz_buzz(3), String::from("Fizz"));
        assert_eq!(fizz_buzz(5), String::from("Buzz"));
        assert_eq!(fizz_buzz(15), String::from("FizzBuzz"));
        assert_eq!(fizz_buzz(2), 2.to_string());
    }
}
