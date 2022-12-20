use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum: i32 = 0;
    let mut calorie_sums: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        
        if line.is_empty() {
            calorie_sums.push(sum);
            sum = 0;
            continue;
        }
        // let num: i32 = line.parse().unwrap();
        sum += line.parse().unwrap();
    }
    if sum != 0 {
        calorie_sums.push(sum);
    }

    calorie_sums.sort_by(|a, b| b.cmp(a));
    println!("{}", calorie_sums[0]);
    println!("{}", calorie_sums[0] + calorie_sums[1] + calorie_sums[2]);
}