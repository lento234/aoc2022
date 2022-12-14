use std::collections::HashSet;
use std::collections::VecDeque;
use std::time::Instant;

struct Grid {
    grid: Vec<char>,
    ncols: usize,
    nrows: usize,
    start: usize,
    end: usize,
}
impl Grid {
    fn new(contents: String) -> Self {
        let ncols = contents.lines().next().unwrap().len();
        let nrows = contents.lines().count();
        let mut grid = contents.lines().fold(Vec::new(), |mut acc, line| {
            acc.extend(line.chars());
            acc
        });
        let start = grid
            .iter()
            .position(|&c| c == 'S')
            .expect("Unable to find starting point");
        let end = grid
            .iter()
            .position(|&c| c == 'E')
            .expect("Unable to find ending point");
        grid[start] = 'a';
        grid[end] = 'z';
        Self {
            // fold all lines to a single vector of chars
            grid,
            ncols,
            nrows,
            start,
            end,
        }
    }

    #[allow(dead_code)]
    fn draw(&self) {
        println!("Grid: (nrows={}, ncols={}):\n", self.nrows, self.ncols,);
        let mut c;
        for i in 0..self.nrows {
            for j in 0..self.ncols {
                let p = i * self.ncols + j;
                c = self.grid[p].to_string();
                if p == self.start {
                    print!("{}", utils::color_text(c.as_str(), 'b'));
                } else if p == self.end {
                    print!("{}", utils::color_text(c.as_str(), 'g'));
                } else {
                    print!("{}", c.as_str());
                }
            }
            println!();
        }
        println!();
    }

    fn travel(&self) -> usize {
        // Performs Dijkstra's search from start to end
        let (_cost_map, nsteps) = self.dijkstra();

        // for line in cost_map.chunks(self.ncols) {
        //     println!("{:?}", line);
        // }
        // println!("It took {:?} steps.", nsteps);

        nsteps
    }

    fn dijkstra(&self) -> (Vec<usize>, usize) {
        // Initalize cost map
        let mut cost_map = vec![usize::MAX; self.grid.len()];
        cost_map[self.start] = 0;

        // Initalize queue
        let mut queue = VecDeque::new();
        queue.push_back((self.start, 0));

        // Initalize visited set
        let mut visited = HashSet::new();

        // self.nsteps
        while !queue.is_empty() {
            let (p, cost) = queue.pop_front().expect("Unable to pop from queue");

            if visited.contains(&p) {
                continue;
            } else {
                visited.insert(p);
            }

            if p == self.end {
                break;
            }

            // Get neighbors
            let neighbors = self.neighbors(p);

            // Iterate over neighbors
            for p_nbr in neighbors.iter() {
                if ((self.grid[*p_nbr] as i64) - (self.grid[p] as i64)) <= 1 {
                    cost_map[*p_nbr] = cost + 1;
                    queue.push_back((*p_nbr, cost_map[*p_nbr]));
                }
            }
        }
        // println!("Destination => {:?}", cost_map[self.end]);
        let nsteps = cost_map[self.end];
        (cost_map, nsteps)
    }

    fn neighbors(&self, k: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();

        let i = k / self.ncols;
        let j = k % self.ncols;

        if i > 0 {
            neighbors.push((i - 1) * self.ncols + j);
        }
        if i < self.nrows - 1 {
            neighbors.push((i + 1) * self.ncols + j);
        }
        if j > 0 {
            neighbors.push(i * self.ncols + j - 1);
        }
        if j < self.ncols - 1 {
            neighbors.push(i * self.ncols + j + 1);
        }
        neighbors
    }
}

fn part_1(path: &str) -> usize {
    let contents = utils::parse_file(path);

    let grid = Grid::new(contents);

    // grid.draw();
    grid.travel()
}

fn main() {
    println!();
    println!("🎄🎁 Advent of Code: Day 12 🎁🎄");
    println!("------------------------------\n");

    let start = Instant::now();

    // // Challenge 1
    println!(
        "{}: {}",
        utils::color_text("[Part 1]", 'g'),
        part_1("test_input.txt")
    );
    // // Challenge 2
    // println!("{}: {}", utils::color_text("[Part 2]", 'g'), part_2("test_input.txt"));

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
        assert!(part_1("test_input.txt") == 31);
    }

    // #[test]
    // fn test_part2() {
    //     assert!(part_2("test_input.txt") == 0);
    // }
}
