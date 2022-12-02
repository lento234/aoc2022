use std::fs;
// use std::collections;

fn part_1(path: &str) -> i64 {
    let contents = fs::read_to_string(path).expect("Cannot find file!");

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
    let contents = fs::read_to_string(path).expect("Cannot find file!");

    // Initialize sum
    let mut sums: Vec<i64> = Vec::new();
    let mut sum: i64 = 0;
    let mut largest_sum: i64 = 0;

    for line in contents.lines() {
        if line == "" {
            if sum > largest_sum || sums.len() <= 3 {
                sums.push(sum.clone());
                largest_sum = sum.clone();
            }
            sum = 0;
        } else {
            sum += line.parse::<i64>().unwrap();
        }
    }
    sums.sort();

    sums.iter().rev().enumerate().filter(|x| x.0 <= 3 ).map(|x| x.1).sum::<i64>()

}

fn main() {
    // Tests

    // Part 1
    let answer: i64 = part_1("test_input.txt");
    println!("[Test]: Part 1: {}", answer);
    assert!(answer == 24_000);

    // Part 1
    let answer: i64 = part_2("test_input.txt");
    println!("[Test]: Part 2: {:?}", answer);
    assert!(answer == 45_000);


    // // Input
    // let largest_sum: i64 = part_1("input.txt");
    // println!("[Solution]: Part 1: {}", largest_sum);
}
