fn main() {
    let input = std::fs::read_to_string("test-input").unwrap();
    let mut char_map = input.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<_>>>();
    let mut u8_map: Vec<Vec<u8>> = vec![vec![0; char_map[0].len()]; char_map.len()];
    let (mut y, mut x) = char_map.iter().enumerate().filter_map(|(i, line)| {
        if let Some(x) = line.iter().position(|c| *c == '^') {
            Some((i, x))
        } else {
            None
        }
    }).next().unwrap();
    char_map[y][x] = 'X';
    dbg!(y, x);

    let _ = [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().cycle().take_while(|direction: &&(i32, i32)| {
        let dir_bit: u8 = match direction {
            (-1, 0) => 0b1000, // up
            (0, 1) => 0b0100, // right
            (1, 0) => 0b0010, //down
            (0, -1) => 0b0001, // left
            _ => unreachable!()
        };
        
        loop {
            let (new_y, new_x) = ((y as i32 + direction.0) as usize, (x as i32 + direction.1) as usize);
            if y as i32 + direction.0 < 0 || new_y >= char_map[0].len() || x as i32 + direction.1 < 0 || new_x >= char_map.len() {
                return false;
            }
            if char_map[new_y][new_x] == '#' {
                return true;
            }
                        
            u8_map[y][x] |= dir_bit;
            (y, x) = (new_y, new_x);
            char_map[y][x] = 'X';

        }
    }).take(10).count();
    
    char_map.iter().for_each(|y| {y.iter().for_each(|c| print!("{c}")); println!()});
    u8_map.iter().for_each(|y| {y.iter().for_each(|&b| {
        if count_intersection(b) > 0 {
            print!("\x1b[93m{b:05b}\x1b[0m, ")
        } else {
            print!("{b:05b}, ")
        }

    }); println!()});
    let sum = char_map.iter().map(|y| y.iter().filter(|c| **c == 'X').count() as u32).sum::<u32>();
    let sum2 = u8_map.iter().flat_map(|y| y.iter().map(|&bitmap| {
        count_intersection(bitmap)
    })).sum::<u32>();
    println!("{sum}\n{sum2}");
}

fn count_intersection(bitmap: u8) -> u32 {
    [0b1100, 0b0110, 0b0011, 0b1001].iter().filter(|&&bs| (bitmap & bs) == bs).count() as u32

}