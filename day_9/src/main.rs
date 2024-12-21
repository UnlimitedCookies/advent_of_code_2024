use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockPos {
    File(usize),
    Free,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut file_system_1 = input.chars().filter(|c| *c != '\n').enumerate().flat_map(| (i, c)| {
        let len = c.to_digit(10).unwrap() as usize;
        match i % 2 {
            0 => {
                vec![BlockPos::File(i / 2); len]
            }
            1 => {
                vec![BlockPos::Free; len]
            }
            _ => unreachable!()
        }
    }).collect::<Vec<BlockPos>>();
    let mut file_system_2 = file_system_1.clone();

    // part 1
    let mut i = file_system_1.len() - 1;
    let mut free_pos = 0;
    loop {
        while file_system_1[free_pos] != BlockPos::Free {
            free_pos += 1;
        }
        while file_system_1[i] == BlockPos::Free {
            i -= 1;
        }
        if free_pos >= i {
            break;
        }

        file_system_1[free_pos] = file_system_1[i];
        file_system_1[i] = BlockPos::Free;
    }

    let sum_1 = file_system_1.iter().take_while(|b| **b != BlockPos::Free).enumerate().map(|(i, b)| {
        match *b {
            BlockPos::File(file_id) => i * file_id,
            _ => unreachable!()
        }
    }).sum::<usize>();

    // part 2
    let mut j = file_system_2.len() - 1;
    loop {
        while j > 0 && file_system_2[j] == BlockPos::Free {
            j -= 1;
        }

        let mut i = j;
        while i > 0 && file_system_2[i - 1] == file_system_2[j] {
            i -= 1;
        }

        let len = j - i + 1;
        if len == 0 {
            break;
        }

        if let Some(free_pos) = find_free_pos(&file_system_2, i, len) {
            for (new, old) in free_pos.zip(i..=j) {
                file_system_2[new] = file_system_2[old];
                file_system_2[old] = BlockPos::Free;
            }
        }

        if i > 0 {
            j = i - 1;
        } else {
            break;
        }
    }

    let sum_2 = file_system_2.iter().enumerate().filter(|(_, b)| **b != BlockPos::Free).map(|(i, b)| {
        match *b {
            BlockPos::File(file_id) => i * file_id,
            _ => unreachable!()
        }
    }).sum::<usize>();
    println!("{sum_1}\n{sum_2}");
}

fn find_free_pos(file_system: &Vec<BlockPos>, right_bound: usize, len: usize) -> Option<Range<usize>> {
    file_system[..right_bound].windows(len).position(|slice| {
        slice.iter().all(|b| *b == BlockPos::Free)
    }).map(|start| (start..start + len))
}