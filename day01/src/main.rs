use std::fs;
// use std::collections;

fn part_1(path: &str) -> i64 {
    let contents: String = fs::read_to_string(path).expect("Cannot find file!");

    let mut sum: i64 = 0;
    let mut largest_sum: i64 = 0;

    for line in contents.lines() {
        if line == "" {
            if sum > largest_sum {
                largest_sum = sum;
            }
            sum = 0;
        } else {
            sum += line.parse::<i64>().unwrap();
        }
    }

    largest_sum
}

fn part_2(path: &str) -> i64 {
    // Read the file
    let contents: String = fs::read_to_string(path).expect("Cannot find file!");

    let input: Vec<&str> = contents.split('\n').collect(); // Assume space on last line

    // Initialize sum
    let mut sums: Vec<i64> = Vec::new();
    let mut sum: i64 = 0;

    for line in input {
        if line == "" {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i64>().unwrap();
        }
    }
    // Store sum
    sums.sort();

    // Return the sum of the biggest 3
    sums.iter().rev().take(3).sum::<i64>()
}

fn main() {
    // Test: Part 1
    let answer: i64 = part_1("test_input.txt");
    println!("[Test]: Part 1: {}", answer);
    assert!(answer == 24_000);

    // Test: Part 2
    let answer: i64 = part_2("test_input.txt");
    println!("[Test]: Part 2: {:?}", answer);
    assert!(answer == 45_000);

    // Challenge 1
    println!("[Solution]: Part 1: {}", part_1("input.txt"));
    // Challenge 2
    println!("[Solution]: Part 2: {}", part_2("input.txt"));
}
