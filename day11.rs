use std::fs::File;
use std::io::{BufRead, BufReader};

struct Monkey {
    operation: String,
    operand: String,
    test_operand: u32,
    test_positive_recipient: u32,
    test_negative_recipient: u32,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            operation: String::new(),
            operand: String::new(),
            test_operand: 0,
            test_positive_recipient: 0,
            test_negative_recipient: 0,
        }
    }

    fn inspect_item(&self, item: u32) -> u32 {
        let mut new_item = 0;
        if self.operation == "+" {
            if self.operand == "old" {
                new_item = item + item;
            } else {
                new_item = item + self.operand.parse::<u32>().unwrap();
            }
        } else if self.operation == "*" {
            if self.operand == "old" {
                new_item = item * item;
            } else {
                new_item = item * self.operand.parse::<u32>().unwrap();
            }
        }
        (new_item as f32 / 3.0).floor() as u32
    }

    fn get_inspection_results(&self, items: Vec<u32>) -> Vec<(u32, u32)> {
        let mut results = Vec::new();
        for item in &items {
            let inspected_item = self.inspect_item(*item);
            if self.test_item(inspected_item) {
                results.push((inspected_item, self.test_positive_recipient));
            } else {
                results.push((inspected_item, self.test_negative_recipient));
            }
        }

        results
    }

    fn test_item(&self, item: u32) -> bool {
        item % self.test_operand == 0
    }
}

fn main() {
    let lines = BufReader::new(File::open("day11.txt").unwrap()).lines();

    let mut monkeys = Vec::new();
    let mut current_monkey;
    let mut monkey = Monkey::new();
    let mut items: Vec<Vec<u32>> = Vec::new();

    let mut inspection_counts: Vec<u32> = Vec::new();

    for line in lines {
        let unwrapped_line = line.unwrap();
        let unwrapped_line = unwrapped_line.trim();

        if unwrapped_line.is_empty() {
            continue;
        }

        let (header, body) = unwrapped_line.split_at(unwrapped_line.find(":").unwrap());

        if header.starts_with("Monkey") {
            current_monkey = header.chars().last().unwrap().to_digit(10).unwrap();
            if current_monkey != 0 {
                monkeys.push(monkey);
                monkey = Monkey::new();
            }
            continue;
        }

        match header {
            "Starting items" => {
                let mut starting_items = Vec::new();
                for item in body[1..].split(",") {
                    starting_items.push(item.trim().parse::<u32>().unwrap());
                }
                items.push(starting_items);
            }
            "Operation" => {
                let mut body = body.split_whitespace();
                monkey.operand = body.next_back().unwrap().to_string();
                monkey.operation = body.next_back().unwrap().to_string();
            }
            "Test" => {
                monkey.test_operand = body
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
            }
            "If true" => {
                monkey.test_positive_recipient = body
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
            }
            "If false" => {
                monkey.test_negative_recipient = body
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
            }
            _ => (),
        }
    }
    monkeys.push(monkey);

    for _ in 0..monkeys.len() {
        inspection_counts.push(0);
    }

    let number_of_monkeys = monkeys.len();
    for _ in 0..20 {
        for i in 0..number_of_monkeys {
            let inspection_results = monkeys[i].get_inspection_results(items[i].clone());
            inspection_counts[i] += inspection_results.len() as u32;
            items[i].clear();
            for result in inspection_results {
                items[result.1 as usize].push(result.0);
            }
        }
    }

    inspection_counts.sort();
    let highest = inspection_counts.pop().unwrap();
    let second_highest = inspection_counts.pop().unwrap();
    let monkey_business = highest * second_highest;
    println!("Level of monkey business: {}", monkey_business);
}
