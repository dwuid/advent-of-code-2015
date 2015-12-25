
use std::fmt;
use std::mem;
use std::str::FromStr;
use std::num::ParseIntError;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// FIXME: This is way too much code. Consider using nom for parsing.

enum ParsingError {
    ParseFailed,
    TooFewArguments
}

impl fmt::Debug for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            ParsingError::ParseFailed => "Unable to parse input.",
            ParsingError::TooFewArguments => "Not enough input for parsing."
        })
    }
}

impl From<ParseIntError> for ParsingError {
    fn from(_: ParseIntError) -> ParsingError {
        ParsingError::ParseFailed
    }
}

struct Coordinate {
    x: usize,
    y: usize
}

impl FromStr for Coordinate {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<_> = input.split(',').collect();
        if numbers.len() < 2 {
            return Err(ParsingError::TooFewArguments);
        }

        let x: usize = try!(numbers[0].parse());
        let y: usize = try!(numbers[1].parse());
        Ok(Coordinate { x: x, y: y })
    }
}

struct Rectangle {
    x: Coordinate,
    y: Coordinate
}

impl FromStr for Rectangle {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = input.split(' ').collect();
        if tokens.len() < 3 || tokens[1] != "through" {
            return Err(ParsingError::TooFewArguments);
        }

        let x: Coordinate = try!(Coordinate::from_str(tokens[0]));
        let y: Coordinate = try!(Coordinate::from_str(tokens[2]));
        Ok(Rectangle { x: x, y: y })
    }
}

impl Rectangle {
    fn points(&self) -> Vec<Coordinate> {
        let mut result: Vec<_> = Vec::new();

        let (a, b) = (&self.x, &self.y);
        let (mut x, mut x_max) = (a.x, b.x);
        let (mut y, mut y_max) = (a.y, b.y);

        if x > x_max {
            mem::swap(&mut x, &mut x_max);
        }

        if y > y_max {
            mem::swap(&mut y, &mut y_max);
        }

        for cur_x in x..(x_max +1) {
            for cur_y in y..(y_max + 1) {
                result.push(Coordinate { x: cur_x, y: cur_y })
            }
        }

        result
    }
}

enum Action {
    On(Rectangle),
    Off(Rectangle),
    Toggle(Rectangle)
}

impl FromStr for Action {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        const TOGGLE: &'static str = "toggle ";
        const ON: &'static str     = "turn on ";
        const OFF: &'static str    = "turn off ";

        if input.starts_with(TOGGLE) {
            let r = try!(Rectangle::from_str(&input[TOGGLE.len()..]));
            Ok(Action::Toggle(r))

        } else if input.starts_with(ON) {
            let r = try!(Rectangle::from_str(&input[ON.len()..]));
            Ok(Action::On(r))

        } else if input.starts_with(OFF) {
            let r = try!(Rectangle::from_str(&input[OFF.len()..]));
            Ok(Action::Off(r))

        } else {
            Err(ParsingError::ParseFailed)
        }
    }
}

const N: usize = 1000;

impl Action {
    fn apply(&self, array: &mut [[u32; N]; N]) {
        use Action::*;

        match *self {
            On(ref r) => {
                for c in r.points() {
                    array[c.x][c.y] += 1;
                }
            },

            Off(ref r) => {
                for c in r.points() {
                    if array[c.x][c.y] > 0 {
                        array[c.x][c.y] -= 1;
                    }
                }
            },

            Toggle(ref r) => {
                for c in r.points() {
                    array[c.x][c.y] += 2;
                }
            }
        }
    }
}

struct Puzzle {
    actions: Vec<Action>,
    grid: [[u32; N]; N]
}

impl Puzzle {
    fn new(lines: &Vec<String>) -> Result<Self, ParsingError> {
        let actions: Vec<_> = lines.into_iter()
            .map(|l| Action::from_str(&l).unwrap())
            .collect();

        let grid = [[0; N]; N];
        Ok(Puzzle { actions: actions, grid: grid })
    }

    fn solve(&mut self) -> usize {
        for action in &self.actions {
            action.apply(&mut self.grid);
        }

        let mut lit = 0;
        for row in self.grid.iter() {
            for field in row.iter() {
                lit += *field;
            }
        }

        lit as usize
    }
}

fn main() {
    let f = File::open("../input.txt").expect("Cannot read challenge.");
    let file = BufReader::new(&f);
    let lines: Vec<_> = file.lines().map(|x| x.unwrap()).collect();

    let puzzle = Puzzle::new(&lines);
    match puzzle {
        Ok(mut puzzle) => println!("{}", puzzle.solve()),
        _ => println!("Failed to create puzzle instance."),
    }
}

