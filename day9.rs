use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Define a constant for the number of knots in the rope
    const NUMBER_OF_KNOTS: u8 = 2;

    let lines = BufReader::new(File::open("day9.txt").unwrap()).lines();

    let moves: Vec<(char, u32)> = lines
        .map(|line| {
            let line = line.unwrap();
            let c = line.chars().nth(0).unwrap();
            let n = line[2..].parse::<u32>().unwrap();
            (c, n)
        })
        .collect();

    let mut visited = std::collections::HashSet::new();
    let mut knots: Vec<(i32, i32)> = Vec::new();
    for _ in 0..NUMBER_OF_KNOTS {
        knots.push((0, 0));
    }

    let mut add_to_visited = |x: i32, y: i32| {
        visited.insert(format!("{}-{}", x, y));
    };

    add_to_visited(0, 0);

    for (c, n) in moves {
        for k in 0..(NUMBER_OF_KNOTS - 1) as usize {
            let head_index = k;
            let tail_index = k + 1;

            let mut head = knots[head_index];
            let mut tail = knots[tail_index];
            match c {
                'U' => {
                    for _ in 0..n {
                        head.1 += 1;
                    }
                }
                'D' => {
                    for _ in 0..n {
                        head.1 -= 1;
                    }
                }
                'R' => {
                    for _ in 0..n {
                        head.0 += 1;
                    }
                }
                'L' => {
                    for _ in 0..n {
                        head.0 -= 1;
                    }
                }
                _ => {}
            }
            knots[head_index] = head;
            let mut diff_x = head.0 - tail.0;
            let mut diff_y = head.1 - tail.1;
            if diff_x.abs() < 2 && diff_y.abs() < 2 {
                continue;
            }
            if diff_x.abs() == 1 {
                tail.0 = head.0;
                diff_x = 0;
            } else if diff_y.abs() == 1 {
                tail.1 = head.1;
                diff_y = 0;
            }
            if diff_x == 0 {
                if head.1 > tail.1 {
                    while head.1 > tail.1 + 1 {
                        tail.1 += 1;
                        if tail_index == NUMBER_OF_KNOTS as usize - 1 {
                            add_to_visited(tail.0, tail.1);
                        }
                    }
                } else {
                    while head.1 < tail.1 - 1 {
                        tail.1 -= 1;
                        if tail_index == NUMBER_OF_KNOTS as usize - 1 {
                            add_to_visited(tail.0, tail.1);
                        }
                    }
                }
            } else {
                // diff_y = 0
                if head.0 > tail.0 {
                    while head.0 > tail.0 + 1 {
                        tail.0 += 1;
                        if tail_index == NUMBER_OF_KNOTS as usize - 1 {
                            add_to_visited(tail.0, tail.1);
                        }
                    }
                } else {
                    while head.0 < tail.0 - 1 {
                        tail.0 -= 1;
                        if tail_index == NUMBER_OF_KNOTS as usize - 1 {
                            add_to_visited(tail.0, tail.1);
                        }
                    }
                }
            }
            knots[tail_index] = tail;
        }
    }
    // Part 1 (Ans for day.txt : 6354)
    println!("The tail visits {} locations", visited.len());
}
