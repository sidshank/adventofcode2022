// Must be compiled using `cargp build` and run using `cargo run`
use std::fs::File;
use std::io::{BufRead, BufReader};
extern crate regex;
use regex::Regex;

fn main() {
    let mut lines = BufReader::new(File::open("day5.txt").unwrap()).lines();

    let mut is_first_line = true;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if !line.contains("[") {
            break;
        }
        let line_length = line.len();

        let number_of_stacks = (line_length + 1) / 4;
        let bytes = line.as_bytes();
        for i in 0..number_of_stacks {
            let index = i * 4 + 1;

            let b: u8 = bytes[index];
            let crate_label: char = b as char;

            if is_first_line {
                let mut stack_of_crates = Vec::new();

                if crate_label != ' ' {
                    stack_of_crates.push(crate_label);
                }
                stacks.push(stack_of_crates);
            } else {
                let stack_of_crates = &mut stacks[i];
                if crate_label != ' ' {
                    stack_of_crates.insert(0, crate_label);
                }
            }
        }
        is_first_line = false;
    }
    lines.next();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let re = Regex::new(r"\d+").unwrap();
        let mut numbers = re.find_iter(&line);
        let num_crates = numbers.next().unwrap().as_str().parse::<usize>().unwrap();
        let source = numbers.next().unwrap().as_str().parse::<usize>().unwrap() - 1;
        let destination = numbers.next().unwrap().as_str().parse::<usize>().unwrap() - 1;

        let source_stack = &mut stacks[source];

        let mut temp_stack = Vec::new();
        for _ in 0..num_crates {
            let crate_label = source_stack.pop().unwrap();
            temp_stack.insert(0, crate_label);
        }
        drop(source_stack);

        let destination_stack = &mut stacks[destination];

        // To execute part 1, make sure the lines below are uncommented, and comment out part 2
        // for _ in 0..num_crates {
        //     let crate_label = temp_stack.pop().unwrap();
        //     destination_stack.push(crate_label);
        // }

        // To execute Part 2, make sure the lines below are uncommented, and comment out part 1
        for crate_label in temp_stack {
            destination_stack.push(crate_label);
        }
    }

    for stack in stacks {
        let crate_label = stack.last().unwrap();
        print!("{} ", crate_label);
    }
}
