fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut d = true;
    let re = regex::Regex::new(r"(do\(\))|(don't\(\))|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let sum = re
        .captures_iter(&input)
        .fold((0, 0), |(mut sum_1, mut sum_2), caps| {
            if caps.get(1).is_some() {
                d = true;
            } else if caps.get(2).is_some() {
                d = false;
            } else {
                let product = [3, 4]
                    .map(|i| caps.get(i).unwrap().as_str().parse::<u32>().unwrap())
                    .iter()
                    .product::<u32>();
                sum_1 += product;
                if d {
                    sum_2 += product;
                }
            }
            (sum_1, sum_2)
        });
    println!("{}\n{}", sum.0, sum.1);
}
