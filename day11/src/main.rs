use std::time::Instant;

mod sim;
use sim::Simulation;

fn solve(path: &str, rounds: usize, reduce_worry: bool) -> usize {
    let contents = utils::parse_file(path);

    let monkey_props = contents.trim().split("\n\n").collect::<Vec<&str>>();

    let mut sim = Simulation::new(monkey_props, reduce_worry);

    // Run simulation
    for _ in 0..rounds {
        sim.step();
    }

    // Visualize final state
    // sim.display(rounds);

    // Collect all inspections
    let inspections = sim.inspections();

    // Multiply the first 2
    inspections[0..2].iter().product()
}

fn main() {
    println!();
    println!("🎄🎁 Advent of Code: Day 11 🎁🎄");
    println!("------------------------------\n");

    let start = Instant::now();

    // Challenge 1
    println!(
        "{}: {}",
        utils::color_text("[Part 1]", 'g'),
        solve("input.txt", 20, true)
    );

    // Challenge 2
    println!(
        "{}: {}",
        utils::color_text("[Part 2]", 'g'),
        solve("input.txt", 10000, false)
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
        assert!(solve("test_input.txt", 20, true) == 10605);
    }

    #[test]
    fn test_part1_challenge() {
        assert!(solve("input.txt", 20, true) == 57838);
    }

    #[test]
    fn test_part2() {
        assert!(solve("test_input.txt", 10000, false) == 2713310158);
    }

    #[test]
    fn test_part2_challenge() {
        assert!(solve("input.txt", 10000, false) == 15050382231);
    }
}
