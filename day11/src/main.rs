use std::time::Instant;

fn solve(path: &str, rounds: usize, reduce_worry: bool) -> usize {
    let contents = utils::parse_file(path);

    let monkey_props = contents.trim().split("\n\n").collect::<Vec<&str>>();

    let mut sim = Simulation::new(monkey_props, reduce_worry);

    // Run simulation
    for _ in 0..rounds {
        sim.step();
    }
    // sim.display();

    // Visualize final state
    println!("== After round {} ==", rounds);
    for (i, monkey) in sim.monkeys.iter().enumerate() {
        println!(
            "Monkey {i} inspected item {:?} times.",
            monkey.n_inspections
        );
    }

    // Collect all inspections
    let mut inspections = sim
        .monkeys
        .iter()
        .map(|monkey| monkey.n_inspections)
        .collect::<Vec<usize>>();

    // Reverse sort
    inspections.sort_by(|a, b| b.cmp(a));

    // Multiply the first 2
    inspections[0..2].iter().fold(1, |acc, x| acc * x)
}

fn main() {
    println!();
    println!("🎄🎁 Advent of Code: Day 11 🎁🎄");
    println!("------------------------------\n");

    let start = Instant::now();

    // Challenge 1
    // println!(
    //     "{}: {}",
    //     utils::color_text("[Part 1]", 'g'),
    //     solve("input.txt", 20, true)
    // );
    // Challenge 2
    println!(
        "{}: {}",
        utils::color_text("[Part 2]", 'g'),
        solve("test_input.txt", 20, false)
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

    // #[test]
    // fn test_part2() {
    //     assert!(solve("test_input.txt", 20) == 2713310158);
    // }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: (String, String),
    divisor: usize,
    next_monkeys: Vec<usize>,
    n_inspections: usize,
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        op: (String, String),
        divisor: usize,
        next_monkeys: Vec<usize>,
    ) -> Self {
        Self {
            items,
            op,
            divisor,
            next_monkeys,
            n_inspections: 0,
        }
    }

    fn throw(&mut self, reduce_worry: bool) -> (usize, usize) {
        self.n_inspections += 1;
        let mut item = self.items.pop().expect("No item left!");
        item = match (self.op.0.as_str(), self.op.1.as_str()) {
            ("*", "old") => item * 2,
            ("*", value) => item * value.parse::<usize>().unwrap(),
            ("+", "old") => item + item,
            ("+", value) => item + value.parse::<usize>().unwrap(),
            _ => panic!("Panic"),
        };
        if reduce_worry {
            item /= 3;
        }
        let to = !(item % self.divisor == 0) as usize;
        (self.next_monkeys[to], item)
    }

    fn parse_monkey_props(prop: &str) -> Self {
        let prop = prop.lines().collect::<Vec<&str>>();

        // Parse starting item
        let items: Vec<usize> = prop[1]
            .split(":")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .split(",")
            .map(|s| s.trim().parse().expect("Unable to parse starting item"))
            .collect();

        // Parse operation
        let op = prop[2]
            .trim_start()
            .trim_end()
            .split_at(21)
            .1
            .split_once(' ')
            .expect("Unable to parse operation");

        // Parse operation
        let divisor: usize = prop[3]
            .split(" ")
            .last()
            .expect("Unable to split divisible")
            .parse()
            .expect("Unable to convert to i64");

        // Parse if true condition
        let next_monkeys = prop[4..]
            .iter()
            .map(|line| {
                line.split(" ")
                    .last()
                    .expect("Unable to get if true monkey")
                    .parse::<usize>()
                    .expect("Unable to parse next monkey")
            })
            .collect::<Vec<usize>>();

        Self::new(
            items,
            (op.0.to_string(), op.1.to_string()),
            divisor,
            next_monkeys,
        )
    }
}

#[derive(Debug)]
struct Simulation {
    monkeys: Vec<Monkey>,
    round: usize,
    reduce_worry: bool,
}

impl Simulation {
    fn new(monkey_props: Vec<&str>, reduce_worry: bool) -> Self {
        // Collect all monkeys
        Self {
            monkeys: monkey_props
                .iter()
                .map(|prop| Monkey::parse_monkey_props(prop))
                .collect::<Vec<Monkey>>(),
            round: 0,
            reduce_worry,
        }
    }

    fn display(&self) {
        println!("Round: {}", self.round);
        for (i, monkey) in self.monkeys.iter().enumerate() {
            println!("Monkey {i}: {:?}", monkey.items);
        }
    }
    fn step(&mut self) {
        for i in 0..self.monkeys.len() {
            while self.monkeys[i].items.len() > 0 {
                let (to, value) = self.monkeys[i].throw(self.reduce_worry);
                self.monkeys[to].items.push(value);
            }
        }
        self.round += 1;
    }
}
