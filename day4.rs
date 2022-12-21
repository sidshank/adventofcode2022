use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = BufReader::new(File::open("day4.txt").unwrap()).lines();
    let mut complete_overlap_count = 0;
    let mut partial_overlap_count = 0;
    for line in lines {
        let line = line.unwrap();
        let mut nums = line.split(|c| c == '-' || c == ',');
        let first = nums.next().unwrap().parse::<i32>().unwrap();
        let second = nums.next().unwrap().parse::<i32>().unwrap();
        let third = nums.next().unwrap().parse::<i32>().unwrap();
        let fourth = nums.next().unwrap().parse::<i32>().unwrap();

        let range_start_diffs = first - third;
        let range_end_diffs = second - fourth;

        if (range_start_diffs >=0 && range_end_diffs <= 0) || (range_start_diffs <= 0 && range_end_diffs >= 0) {
            println!("{} {} {} {} are in complete overlap", first, second, third, fourth);
            complete_overlap_count += 1;
        } else if (first >= third && first <= fourth) || (second >= third && second <= fourth) {
            println!("{} {} {} {} are in partial overlap", first, second, third, fourth);
            partial_overlap_count += 1;
        }
    }
    // print the count of complete overlaps (part 1)
    println!("{}", complete_overlap_count);

    // print the count of complete overlaps + partial overlaps (part 2)
    println!("{}", complete_overlap_count + partial_overlap_count);
    
}
