use std::fs;

#[derive(Debug, PartialEq)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
    Unknown = 0,
}

fn parse_line(line: &str) -> (RPS, RPS) {
    let left = match line.get(0..1).unwrap() {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissor,
        _ => RPS::Unknown,
    };

    let right = match line.get(2..).unwrap() {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissor,
        _ => RPS::Unknown,
    };
    (left, right)
}

fn part_1(path: &str) -> i64 {
    let contents = fs::read_to_string(path).expect("Cannot find file!");

    let mut total_left_score: i64 = 0;
    let mut total_right_score: i64 = 0;

    for line in contents.lines() {
        let (left_score, right_score) = match parse_line(line) {
            (RPS::Rock, RPS::Paper) => (RPS::Rock as i64, (RPS::Paper as i64) + 6),
            (RPS::Paper, RPS::Scissor) => (RPS::Paper as i64, (RPS::Scissor as i64) + 6),
            (RPS::Scissor, RPS::Rock) => (RPS::Scissor as i64, (RPS::Rock as i64) + 6),
            (left, right) => {
                if left == right {
                    ((left as i64) + 3, (right as i64) + 3)
                } else {
                    (left as i64, right as i64)
                }
            }
        };
        total_left_score += left_score;
        total_right_score += right_score;
    }

    if total_left_score > total_right_score {
        total_left_score
    } else {
        total_right_score
    }
}

#[test]
fn test_part1() {
    assert!(part_1("test_input.txt") == 15);
}

// #[test]
// fn test_part2() {
//     assert!(part_1("test_input.txt") == 15);
// }

fn main() {
    println!("Advent of Code: Day 2");
    println!("---------------------");

    // Challenge 1
    println!(
        "\u{1b}[32m[Solution]\u{1b}[39m: Part 1: {}",
        part_1("input.txt")
    );
}
