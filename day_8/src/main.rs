use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let bounds = (input.lines().next().unwrap().chars().count(), input.lines().count());
    let coord_map: HashMap<_, Vec<_>> =
        input
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut map, (y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .for_each(|(x, c)| {
                        map.entry(c).or_default().push((x as i32, y as i32));
                    });
                map
            });

    let coord_set = coord_map.iter().fold(
        (HashSet::new(), HashSet::new()),
        |(mut set_1, mut set_2), (_, coords)| {
            coords
                .iter()
                .tuple_combinations()
                .for_each(|(first, second)| {
                    let diff = (first.0 - second.0, first.1 - second.1);
                    for (f, mut coords) in [
                        (Add::add as fn(i32, i32) -> i32, (first.0, first.1)),
                        (Sub::sub as fn(i32, i32) -> i32, (second.0, second.1)),
                    ] {
                        for i in 0.. {
                            if !check_bounds(bounds, coords) {
                                break;
                            }

                            if i == 1 {
                                set_1.insert((coords.0, coords.1));
                            }
                            set_2.insert(coords);
                            coords = (f(coords.0, diff.0), f(coords.1, diff.1));
                        }
                    }
                });
            (set_1, set_2)
        },
    );

    println!("{}\n{}", coord_set.0.len(), coord_set.1.len());
}

fn check_bounds(bounds: (usize, usize), coords: (i32, i32)) -> bool {
    0 <= coords.0 && coords.0 < bounds.0 as i32 && 0 <= coords.1 && coords.1 < bounds.1 as i32
}
