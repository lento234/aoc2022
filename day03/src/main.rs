use std::collections::HashSet;
use std::fs;

fn parse_file(path: &str) -> String {
    fs::read_to_string(path).expect("Cannot find file!")
}

fn part_1(path: &str) -> i64 {
    let contents = parse_file(path);

    let mut sum: i64 = 0;
    for line in contents.lines() {
        // Collect bags
        let mid = line.len() / 2;
        let bag_a: HashSet<&u8> = HashSet::from_iter(line.get(0..mid).unwrap().as_bytes());
        let bag_b: HashSet<&u8> = HashSet::from_iter(line.get(mid..).unwrap().as_bytes());

        // Find the priority char
        let priority_char = **bag_a.intersection(&bag_b).last().unwrap();

        // Calculate the value
        let priority = {
            if priority_char < 96 {
                priority_char - 65 + 27
            } else {
                priority_char - 97 + 1
            }
        };
        sum += priority as i64;
    }
    sum
}

fn main() {
    println!("");
    println!("🎄🎁 Advent of Code: Day 3 🎁🎄");
    println!("------------------------------\n");

    // Challenge 1
    println!("\u{1b}[32m[Part 1]\u{1b}[39m: {}", part_1("input.txt"));
    // // Challenge 2
    // println!("\u{1b}[32m[Part 2]\u{1b}[39m: {}", part_2("input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part_1("test_input.txt") == 157);
    }
}
