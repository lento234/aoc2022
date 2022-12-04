use std::fs;

fn parse_file(path: &str) -> String {
    fs::read_to_string(path).expect("Cannot find file!")
}

fn part_1(path: &str) -> i64 {
    let contents = parse_file(path);

    let mut sum: i64 = 0;
    for line in contents.lines() {
        let bounds: Vec<i64> = line
            .split(',')
            .map(|x| x.split('-'))
            .into_iter()
            .flatten()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let a_min: i64 = bounds[0];
        let a_max: i64 = bounds[1];
        let b_min: i64 = bounds[2];
        let b_max: i64 = bounds[3];

        if (a_min <= b_min && a_max >= b_max) || (b_min <= a_min && b_max >= a_max) {
            sum += 1;
        }
    }
    sum
}

fn part_2(path: &str) -> i64 {
    let contents = parse_file(path);

    let mut sum: i64 = 0;
    for line in contents.lines() {
        let bounds: Vec<i64> = line
            .split(',')
            .map(|x| x.split('-'))
            .into_iter()
            .flatten()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let a_min: i64 = bounds[0];
        let a_max: i64 = bounds[1];
        let b_min: i64 = bounds[2];
        let b_max: i64 = bounds[3];

        if (a_max >= b_min) && (a_min <= b_max) {
            sum += 1;
        }
    }
    sum
}

fn main() {
    println!("");
    println!("🎄🎁 Advent of Code: Day 4 🎁🎄");
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
        assert!(part_1("test_input.txt") == 2);
    }

    #[test]
    fn test_part2() {
        assert!(part_2("test_input.txt") == 4);
    }
}
