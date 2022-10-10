pub fn fizzbuzz(num: usize) -> String {
    match (num % 3, num % 5) {
        (0, 0) => "fizzbuzz",
        (0, _) => "fizz",
        (_, 0) => "buzz",
        (_, _) => {
            return num.to_string();
        }
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_1_and_return_1() {
        let result = fizzbuzz(1);
        assert_eq!(result, "1");
    }

    #[test]
    fn input_2_and_return_2() {
        let result = fizzbuzz(2);
        assert_eq!(result, "2");
    }

    #[test]
    fn input_3_and_return_fizz() {
        let result = fizzbuzz(3);
        assert_eq!(result, "fizz");
    }

    #[test]
    fn input_6_and_return_fizz() {
        let result = fizzbuzz(6);
        assert_eq!(result, "fizz");
    }

    #[test]
    fn input_5_and_return_buzz() {
        let result = fizzbuzz(5);
        assert_eq!(result, "buzz");
    }

    #[test]
    fn input_10_and_return_buzz() {
        let result = fizzbuzz(10);
        assert_eq!(result, "buzz");
    }

    #[test]
    fn input_15_and_return_fizzbuzz() {
        let result = fizzbuzz(15);
        assert_eq!(result, "fizzbuzz");
    }

    #[test]
    fn input_30_and_return_fizzbuzz() {
        let result = fizzbuzz(30);
        assert_eq!(result, "fizzbuzz");
    }
}
