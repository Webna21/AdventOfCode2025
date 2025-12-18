use std::fs;

fn main() {
    let _part1 = part1();
    println!("part1: {_part1}");

    let part2 = part2();
    println!("part2: {part2}");
}

fn part2() -> i128 {
    let input = fs::read_to_string("input/input.txt")
        .expect("bad file read");

    let vec: Vec<Vec<i128>> = input
        .split_whitespace()
        .map(|s| {
            s.chars().map(|r| {
                r.to_digit(10).unwrap() as i128
            }).collect()
        }).collect();

    let mut sum: i128 = 0;

    for row in &vec {
        let mut joltage: i128 = 0;
        let mut next_index_start: usize = 0;
        
        for iter_count in (2..=12).rev() {
            let (index, joltage_digit) = range_max(&row, next_index_start, row.len()-iter_count+1);

            joltage = joltage * 10 + joltage_digit;
            next_index_start = index + 1;
        }

        let (_, joltage_digit) = range_max(&row, next_index_start, row.len());
        joltage = joltage * 10 + joltage_digit;

        assert!(joltage.to_string().len() == 12);


        sum += joltage;
    }

    sum
}


fn range_max(vec: &Vec<i128>, start: usize, end: usize) -> (usize, i128) {
    let mut max_index: usize = start;
    for (i, e) in vec[start..end].iter().enumerate() {
        if e > &vec[max_index] {
            max_index = i + start;
        }
    }
    (max_index, vec[max_index])
}




fn part1() -> i32 {
    let input = fs::read_to_string("input/input.txt")
        .expect("file read error");

    let vec: Vec<Vec<i32>> = input
        .split_whitespace()
        .map(|s| {
            s.chars().map(|r| {
                r.to_digit(10).unwrap() as i32
            }).collect()
        })
        .collect();

    let mut sum = 0;

    for row in &vec {
        let mut first_index = 0;
        for (i, e) in row[..row.len()-1].iter().enumerate() {
            if e > &row[first_index] {
                first_index = i;
            }
        }
        let mut second = 0;
        for e in &row[first_index+1..] {
            if *e > second {
                second = *e;
            }
        }
        sum += 10*&row[first_index] + second;
    }

    sum
}