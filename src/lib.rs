#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]
#[cfg(test)]
extern crate quickcheck;

fn fizz_buzz(number: i32) -> String {
    let mut result = String::new();

    if number % 3 == 0 {
        result.push_str("Fizz")
    }
    if number % 5 == 0 {
        result.push_str("Buzz")
    }
    if !result.is_empty() {
        return result.to_string()
    }

    format!("{}", number)
}

#[cfg(test)]
mod tests {
    use fizz_buzz;
    use quickcheck::TestResult;


    #[quickcheck]
    fn multiples_of_three_start_with_fizz(data: i32) -> bool {
        let output = fizz_buzz(data * 3);
        output.starts_with("Fizz")
    }
    #[quickcheck]
    fn multiples_of_five_end_with_buzz(data: i32) -> bool {
        let output = fizz_buzz(data * 5);
        output.ends_with("Buzz")
    }

    #[quickcheck]
    fn multiples_of_three_and_five_are_fizz_buzz(data: i32) -> bool {
        let output = fizz_buzz(data * 5 * 3);
        output == "FizzBuzz"
    }

    #[quickcheck]
    fn multiples_of_neither_three_or_five_output_the_number(data: i32) -> TestResult {
        if data % 3 == 0 {
            return TestResult::discard()
        }

        if data % 5 == 0 {
            return TestResult::discard()
        }

        TestResult::from_bool(fizz_buzz(data) == format!("{}", data))
    }
}
