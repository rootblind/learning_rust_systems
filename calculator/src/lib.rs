pub fn calculate(x: f64, y: f64, operator: char) -> f64 {
    match operator {
        '+' => x + y,
        '-' => x - y,
        '*' => x * y,
        '/' => {
            if y == 0 as f64 {
                panic!("Can not divide by 0!");
            } else {
                x / y
            }
        }
        '^' => x.powf(y),
        _ => panic!("Invalid operator!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_working() {
        assert_eq!(calculate(1., 2., '+'), 3.);
        assert_eq!(calculate(2., 2., '^'), 4.);
    }

    #[test]
    #[should_panic]
    fn invalid_operator() {
        calculate(1., 1., '?');
    }

    #[test]
    #[should_panic]
    fn divide_by_zero() {
        calculate(1., 0., '/');
    }
}
