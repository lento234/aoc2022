use std::time::Instant;

fn part_1(path: &str) -> i64 {
    let contents = utils::parse_file(path);

    let mut cpu = CPU {
        register: 1,
        cycle: 0,
        flags: (20..=220).step_by(40).collect(),
        signal: Vec::new(),
    };

    for line in contents.lines() {
        if line.starts_with("addx") {
            let value = line.split_once(' ').unwrap().1.parse::<i64>().unwrap();
            for _ in 0..2 {
                cpu.cycle();
            }
            cpu.addx(value);
        } else {
            cpu.cycle();
        }
    }
    cpu.get_signal()
}

fn part_2(path: &str) {
    let contents = utils::parse_file(path);

    let mut crt = CRT::new();

    for line in contents.lines() {
        if line.starts_with("addx") {
            let value = line.split_once(' ').unwrap().1.parse::<i64>().unwrap();
            for _ in 0..2 {
                crt.cycle();
            }
            crt.cpu.addx(value);
        } else {
            crt.cycle();
        }
    }

    crt.draw();
}

fn main() {
    println!();
    println!("🎄🎁 Advent of Code: Day 10 🎁🎄");
    println!("------------------------------\n");

    let start = Instant::now();

    // Challenge 1
    println!(
        "{}: {}",
        utils::color_text("[Part 1]", 'g'),
        part_1("input.txt")
    );
    // Challenge 2
    println!("{}:", utils::color_text("[Part 2]", 'g'));
    part_2("input.txt");

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
        assert!(part_1("test_input.txt") == 13140);
    }

    #[test]
    fn test_part1_challenge() {
        assert!(part_1("input.txt") == 13760);
    }
}

#[derive(Debug, Clone)]
struct CPU {
    register: i64,
    cycle: usize,
    flags: Vec<usize>,
    signal: Vec<(usize, i64)>,
}

impl CPU {
    fn addx(&mut self, value: i64) {
        self.register += value;
    }

    fn cycle(&mut self) {
        self.cycle += 1;
        if self.flags.contains(&self.cycle) {
            self.signal.push((self.cycle, self.register));
        }
    }

    fn get_signal(&self) -> i64 {
        self.signal
            .iter()
            .fold(0_i64, |acc, (cycle, value)| acc + (value * (*cycle as i64)))
    }
}

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

#[derive(Debug)]
struct CRT {
    cpu: CPU,
    cycle: usize,
    display: Vec<char>,
}

impl CRT {
    fn new() -> Self {
        let cpu = CPU {
            register: 1,
            cycle: 0,
            flags: Vec::new(),
            signal: Vec::new(),
        };
        Self {
            cpu: cpu,
            cycle: 0,
            display: vec![' '; WIDTH * HEIGHT],
        }
    }
    fn cycle(&mut self) {
        self.draw_display();
        self.cycle += 1;
        self.cpu.cycle();
    }

    fn draw_display(&mut self) {
        if ((self.cycle % WIDTH) as i64 - self.cpu.register).abs() < 2 {
            self.display[self.cycle] = '■';
        }
    }

    fn draw(&self) {
        for (i, pixel) in self.display.iter().enumerate() {
            if i % WIDTH == 0 {
                print!("\n{}", pixel);
            } else {
                print!("{}", pixel);
            }
        }
        println!();
    }
}
