
use std::fs::File;
use std::io::Read;

use std::collections::HashMap;

#[derive(PartialOrd, PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    x: i32,
    y: i32
}

enum Step {
    North,
    South,
    East,
    West
}

impl Location {
    fn update(&mut self, step: Step) {
        match step {
            Step::North => self.y += 1,
            Step::South => self.y -= 1,
            Step::East  => self.x += 1,
            Step::West  => self.x -= 1
        }
    }
}

type Grid = HashMap<Location, u32>;

struct Puzzle {
    location_santa: Location,
    location_robot: Location,
    grid: Grid
}

impl Puzzle {
    fn new() -> Self {
        let mut puzzle = Puzzle {
            location_santa: Location { x: 0, y: 0 },
            location_robot: Location { x: 0, y: 0 },

            grid: Grid::new()
        };

        puzzle.grid.insert(puzzle.location_santa, 1);
        puzzle
    }

    fn visit_house(&mut self, location: &Location) {
        let count = self.grid.entry(*location).or_insert(0);
        *count += 1;
    }

    fn step_santa(&mut self, step: Step) {
        self.location_santa.update(step);

        let location = self.location_santa;
        self.visit_house(&location);
    }

    fn step_robot(&mut self, step: Step) {
        self.location_robot.update(step);

        let location = self.location_robot;
        self.visit_house(&location);
    }

    fn at_least_one_present(&self) -> usize {
        self.grid.values().filter(|&x| *x > 0).count()
    }
}

fn parse_step(encoded: char) -> Option<Step> {
    match encoded {
        '^' => Some(Step::North),
        'v' => Some(Step::South),
        '>' => Some(Step::East),
        '<' => Some(Step::West),
        _   => None
    }
}

fn main() {
    let mut file = File::open("../input.txt")
        .expect("Cannot open challenge file.");

    let mut challenge = String::new();
    file.read_to_string(&mut challenge).unwrap();

    let mut puzzle = Puzzle::new();
    for (index, character) in challenge.chars().enumerate() {
        let step = parse_step(character)
            .expect("Encountered an invalid step encoding.");

        if index & 1 == 0 {
            puzzle.step_santa(step);
        } else {
            puzzle.step_robot(step);
        }
    }

    let solution = puzzle.at_least_one_present();
    println!("{} houses received at least one present.", solution);
}

