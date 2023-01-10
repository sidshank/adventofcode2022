use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = BufReader::new(File::open("day10.txt").unwrap()).lines();

    let mut acc = 1;
    let mut cycle_numbers_set = std::collections::HashSet::new();
    let mut last_inserted_number = 0;
    for i in 0..6 {
        last_inserted_number = 20 + i * 40;
        cycle_numbers_set.insert(last_inserted_number);
    }

    let mut current_cycle_number = 0;
    let mut sum_of_signal_strengths = 0;
    for line in lines {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let command = split.next().unwrap();

        let number = match command {
            "addx" => split.next().unwrap().parse::<i32>().unwrap(),
            _ => 0,
        };
        
        match command {
            "addx" => {
                for _ in 0..2 {
                    current_cycle_number += 1;
                    if cycle_numbers_set.contains(&current_cycle_number) {
                        sum_of_signal_strengths += current_cycle_number * acc;
                    }
                }
                acc += number;
            },
            "noop" => {
                current_cycle_number += 1;
                if cycle_numbers_set.contains(&current_cycle_number) {
                    sum_of_signal_strengths += current_cycle_number * acc;
                }
            },
            _ => (),
        }
        if current_cycle_number == last_inserted_number {
            break;
        }
    }
    // Print the sum of the signal strengths
    println!("{}", sum_of_signal_strengths);

}