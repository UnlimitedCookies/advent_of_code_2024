struct Case<'a> {
    x_limit: usize,
    y_limit: usize,
    positions: &'a [(usize, usize)],
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    // part 1
    let cases = [
        (3, 0, &[(0, 0), (0, 1), (0, 2), (0, 3)]), // straight right
        (3, 0, &[(0, 3), (0, 2), (0, 1), (0, 0)]), // straight left
        (0, 3, &[(3, 0), (2, 0), (1, 0), (0, 0)]), // straight up
        (0, 3, &[(0, 0), (1, 0), (2, 0), (3, 0)]), // straight down
        (3, 3, &[(3, 0), (2, 1), (1, 2), (0, 3)]), // right up
        (3, 3, &[(0, 0), (1, 1), (2, 2), (3, 3)]), // right down
        (3, 3, &[(3, 3), (2, 2), (1, 1), (0, 0)]), // left up
        (3, 3, &[(0, 3), (1, 2), (2, 1), (3, 0)]), // left down
    ]
    .map(|case| Case {
        x_limit: case.0,
        y_limit: case.1,
        positions: case.2,
    });
    println!("{}", count_cases(&lines, &cases, "XMAS"));

    // part 2
    let mut count = 0;
    for i in 0..(lines.len() - 2) {
        for j in 0..(lines[0].len() - 2) {
            let cases = [
                [(2, 0), (0, 2)], // up right
                [(0, 0), (2, 2)], // down right
                [(2, 2), (0, 0)], // up left
                [(0, 2), (2, 0)], // down left
            ];
            if lines[i + 1][j + 1] == 'A'
                && cases
                    .iter()
                    .filter(|cond| {
                        cond.iter()
                            .zip("MS".chars())
                            .all(|case| lines[i + case.0.0][j + case.0.1] == case.1)
                    })
                    .count()
                    >= 2
            {
                count += 1;
            }
        }
    }
    println!("{count}");
}

#[inline]
fn count_cases(lines: &Vec<Vec<char>>, cases: &[Case], chars: &str) -> u32 {
    cases
        .iter()
        .map(|case| {
            (0..(lines.len() - case.y_limit))
                .map(|i| {
                    (0..(lines[0].len() - case.x_limit))
                        .filter(|j| {
                            case.positions
                                .iter()
                                .zip(chars.chars())
                                .all(|case| lines[i + case.0.0][j + case.0.1] == case.1)
                        })
                        .count() as u32
                })
                .sum::<u32>()
        })
        .sum()
}
