use std::collections::HashSet;
use std::time::Instant;

use utils;

fn solve(path: &str, packet_size: usize) -> usize {
    let contents = utils::parse_file(path);

    let line = contents.lines().next().unwrap();
    let chars: Vec<char> = line.chars().collect();

    for (i, batch) in chars.windows(packet_size).enumerate() {
        if batch.into_iter().collect::<HashSet<_>>().len() == packet_size {
            return i + packet_size;
        }
    }
    0
}

fn main() {
    println!();
    println!("🎄🎁 Advent of Code: Day 6 🎁🎄");
    println!("------------------------------\n");

    let start = Instant::now();

    // Challenge 1
    println!(
        "{}: {:?}",
        utils::color_text("[Part 1]", 'g'),
        solve("input.txt", 4)
    );
    // Challenge 2
    println!(
        "{}: {:?}",
        utils::color_text("[Part 1]", 'g'),
        solve("input.txt", 14)
    );
    println!(
        "{}: {} μs",
        utils::color_text("[Summary]", 'b'),
        start.elapsed().as_micros()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(solve("test_input.txt", 4) == 7);
        assert!(solve("test_input2.txt", 4) == 5);
        assert!(solve("test_input3.txt", 4) == 10);
    }

    #[test]
    fn test_part2() {
        assert!(solve("test_input.txt", 14) == 19);
    }
}
