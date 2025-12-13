use std::fs;

fn main() {
    let day1 = day1();
    let day2 = day2();

    println!("{day1}");
    println!("{day2}");

}

fn day1() -> i64 {
    let input = fs::read_to_string("input/input.txt").expect("could not read file");

    let vec: Vec<(i64, i64)> = input.trim().split(',').map(|pair| {
        let (a, b) = pair.split_once('-').expect("bad split once");
        (a.parse::<i64>().expect("failed i64 parse"), b.parse::<i64>().expect("failed i64 parse"))
    }).collect();

    let mut sum: i64 = 0;
    for &(start, end) in &vec {
        for num in start..=end {
            let num_str = num.to_string();
            let mid = num_str.len() / 2;
            let (l, r) = num_str.split_at(mid);
            if l == r {
                sum += num as i64;
            }
        }
    }

    sum
}

fn day2() -> i64 {
    let input = fs::read_to_string("input/input.txt").expect("could not read file");
    let ranges: Vec<(i64, i64)> = input.
        trim().
        split(',').
        map(|pair| {
            let pair = pair.trim();
            let (a, b) = pair.split_once('-').expect("bad split once");
            (a.parse::<i64>().expect("failed parse"), 
             b.parse::<i64>().expect("failed parse"))
        }).collect();

    let mut sum: i64 = 0;
    for &(start, end) in &ranges {
        for num in start..=end {
            if has_repeating_groups(num) {
                sum += num;
            }
        }
    }

    sum
}

fn has_repeating_groups(num: i64) -> bool {
    let s = num.to_string();
    let n = s.len();

    // groups = number of repeats (2..=n)
    for groups in 2..=n {
        if n % groups != 0 {
            continue;
        }
        let chunk = n / groups;
        let first = &s[0..chunk];

        if (1..groups).all(|g| &s[g * chunk..(g + 1) * chunk] == first) {
            return true;
        }
    }
    false
}
