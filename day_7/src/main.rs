fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let sum = input.lines().fold((0, 0), |(mut sum1, mut sum2), line| {        
        fn calculate_and_test_1(target_val: u64, val: Option<u64>, remaining_str: &str) -> bool {
            if remaining_str == "" {
                return target_val == val.unwrap();
            }

            let (first, remainder) = remaining_str.split_once(' ').unwrap_or((remaining_str, ""));
            calculate_and_test_1(target_val, Some(val.unwrap_or_default() + first.parse::<u64>().unwrap()), remainder) |
            calculate_and_test_1(target_val, Some(val.unwrap_or(1) * first.parse::<u64>().unwrap()), remainder)
        }

        fn calculate_and_test_2(target_val: u64, val: Option<u64>, remaining_str: &str) -> bool {
            if remaining_str == "" {
                return target_val == val.unwrap();
            }

            let (first, remainder) = remaining_str.split_once(' ').unwrap_or((remaining_str, ""));
            calculate_and_test_2(target_val, Some(val.unwrap_or_default() + first.parse::<u64>().unwrap()), remainder) |
            calculate_and_test_2(target_val, Some(val.unwrap_or(1) * first.parse::<u64>().unwrap()), remainder) | {
                if let Some(val) = val {
                    calculate_and_test_2(target_val, Some(format!("{val}{first}").parse::<u64>().unwrap()), remainder)
                } else {
                    false
                }
            }
        }

        let (test_val, nums) = line.split_once(": ").unwrap();
        let test_val = test_val.parse::<u64>().unwrap();
        if calculate_and_test_1(test_val, None, nums) {
            sum1 += test_val;
        }
        if calculate_and_test_2(test_val, None, nums) {
            sum2 += test_val;
        }

        (sum1, sum2)
    });

    println!("{}\n{}", sum.0, sum.1);
}
