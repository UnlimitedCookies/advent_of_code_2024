fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut count: u32 = 0;
    count += input.match_indices("XMAS").count() as u32;
    count += input.match_indices("SAMX").count() as u32;

    for i in 0..(lines.len() - 3) {
        for j in 0..(lines[0].len() - 3) {
            let cases = [
                [(3, 0), (2, 1), (1, 2), (0, 3)], // up right
                [(0, 0), (1, 1), (2, 2), (3, 3)], // down right
                [(3, 3), (2, 2), (1, 1), (0, 0)], // up left
                [(0, 3), (1, 2), (2, 1), (3, 0)], // down left
            ];
            count += cases
                .iter()
                .filter(|cond| {
                    cond.iter()
                        .zip("XMAS".chars())
                        .all(|case| lines[i + case.0.0][j + case.0.1] == case.1)
                })
                .count() as u32;
        }
    }

    for i in 0..(lines.len() - 3) {
        for j in 0..(lines[0].len()) {
            let cases = [
                [(3, 0), (2, 0), (1, 0), (0, 0)], // straight up
                [(0, 0), (1, 0), (2, 0), (3, 0)], // straight down
            ];
            count += cases
                .iter()
                .filter(|cond| {
                    cond.iter()
                        .zip("XMAS".chars())
                        .all(|case| lines[i + case.0.0][j + case.0.1] == case.1)
                })
                .count() as u32;
        }
    }
    println!("{count}");

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