use std::{fs, io};
use std::io::{Write, Result};

fn main() {
    // println!("part1: {}", part1("input/test.txt", "output/output1.txt").expect("bad write"));
    // println!("part2: {}", part2("input/input.txt"));
    draw();
}

fn draw() {
    let filename_base: (&str, &str) = ("output/output", ".txt");
    let mut filename_number = 1;

    loop {
        let next_filename = format!("{}{}{}", filename_base.0, filename_number, filename_base.1);

        print!("\x1B[2J\x1B[H");

        match fs::read_to_string(&next_filename) {

            Ok(image) => {
                print!("{}", image);
                io::stdout().flush().unwrap();

                let mut line = String::new();
                io::stdin().read_line(&mut line).unwrap();

                filename_number += 1;
            },

            Err(_) => break,
        }
    }
}



fn _part2(filename: &str) -> i32 {
    let filename_base: (&str, &str) = ("output/output", ".txt");

    let mut filenames = vec![filename.to_owned()];
    let mut filename_number = 1;

    let mut sum = 0;

    loop {
        let new_filename = format!("{}{}{}", filename_base.0, filename_number, filename_base.1);
        let result = _part1(&filenames.last().expect("empty filenames vec"), &new_filename).expect("bad file write");
        filenames.push(new_filename.to_owned());

        filename_number += 1;
        sum += result;

        if result == 0 {
            break;
        }
    }

    sum
}

fn _part1(filename: &str, new_filename: &str) -> Result<i32> {
    let input = fs::read_to_string(filename).expect("bad file read");
    let mut new_file = fs::File::create(new_filename).expect("bad file create");

    let grid: Vec<Vec<char>> = input.
        split_whitespace()
        .map(|line| {
            line.chars().map(|c| c).collect()
        })
        .collect();


    let mut result = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch != '@' {
                write!(new_file, ".")?;
                continue;
            }
            if _local_obstacle_count(&grid, i, j) < 4 {
                result += 1;
                write!(new_file, "x")?;
            } else {
                write!(new_file, "@")?;
            }
        }
        writeln!(new_file)?;
    }

    Ok(result)
}


fn _local_obstacle_count(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    // print!("({}, {}): ", i, j);

    let mut local_obstacles = 0;

    for drow in -1_isize..=1 {
        for dcol in -1_isize..=1 {
            if (drow, dcol) == (0, 0) {
                continue;
            }
            let di = i as isize + drow;
            let dj = j as isize + dcol;

            if !(di >= 0 && dj >= 0 && di < (grid.len() as isize) && dj < (grid[0].len() as isize)) {
                continue;
            }

            if grid[di as usize][dj as usize] == '@' {
                local_obstacles += 1;
                // print!("({}, {}), ", di, dj);
            }
            
        }
    }

    // println!();

    local_obstacles
}