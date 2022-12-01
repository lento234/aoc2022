
use std::fs;

fn part_1(path: &str) -> i64 {
    let contents = fs::read_to_string(path)
        .expect("Cannot find file");

    let mut sum: i64 = 0;
    let mut largest_sum: i64 = 0;

    for line in contents.lines() {
        if line == "" {
            if sum > largest_sum {
                largest_sum = sum;
            }
            sum = 0;
        }
        else {
            sum += line.parse::<i64>().unwrap();
        }
    }

    largest_sum
}


fn main() {
    // Tests

    // Part 1
    let largest_sum: i64 = part_1("test_input.txt");
    println!("[Test]: Part 1: {}", largest_sum);
    assert!(largest_sum == 24_000);

    // Input
    let largest_sum: i64 = part_1("input.txt");
    println!("[Solution]: Part 1: {}", largest_sum);

}
