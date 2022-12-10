use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

fn part_1(path: &str) -> usize {
    // Collect file contents
    let content = utils::parse_file(path);

    // Create root node
    let root = Rc::new(Dir::new());
    // Create a pointer the current working directory
    let mut cwd = Rc::clone(&root);

    for line in content.lines().skip(1) {
        let words = line.split(' ').collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => {
                match words[2] {
                    ".." => cwd = Rc::clone(cwd.parent.as_ref().unwrap()),
                    dirname => {
                        let newdir = cwd.subdir.borrow().get(dirname).unwrap().clone();
                        cwd = newdir;
                    }
                };
            }
            ("dir", dirname) => cwd.add_subdir(dirname, &cwd),
            (size, _) => *cwd.size.borrow_mut() += size.parse::<usize>().unwrap(),
        }
    }

    // Travel to all nodes in the directory starting from the root
    let mut total_size: usize = 0;
    let mut queue = Vec::new();
    queue.push(Rc::clone(&root));

    while let Some(node) = queue.pop() {
        for subdir in node.subdir.borrow().values() {
            queue.push(Rc::clone(subdir));
        }

        let size = node.get_size();
        if size < 100000 {
            total_size += size;
        }
    }
    total_size
}

fn main() {
    println!();
    println!("🎄🎁 Advent of Code: Day 7 🎁🎄");
    println!("------------------------------\n");

    let start = Instant::now();

    // // Challenge 1
    println!(
        "{}: {}",
        utils::color_text("[Part 1]", 'g'),
        part_1("input.txt")
    );
    // // Challenge 2
    // println!("{}: {}", utils::color_text("[Part 2]", 'g'), part_2("input.txt"));

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
        assert!(part_1("test_input.txt") == 95437);
    }
}

#[derive(Default)]
struct Dir {
    parent: Option<Rc<Dir>>,
    size: RefCell<usize>,
    subdir: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    fn new() -> Self {
        Self {
            parent: None,
            size: RefCell::new(0),
            subdir: RefCell::new(HashMap::new()),
        }
    }
    // Add subdir into cwd
    fn add_subdir(&self, name: &str, parent: &Rc<Dir>) {
        let newdir = Rc::new(Dir {
            size: RefCell::new(0),
            parent: Some(Rc::clone(parent)),
            subdir: RefCell::new(HashMap::new()),
        });
        self.subdir.borrow_mut().insert(String::from(name), newdir);
    }

    fn get_size(&self) -> usize {
        let subdir_size = self
            .subdir
            .borrow()
            .values()
            .fold(0, |a, b| a + b.get_size());
        *self.size.borrow() + subdir_size
    }
}
