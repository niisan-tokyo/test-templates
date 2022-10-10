pub fn fizzbuzz(num: usize) -> String {
    if num == 3 {
        "fizz".to_string()
    } else {
        num.to_string()
    }
    
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

    }

    #[test]
    fn input_10_and_return_buzz() {

    }

    #[test]
    fn input_15_and_return_fizzbuzz() {

    }

    #[test]
    fn input_30_and_return_fizzbuzz() {

    }
}
