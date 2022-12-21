use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = BufReader::new(File::open("day6.txt").unwrap()).lines();
    let line = lines.next().unwrap().unwrap();
    let bytes = line.as_bytes();

    let mut chars = Vec::new();
    let mut start_of_packet_marker_position = 0;
    let mut packet_marker_found = false;
    let packet_marker_limit = 4;
    let message_marker_limit = 14;

    for b in bytes {
        let c: char = *b as char;
        chars.push(c);
        start_of_packet_marker_position += 1;

        if chars.len() >= packet_marker_limit && !packet_marker_found {
            let mut set = std::collections::HashSet::new();

            for i in 0..packet_marker_limit {
                set.insert(chars[chars.len() - 1 - i]);
            }
            
            if set.len() == packet_marker_limit {
                println!("Start of packet marker found at index {}", start_of_packet_marker_position);
                packet_marker_found = true;
            }
        }

        if chars.len() >= message_marker_limit {
            let mut set = std::collections::HashSet::new();
            for i in 0..message_marker_limit {
                set.insert(chars[chars.len() - 1 - i]);
            }
            
            if set.len() == message_marker_limit {
                println!("Start of message marker found at index {}", start_of_packet_marker_position);
                break;
            }
        }
    }
}
