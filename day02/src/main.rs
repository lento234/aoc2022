use std::fs;

#[derive(Debug,PartialEq)]
enum RPS {
    rock = 1,
    paper,
    scissor,
    err
}

fn parse_line(line: &str) -> (RPS, RPS) {
    let left = match line.get(0..1).unwrap() {
        "A" => RPS::rock,
        "B" => RPS::paper,
        "C" => RPS::scissor,
        _ => RPS::err
    };

    let right = match line.get(2..).unwrap() {
        "X" => RPS::rock,
        "Y" => RPS::paper,
        "Z" => RPS::scissor,
        _ => RPS::err
    };
    (left, right)
}

fn part_1(path: &str) -> i64 {
    let contents = fs::read_to_string(path).expect("Cannot find file!");

    let mut total_score: i64 = 0;

    for line in contents.lines() {
        let score = match parse_line(line) {
            (RPS::rock, RPS::paper) => (RPS::paper as u8) + 6,
            (RPS::paper, RPS::scissor) => (RPS::scissor as u8) + 6,
            (RPS::scissor, RPS::rock) => (RPS::rock as u8) + 6,
            (left, right) => {
                if left == right {
                    (left as u8) + (right as u8)
                } else {
                    right as u8
                }
            }
        };

        total_score += score as i64;

    }
    total_score
}

fn main() {
        // Test: Part 1
        let answer: i64 = part_1("test_input.txt");
        println!("\u{1b}[31m[Test]\u{1b}[39m: Part 1: {}", answer);
        assert!(answer == 15);
}
