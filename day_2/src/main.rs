#[derive(Default)]
struct ReportAcc {
    last: Option<u32>,
    ascending: Option<bool>,
    val: bool,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let sum = input.lines().filter(|line| {
        let fold_val = line.split(" ").fold(ReportAcc::default(), |mut acc, v| {
            let new_val = v.parse::<u32>().unwrap();
            if acc.last.is_none() {
                acc.last = Some(new_val);
                acc.val = true;
            } else {
                if acc.val == false {
                    return acc;
                }

                if acc.last.is_some() {
                    let diff = acc.last.unwrap() as i64 - new_val as i64;
                    if diff < 0 {
                        if acc.ascending.is_none() {
                            acc.ascending = Some(true);
                        } else {
                            acc.val = acc.ascending.unwrap() == true;
                        }
                    }
                    if diff > 0 {
                        if acc.ascending.is_none() {
                            acc.ascending = Some(false);
                        } else {
                            acc.val = acc.ascending.unwrap() == false;
                        }
                    }
                }
                let diff = acc.last.unwrap().abs_diff(new_val);
                if 1 <= diff && diff <= 3 {
                    acc.last = Some(new_val);
                } else {
                    acc.val = false;
                }
            }
            acc
        });
        fold_val.val
    }).count();
    println!("{sum}");

    let sum = input.lines().filter(|line| {
        let line_iter = line.split(" ");
        (0..line_iter.clone().count()).any(|i| {
            let fold_val = line_iter.clone().enumerate().filter(|&(j, _)| i != j).map(|(_, v)| v).fold(ReportAcc::default(), |mut acc, v| {
                let new_val = v.parse::<u32>().unwrap();
                if acc.last.is_none() {
                    acc.last = Some(new_val);
                    acc.val = true;
                } else {
                    if acc.val == false {
                        return acc;
                    }

                    if acc.last.is_some() {
                        let diff = acc.last.unwrap() as i64 - new_val as i64;
                        if diff < 0 {
                            if acc.ascending.is_none() {
                                acc.ascending = Some(true);
                            } else {
                                acc.val = acc.ascending.unwrap() == true;
                            }
                        }
                        if diff > 0 {
                            if acc.ascending.is_none() {
                                acc.ascending = Some(false);
                            } else {
                                acc.val = acc.ascending.unwrap() == false;
                            }
                        }
                    }
                    let diff = acc.last.unwrap().abs_diff(new_val);
                    if 1 <= diff && diff <= 3 {
                        acc.last = Some(new_val);
                    } else {
                        acc.val = false;
                    }
                }
                acc
            });
            fold_val.val
        })
    }).count();
    println!("{sum}");
}
