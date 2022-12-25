use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = BufReader::new(File::open("day8.txt").unwrap()).lines();
    let numbers: Vec<Vec<u32>> = lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    // Determine number of trees visible at the edge of the forest
    let mut number_of_visible_trees = numbers.len() * 2 + numbers[0].len() * 2 - 4;

    let mut set = std::collections::HashSet::new();

    let mut insert_if_visible =
        |height_of_tree: u32, height_of_last_tallest_tree: u32, i: usize, j: usize| -> u32 {
            let mut updated_tallest_tree = height_of_last_tallest_tree;
            if height_of_tree > height_of_last_tallest_tree {
                updated_tallest_tree = height_of_tree;
                if !set.contains(&format!("{}-{}", i, j)) {
                    set.insert(format!("{}-{}", i, j));
                }
            }
            return updated_tallest_tree;
        };

    'outer1: for i in 1..numbers.len() - 1 {
        let mut height_of_last_visible_left_tree = numbers[i][0];
        for j in 1..numbers[0].len() - 1 {
            if height_of_last_visible_left_tree == 9 {
                continue 'outer1;
            }
            let height = numbers[i][j];
            height_of_last_visible_left_tree =
                insert_if_visible(height, height_of_last_visible_left_tree, i, j);
        }
    }

    'outer2: for i in 1..numbers.len() - 1 {
        let mut height_of_last_visible_right_tree = numbers[i][numbers[0].len() - 1];
        for j in (1..numbers[0].len() - 1).rev() {
            if height_of_last_visible_right_tree == 9 {
                continue 'outer2;
            }
            let height = numbers[i][j];
            height_of_last_visible_right_tree =
                insert_if_visible(height, height_of_last_visible_right_tree, i, j);
        }
    }

    'outer3: for j in 1..numbers[0].len() - 1 {
        let mut height_of_last_visible_top_tree = numbers[0][j];
        for i in 1..numbers.len() - 1 {
            if height_of_last_visible_top_tree == 9 {
                continue 'outer3;
            }

            let height = numbers[i][j];
            height_of_last_visible_top_tree =
                insert_if_visible(height, height_of_last_visible_top_tree, i, j);
        }
    }

    'outer4: for j in 1..numbers[0].len() - 1 {
        let mut height_of_last_visible_bottom_tree = numbers[numbers.len() - 1][j];
        for i in (1..numbers.len() - 1).rev() {
            if height_of_last_visible_bottom_tree == 9 {
                continue 'outer4;
            }

            let height = numbers[i][j];
            height_of_last_visible_bottom_tree =
                insert_if_visible(height, height_of_last_visible_bottom_tree, i, j);
        }
    }

    number_of_visible_trees += set.len();

    // Part 1
    println!("Total number of visible trees from edge is {}", number_of_visible_trees);

    let mut max = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers[0].len() {
            let height = numbers[i][j];
            let mut left = 0;
            let mut right = 0;
            let mut up = 0;
            let mut down = 0;
            for k in 1..j + 1 {
                left+=1;
                if numbers[i][j - k] >= height {
                    break;
                }
            }
            for k in 1..numbers[0].len() - j {
                right+=1;
                if numbers[i][j + k] >= height {
                    break;
                }
            }
            for k in 1..i + 1 {
                up+=1;
                if numbers[i - k][j] >= height {
                    break;
                }
            }
            for k in 1..numbers.len() - i {
                down+=1;
                if numbers[i + k][j] >= height {
                    break;
                }
            }
            let product = left * right * up * down;
            if product > max {
                max = product;
            }
        }
    }
    // Part 2
    println!("Highest scenic score is {}", max);

}
