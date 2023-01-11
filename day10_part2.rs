use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = BufReader::new(File::open("day10.txt").unwrap()).lines();

    let mut sprite_midpoint = 1;
    let mut current_cycle_number = 0;
    const CRT_WIDTH:i32 = 40;

    let print_dot_or_hash = |sprite_midpoint: i32, current_cycle_number: i32| {
        let cursor_position = current_cycle_number % CRT_WIDTH;
        if cursor_position == 0 {
            println!("");
        }
        if cursor_position >= sprite_midpoint - 1 && cursor_position <= sprite_midpoint + 1 {
            print!("#");
        } else {
            print!(".");
        }
    };

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
                    print_dot_or_hash(sprite_midpoint, current_cycle_number);
                    current_cycle_number += 1;
                }
                sprite_midpoint += number;
            }
            "noop" => {
                print_dot_or_hash(sprite_midpoint, current_cycle_number);
                current_cycle_number += 1;
            }
            _ => (),
        }
    }
    println!("");
}
