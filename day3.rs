use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_priority(c: char) -> i32 {
    if c.is_lowercase() {
        // a-z have priority 1 - 26
        return c as i32 - 96;
    } else if c.is_uppercase() {
        // A-Z have priority 27 - 52
        return c as i32 - 38;
    } else {
        return 0;
    }
}

fn main() {
    let lines = BufReader::new(File::open("day3.txt").unwrap()).lines();
    let mut sum_of_priorities = 0;
    for line in lines {
        let unwrapped_line = line.unwrap();
        let line_length = unwrapped_line.chars().count();

        let mut first_half = std::collections::HashSet::new();
        for c in unwrapped_line.chars().take(line_length / 2) {
            first_half.insert(c);
        }

        for c in unwrapped_line.chars().skip(line_length / 2) {
            if first_half.contains(&c) {
                sum_of_priorities += get_priority(c);
                break;
            }
        }
    }
    // Part 1
    println!("{}", sum_of_priorities);

    let lines = BufReader::new(File::open("day3.txt").unwrap()).lines();
    let mut sum_of_priorities_common = 0;
    let mut lines_iter = lines.into_iter();
    
    while let Some(line1) = lines_iter.next() {
        let line2 = lines_iter.next().unwrap();
        let line3 = lines_iter.next().unwrap();

        let line1 = line1.unwrap();
        let line2 = line2.unwrap();
        let line3 = line3.unwrap();
        for c in line1.chars() {
            if line2.contains(c) && line3.contains(c) {
              sum_of_priorities_common += get_priority(c);
                break;
            }
        }
    }

    // Part 2
    println!("{}", sum_of_priorities_common);
}
