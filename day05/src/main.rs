use std::fs;
use std::time::Instant;

fn parse_file(path: &str) -> String {
    fs::read_to_string(path).expect("Cannot find file!")
}

fn part_1(path: &str) -> String {
    let content = parse_file(path);
    let lines: Vec<&str> = content.lines().collect();

    // Find location where number of stacks is given
    let p = lines.iter().enumerate().find(|(_, x)| x.is_empty()).unwrap().0 - 1;

    // Generate stakcs
    let n_stacks = lines[p].trim().chars().last().unwrap().to_digit(10).unwrap();

    // Allocate stacks
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..n_stacks {
        stacks.push(Vec::new());
    }

    // Insert boxes into stacks
    for i in (0..p).rev() {
        for (i, c) in lines[i].chars().enumerate() {
            if c != '[' && c != ']' && c != ' ' {
                stacks[i / 4].push(c);
            }
        }
    }

    // Loop over instructions
    for i in (p+2)..lines.len() {
        let values: Vec<&str> = lines[i].split_whitespace().collect();
        let n_moves: usize = values[1].parse().unwrap();
        let from: usize = values[3].parse().unwrap();
        let to: usize = values[5].parse().unwrap();

        for _ in 0..n_moves {
            let b = stacks[from-1].pop().expect("stack is empty");
            stacks[to-1].push(b);
        }
    }

    // Answer
    let mut answer: String = String::new();
    for stack in stacks.iter() {
        let c = stack.last().expect("stack is empty");
        answer.push(*c);
    }

    answer
}


fn part_2(path: &str) -> String {
    let content = parse_file(path);
    let lines: Vec<&str> = content.lines().collect();

    // Find location where number of stacks is given
    let p = lines.iter().enumerate().find(|(_, x)| x.is_empty()).unwrap().0 - 1;

    // Generate stakcs
    let n_stacks = lines[p].trim().chars().last().unwrap().to_digit(10).unwrap();

    // Allocate stacks
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..n_stacks {
        stacks.push(Vec::new());
    }

    // Insert boxes into stacks
    for i in (0..p).rev() {
        for (i, c) in lines[i].chars().enumerate() {
            if c != '[' && c != ']' && c != ' ' {
                stacks[i / 4].push(c);
            }
        }
    }

    // Loop over instructions
    for i in (p+2)..lines.len() {
        let values: Vec<&str> = lines[i].split_whitespace().collect();
        let n_moves: usize = values[1].parse().unwrap();
        let from: usize = values[3].parse().unwrap();
        let to: usize = values[5].parse().unwrap();

        let mut boxes: Vec<char> = Vec::new();
        for _ in 0..n_moves {
            let b = stacks[from-1].pop().expect("stack is empty");
            boxes.push(b);
        }

        for _ in 0..n_moves {
            let b = boxes.pop().expect("stack is empty");
            stacks[to-1].push(b);
        }
    }

    // Answer
    let mut answer: String = String::new();
    for stack in stacks.iter() {
        let c = stack.last().expect("stack is empty");
        answer.push(*c);
    }
    answer
}

fn main() {
    println!("");
    println!("🎄🎁 Advent of Code: Day 5 🎁🎄");
    println!("------------------------------\n");

    let start = Instant::now();

    // Challenge 1
    println!("\u{1b}[32m[Part 1]\u{1b}[39m: {}", part_1("input.txt"));
    // Challenge 2
    println!("\u{1b}[32m[Part 2]\u{1b}[39m: {}", part_2("input.txt"));

    println!(
        "\n\u{1b}[34m[Summary]\u{1b}[39m: It took {} μs.",
        start.elapsed().as_micros()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part_1("test_input.txt") == "CMZ");
    }

    #[test]
    fn test_part2() {
        assert!(part_2("test_input.txt") == "MCD");
    }
}
