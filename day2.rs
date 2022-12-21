use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_your_score(elf_turn_value: i32, your_response_value: i32) -> i32 {
    if elf_turn_value == your_response_value {
        // draw
        return your_response_value + 3;
    }
    let difference = elf_turn_value - your_response_value;
    let abs_difference = difference.abs();

    if (abs_difference == 1) {
        if (difference > 0) {
            // you lose
            return your_response_value + 0;
        } else {
            // you win
            return your_response_value + 6;
        }
    }
    // If we are here, then the difference is 2
    if (difference < 0) {
        // elf got rock, you got scissors
        // you lose
        return your_response_value + 0;
    } else {
        // elf got scissors, you got rock
        // you win
        return your_response_value + 6;
    }
}

fn calculate_your_strategic_score(elf_turn_value: i32, your_response: &str) -> i32 {
    let your_score;
    let round_score;
    if your_response == "Y" {
        // This is a draw
        your_score = elf_turn_value;
        round_score = 3;
    } else if your_response == "X" {
        // This means you need to lose
        round_score = 0;

        if elf_turn_value == 1 {
            your_score = 3;
        } else if elf_turn_value == 2 {
            your_score = 1;
        } else {
            your_score = 2;
        }
    } else {
        // you need to win
        round_score = 6;
        if elf_turn_value == 1 {
            your_score = 2;
        } else if elf_turn_value == 2 {
            your_score = 3;
        } else {
            your_score = 1;
        }
    }
    return round_score + your_score;
}

fn main() {
    let mut rock_paper_scissor_values = HashMap::new();
    rock_paper_scissor_values.insert('A', 1);
    rock_paper_scissor_values.insert('X', 1);
    rock_paper_scissor_values.insert('B', 2);
    rock_paper_scissor_values.insert('Y', 2);
    rock_paper_scissor_values.insert('C', 3);
    rock_paper_scissor_values.insert('Z', 3);

    let lines = BufReader::new(File::open("day2.txt").unwrap()).lines();
    let mut total = 0;
    let mut strategic_total = 0;
    for line in lines {
        let unwrapped_line = line.unwrap();
        let mut words = unwrapped_line.split_whitespace();

        let elf_turn = words.next().unwrap();
        let your_response = words.next().unwrap();

        let elf_turn_value = rock_paper_scissor_values
            .get(&elf_turn.chars().next().unwrap())
            .unwrap();
        let your_response_value = rock_paper_scissor_values
            .get(&your_response.chars().next().unwrap())
            .unwrap();

        let your_score = calculate_your_score(*elf_turn_value, *your_response_value);
        let your_strategic_score = calculate_your_strategic_score(*elf_turn_value, your_response);

        total += your_score;
        strategic_total += your_strategic_score;
    }

    // print the total (part 1)
    println!("{}", total);

    // print the strategic_total (part 2)
    println!("{}", strategic_total);
}
