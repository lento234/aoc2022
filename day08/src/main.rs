use std::time::Instant;

fn part_1(path: &str) -> usize {
    let contents = utils::parse_file(path);
    // Parse grid
    let grid: Vec<Vec<u8>> = contents
        .lines()
        .map(|line| line.chars().map(|x| x as u8 - 48).collect())
        .collect();

    // Get size
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut n_visible = nrows * 2 + ncols * 2 - 4;

    for i in 1..nrows - 1 {
        for j in 1..ncols - 1 {
            let value = &grid[i][j];
            let l_max = grid[i][0..j].iter().max().unwrap();
            let r_max = grid[i][j + 1..ncols].iter().max().unwrap();
            let t_max = grid[0..i].iter().map(|x| &x[j]).max().unwrap();
            let b_max = grid[i + 1..nrows].iter().map(|x| &x[j]).max().unwrap();

            if value > l_max || value > r_max || value > t_max || value > b_max {
                n_visible += 1;
            }
        }
    }
    n_visible
}

fn part_2(path: &str) -> usize {
    let contents = utils::parse_file(path);
    // Parse grid
    let grid: Vec<Vec<u8>> = contents
        .lines()
        .map(|line| line.chars().map(|x| x as u8 - 48).collect())
        .collect();

    // Get size
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut score: usize = 0;

    for i in 1..nrows - 1 {
        for j in 1..ncols - 1 {
            let value = &grid[i][j];

            let l_max = grid[i][0..j].iter().max().unwrap();
            let r_max = grid[i][j + 1..ncols].iter().max().unwrap();
            let t_max = grid[0..i].iter().map(|x| &x[j]).max().unwrap();
            let b_max = grid[i + 1..nrows].iter().map(|x| &x[j]).max().unwrap();

            if value > l_max || value > r_max || value > t_max || value > b_max {
                let left = if value > l_max {
                    j
                } else {
                    grid[i][0..j]
                        .iter()
                        .rev()
                        .position(|x| x >= value)
                        .expect("Cannot find left")
                        + 1
                };
                let right = if value > r_max {
                    ncols - 1 - j
                } else {
                    grid[i][j + 1..ncols]
                        .iter()
                        .position(|x| x >= value)
                        .expect("cannot find right")
                        + 1
                };
                let top = if value > t_max {
                    i
                } else {
                    grid[0..i]
                        .iter()
                        .rev()
                        .position(|x| &x[j] >= value)
                        .expect("cannot find top")
                        + 1
                };
                let bottom = if value > b_max {
                    nrows - 1 - i
                } else {
                    grid[i + 1..nrows]
                        .iter()
                        .position(|x| &x[j] >= value)
                        .expect("cannot find bottom")
                        + 1
                };
                score = score.max(left * right * bottom * top);
            }
        }
    }
    score
}

fn main() {
    println!();
    println!("🎄🎁 Advent of Code: Day 8 🎁🎄");
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
        "{}: {}",
        utils::color_text("[Summary]", 'b'),
        start.elapsed().as_micros()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part_1("test_input.txt") == 21);
    }

    #[test]
    fn test_part2() {
        assert!(part_2("test_input.txt") == 8);
    }
}
