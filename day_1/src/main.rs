fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    // part 1
    let (mut list_l, mut list_r) = input.lines().map(|line| {
        let tuple = line.split_once("   ").unwrap();
        (tuple.0.parse::<u32>().unwrap(), tuple.1.parse::<u32>().unwrap())
    }).collect::<(Vec<_>, Vec<_>)>();
    list_l.sort();
    list_r.sort();

    let sum = &list_l.iter().zip(&list_r).map(|(l, r)| {
        l.abs_diff(*r)
    }).sum::<u32>();
    println!("{sum}");

    // part 2
    let similarity_score = list_l.iter().map(|l| {
        l * list_r.iter().filter(|r| l == *r).count() as u32
    }).sum::<u32>();
    println!("{similarity_score}");
}
