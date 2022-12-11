use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Default)]
struct Rope {
    head: (i64, i64),
    tail: (i64, i64),
    tail_history: HashSet<(i64, i64)>,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: (0, 0),
            tail: (0, 0),
            tail_history: vec![(0, 0)].into_iter().collect(),
        }
    }

    fn chebyshev_distance(&self) -> i64 {
        let dx = (self.head.0 - self.tail.0).abs();
        let dy = (self.head.1 - self.tail.1).abs();
        dx.max(dy)
    }

    fn move_head(&mut self, dir: &str) {
        match dir {
            "L" => self.head.0 -= 1,
            "R" => self.head.0 += 1,
            "U" => self.head.1 += 1,
            "D" => self.head.1 -= 1,
            _ => panic!("Invalid direction"),
        };
        if self.chebyshev_distance() > 1 {
            let mut dx = self.head.0 - self.tail.0;
            if dx.abs() > 0 {
                dx /= dx.abs()
            };
            let mut dy = self.head.1 - self.tail.1;
            if dy.abs() > 0 {
                dy /= dy.abs()
            };
            self.tail.0 += dx;
            self.tail.1 += dy;
            self.tail_history.insert((self.tail.0, self.tail.1));
        };
    }
}

fn part_1(path: &str) -> usize {
    let contents = utils::parse_file(path);

    let mut rope = Rope::new();
    for line in contents.lines() {
        println!("{:?}", line);
        let (dir, n_moves) = line.split_once(" ").expect("Failed to split line!");
        for _ in 0..n_moves.parse::<_>().unwrap() {
            rope.move_head(dir);
        }
    }
    rope.tail_history.len()
}

fn main() {
    println!();
    println!("🎄🎁 Advent of Code: Day 9 🎁🎄");
    println!("------------------------------\n");

    let start = Instant::now();

    // Challenge 1
    println!(
        "{}: {}",
        utils::color_text("[Part 1]", 'g'),
        part_1("input.txt")
    );
    // Challenge 2
    // println!("{}: {}", utils::color_text("[Part 2]", 'g'), part_2("input.txt"));

    println!(
        "{}: {} µs",
        utils::color_text("[Summary]", 'b'),
        start.elapsed().as_micros()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part_1("test_input.txt") == 13);
    }
}
