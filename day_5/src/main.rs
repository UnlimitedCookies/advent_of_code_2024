use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let mut order_table: HashMap<_, HashSet<_>> = HashMap::new();
    rules.lines().for_each(|line| {
        let (first, second) = line.split_once('|').unwrap();
        order_table
            .entry(second.parse::<u32>().unwrap())
            .or_default()
            .insert(first.parse::<u32>().unwrap());
    });

    let sum = pages.lines().fold((0, 0), |(mut sum_1, mut sum_2), line| {
        let mut page_nums = line
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let mut unallowed: HashSet<u32> = HashSet::new();

        if page_nums.iter().all(|page_num| {
            if unallowed.get(&page_num).is_some() {
                return false;
            }
            if let Some(new_unallowed) = order_table.get(&page_num) {
                unallowed.extend(new_unallowed);
            }
            true
        }) {
            sum_1 += page_nums[page_nums.len() / 2];
        } else {
            page_nums.sort_by(|&a, &b| {
                if order_table
                    .get(&a)
                    .and_then(|unallowed| unallowed.get(&b))
                    .is_some()
                {
                    Ordering::Greater
                } else if order_table
                    .get(&b)
                    .and_then(|unallowed| unallowed.get(&a))
                    .is_some()
                {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });
            sum_2 += page_nums[page_nums.len() / 2];
        }
        (sum_1, sum_2)
    });
    println!("{}\n{}", sum.0, sum.1);
}
