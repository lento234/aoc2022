#[derive(Debug)]
pub struct Monkey {
    items: Vec<usize>,
    op: (String, String),
    divisor: usize,
    next_monkeys: Vec<usize>,
    n_inspections: usize,
}

impl Monkey {
    pub fn new(
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

    fn yeet(&mut self, reduce_worry: bool, lcd: usize) -> (usize, usize) {
        self.n_inspections += 1;
        let mut item = self.items.pop().expect("No item left!");
        item = match (self.op.0.as_str(), self.op.1.as_str()) {
            ("*", "old") => item * item,
            ("*", value) => item * value.parse::<usize>().unwrap(),
            ("+", "old") => item + item,
            ("+", value) => item + value.parse::<usize>().unwrap(),
            _ => panic!("Panic"),
        };
        if reduce_worry {
            item /= 3;
        } else {
            item %= lcd;
        }
        let to = (item % self.divisor != 0) as usize;
        (self.next_monkeys[to], item)
    }

    fn yoink(&mut self, value: usize) {
        self.items.push(value);
    }

    fn parse_monkey_props(prop: &str) -> Self {
        let prop = prop.lines().collect::<Vec<&str>>();

        // Parse starting item
        let items: Vec<usize> = prop[1]
            .split(':')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .split(',')
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
            .split(' ')
            .last()
            .expect("Unable to split divisible")
            .parse()
            .expect("Unable to convert to i64");

        // Parse if true condition
        let next_monkeys = prop[4..]
            .iter()
            .map(|line| {
                line.split(' ')
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
pub struct Simulation {
    monkeys: Vec<Monkey>,
    round: usize,
    reduce_worry: bool,
    lcd: usize,
}

impl Simulation {
    pub fn new(monkey_props: Vec<&str>, reduce_worry: bool) -> Self {
        // Collect all monkeys
        let monkeys = monkey_props
            .iter()
            .map(|prop| Monkey::parse_monkey_props(prop))
            .collect::<Vec<Monkey>>();
        // Calculate the least common divisor
        let lcd = monkeys.iter().map(|m| m.divisor).product();
        Self {
            monkeys,
            round: 0,
            reduce_worry,
            lcd,
        }
    }
    #[allow(dead_code)]
    pub fn display(&self, round: usize) {
        // println!("Round: {}", self.round);
        // for (i, monkey) in self.monkeys.iter().enumerate() {
        //     println!("Monkey {i}: {:?}", monkey.items);
        // }
        println!("== After round {} ==", round);
        for (i, monkey) in self.monkeys.iter().enumerate() {
            println!(
                "Monkey {i} inspected item {:?} times.",
                monkey.n_inspections
            );
        }
    }

    pub fn step(&mut self) {
        for i in 0..self.monkeys.len() {
            while !self.monkeys[i].items.is_empty() {
                let (to, value) = self.monkeys[i].yeet(self.reduce_worry, self.lcd);
                self.monkeys[to].yoink(value);
            }
        }
        self.round += 1;
    }

    pub fn inspections(&self) -> Vec<usize> {
        let mut inspections = self
            .monkeys
            .iter()
            .map(|monkey| monkey.n_inspections)
            .collect::<Vec<usize>>();
        inspections.sort_by(|a, b| b.cmp(a));
        inspections
    }
}
