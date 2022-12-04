use std::collections::HashSet;
use std::fs;

fn parse_file(path: &str) -> String {
    fs::read_to_string(path).expect("Cannot find file!")
}

fn char_to_priority(c: char) -> u8 {
    let c_u8 = c as u8;
    if c_u8 > 96 {
        c_u8 - 97 + 1
    } else if c_u8 > 64 {
        c_u8 - 65 + 27
    } else {
        0
    }
}

fn part_1(path: &str) -> i64 {
    let contents = parse_file(path);

    let mut sum: i64 = 0;
    for line in contents.lines() {
        // Collect bags
        let mid = line.len() / 2;
        let bag_a: HashSet<char> = HashSet::from_iter(line.get(0..mid).unwrap().chars());
        let bag_b: HashSet<char> = HashSet::from_iter(line.get(mid..).unwrap().chars());

        // Find the priority char
        let priority_char = *bag_a.intersection(&bag_b).last().unwrap();

        // Calculate the value
        let priority = char_to_priority(priority_char);
        sum += priority as i64;
    }
    sum
}

fn part_2(path: &str) -> i64 {
    let contents = parse_file(path);
    let mut sum: i64 = 0;
    let lines: Vec<&str> = contents.lines().collect();
    let size = lines.len();

    for i in (0..size).step_by(3) {
        // Collect bags
        let mut bag: HashSet<char> = HashSet::from_iter(lines[i].chars());
        bag = bag
            .intersection(&HashSet::from_iter(lines[i + 1].chars()))
            .cloned()
            .collect();
        bag = bag
            .intersection(&HashSet::from_iter(lines[i + 2].chars()))
            .cloned()
            .collect();

        let priority_char = *bag.iter().last().unwrap();
        let priority = char_to_priority(priority_char);

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
    // Challenge 2
    println!("\u{1b}[32m[Part 2]\u{1b}[39m: {}", part_2("input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part_1("test_input.txt") == 157);
    }

    #[test]
    fn test_part2() {
        assert!(part_2("test_input.txt") == 70);
    }
}
