use std::fmt;
use std::cmp::min;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Present {
    length: u32,
    width:  u32,
    height: u32
}

impl fmt::Display for Present {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}x{}", self.length, self.width, self.height)
    }
}

impl Present {
    fn surface_area(&self) -> u32 {
        2 * self.length * self.width + 2 * self.width * self.height +
            2 * self.height * self.length
    }

    fn smallest_side_area(&self) -> u32 {
        min(self.length * self.width, min(self.width * self.height,
            self.height * self.length))
    }

    pub fn required_paper(&self) -> u32 {
        self.surface_area() + self.smallest_side_area()
    }

    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    pub fn required_ribbon(&self) -> u32 {
        let shortest_distance = min(2 * self.length + 2 * self.width,
                                    min(2 * self.height + 2 * self.width,
                                        2 * self.height + 2 * self.length));
        let bow_ribbon = self.volume();
        shortest_distance + bow_ribbon
    }
}

fn parse_present(description: &str) -> Option<Present> {
    let current: Vec<_> = description.split('x').collect();
    if current.len() != 3 {
        return None;
    }

    let vec: Vec<u32> = current.iter()
        .map(|&x| x.parse().expect("Invalid present description."))
        .collect();

    let present = Present {
        length: vec[0],
        width:  vec[1],
        height: vec[2]
    };

    Some(present)
}

fn main() {
    let f = File::open("../input.txt").expect("Cannot open challenge file.");
    let file = BufReader::new(&f);

    let presents: Vec<Present> = file.lines()
        .map(|x| parse_present(&x.unwrap()).expect("Unable to parse present."))
        .collect();

    let total_paper = presents.iter()
        .fold(0, |acc, p| acc + p.required_paper());

    let total_ribbon = presents.iter()
        .fold(0, |acc, p| acc + p.required_ribbon());

    println!("Required wrapping paper for all presents: {}.", total_paper);
    println!("Required ribbon for all presents: {}.", total_ribbon);
}

