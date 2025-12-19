use std::fs;
use std::collections::HashSet;

fn main() {
    let (a, b) = _part1();
    println!("part1: {a}\npart2: {b}");
}


fn _part1() -> (i32, usize) {
    let input = fs::read_to_string("input/input.txt").expect("bad file read");

    let data_split: (&str, &str) = input.split_once("\n\n").expect("bad split_once");

    let fresh_ranges: Vec<(i64, i64)> = data_split.0
        .split_whitespace()
        .map(|range| {
            let (start, end) = range.split_once("-").expect("bad split_once on range");
            (start.parse::<i64>().expect("bad parse"), end.parse::<i64>().expect("bad parse"))
        })
        .collect();

    let ingredients: Vec<i64> = data_split.1
        .split_whitespace()
        .map(|ing| {
            ing.parse::<i64>().expect("bad parse")
        })
        .collect();

    // part 1
    let mut fresh_count = 0;
    for i in &ingredients {
        let mut is_fresh = false;

        for (start, end) in &fresh_ranges {
            if start <= i && i <= end {
                is_fresh = true;
                break;
            }
        }

        if is_fresh {
            fresh_count += 1;
        }
    }

    // part 2
        let mut reduced_fresh_ranges: Vec<(i64, i64)> = Vec::new();

        // subset/superset 
            // current.start < other.start < other.end < current.end
                // [current.start, current.end]
            // other.start < current.start < current.end < other.end
                // [other.start, other.end]

        // venn-diagram-like overlap
            // current.start < other.start < current.end < other.end
                // [current.start, other.end]
            // other.start < current.start < other.end < current.end
                // [other.start, current.end]

        // no overlap

    for (i, (c_start, c_end)) in fresh_ranges.into_iter().enumerate() {
        for (o_start, o_end) in fresh_ranges.into_iter().skip(i+1) {
            reduced_fresh_ranges.push()        
        }
    }


    (fresh_count, 123)
}

fn reduce_range(current: (i64, i64), other: (i64, i64)) -> (i64, i64) {
    if current.0 < other.0 && other.0 < other.1 && other.1 < current.1 {
        (current.0, current.1)
    } else if other.0 < current.0 && current.0 < current.1 && current.1 < other.1 {
        (other.0, other.1)
    } else if current.0 < other.0 && other.0 < current.1 && current.1 < other.1 {
        (current.0, other.1)
    } else {

    } else {

    }
}