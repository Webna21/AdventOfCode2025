use std::io::{BufRead, BufReader};
use std::fs::File;
use std::env;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).ok_or("usage: <program> <file_path>")?;
    let (_p11, _p12) = _d1(&path)?;

    println!("day1 part1 result: {_p11}");
    println!("day1 part2 result: {_p12}");

    Ok(())
}

fn _d1(path: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let input = get_data(path)?;

    let d1_p1_result = _d1_p1(&input)?;
    let d1_p2_result = _d1_p2(&input)?;

    Ok((d1_p1_result, d1_p2_result))
}

fn _d1_p1(input: &Vec<(char, i32)>) -> Result<i32, Box<dyn Error>> {

    let mut zero_count = 0;
    let mut dial: i32 = 50;

    for instr in input {
        dial += match instr.0 {
            'L' => -instr.1,
            'R' => instr.1,
            _ => unreachable!("invalid direction"),
        };
        
        dial = dial.rem_euclid(100);

        if dial == 0 {
            zero_count += 1;
        }
    }

    Ok(zero_count)
}

fn _d1_p2(input: &Vec<(char, i32)>) -> Result<i32, Box<dyn Error>> {
    let mut zero_count = 0;
    let mut dial = 50;

    for (dir, mag) in input {
        match dir {
            'L' => {    // negative 
                zero_count += mag / 100;
                if dial != 0 && mag % 100 >= dial {
                    zero_count += 1;
                }
                dial = (dial - mag).rem_euclid(100);
            },
            'R' => {    // positive
                dial += mag;
                zero_count += dial / 100;
                dial = dial.rem_euclid(100);
            },
            _ => unreachable!("invalid direction"),
        }
    }

    Ok(zero_count)
}




fn get_data(path: &str) -> Result<Vec<(char, i32)>, Box<dyn Error>> {
    let file = File::open(path)?;

    let mut result = Vec::new();
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line?;
        result.push((line.chars().nth(0).unwrap(), line[1..].parse()?));
    }

    Ok(result)
}