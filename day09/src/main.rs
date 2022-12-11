use std::collections::HashSet;
use std::time::Instant;

fn parse_direction(dir: &str) -> (i64, i64) {
    match dir {
        "L" => (-1, 0),
        "R" => (1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("Invalid direction"),
    }
}
#[derive(Debug, Clone)]
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

    fn update_knot(&mut self, head_pos: (i64, i64)) -> (i64, i64) {
        self.head.0 = head_pos.0;
        self.head.1 = head_pos.1;
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
        }
        self.tail
    }
}

fn part_1(path: &str) -> usize {
    let contents = utils::parse_file(path);

    let mut rope = Rope::new();
    for line in contents.lines() {
        let (dir, n_moves) = line.split_once(' ').expect("Failed to split line!");
        let dir = parse_direction(dir);
        for _ in 0..n_moves.parse::<_>().unwrap() {
            let head_pos = (rope.head.0 + dir.0, rope.head.1 + dir.1);
            rope.update_knot(head_pos);
        }
    }
    rope.tail_history.len()
}

fn part_2(path: &str) -> usize {
    let contents = utils::parse_file(path);

    // Initialize 10 rope vector
    let mut ropes = vec![Rope::new(); 9];

    for line in contents.lines() {
        // Parse direction and number of moves
        let (dir_str, n_moves) = line.split_once(' ').expect("Failed to split line!");
        // Perform the moves
        for _ in 0..n_moves.parse::<_>().unwrap() {
            // Convert the direction to a tuple
            let dir = parse_direction(dir_str);
            // Get new position of the head rope
            let mut head_pos = (
                ropes.first().unwrap().head.0 + dir.0,
                ropes.first().unwrap().head.1 + dir.1,
            );
            // Update the knot position for each rope
            for rope in ropes.iter_mut() {
                head_pos = rope.update_knot(head_pos);
            }
        }
    }
    ropes.last().unwrap().tail_history.len()
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
    println!(
        "{}: {}",
        utils::color_text("[Part 2]", 'g'),
        part_2("input.txt")
    );

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

    #[test]
    fn test_part1_challenge() {
        assert!(part_1("input.txt") == 6357);
    }

    #[test]
    fn test_part2_1() {
        assert!(part_2("test_input.txt") == 1);
    }

    #[test]
    fn test_part2_2() {
        assert!(part_2("test_input2.txt") == 36);
    }

    #[test]
    fn test_part2_challenge() {
        assert!(part_2("input.txt") == 2627);
    }
}
