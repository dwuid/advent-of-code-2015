
use std::str::from_utf8;

use std::fmt;
use std::fmt::{Display, Formatter};

static PARSE_ERROR: &'static str = "Invalid input format.";

#[derive(Debug)]
enum Phase {
    Fly(u16),
    Rest(u16)
}

struct Reindeer {
    name: String,
    distance: u32,
    points: u32,
    phase: Phase,

    speed: u16,
    duration: u16,
    cooldown: u16,
}

impl Display for Reindeer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = format!("I am {} and I flew {} km! Currently, I am {}. \
                             I scored {} point(s) so far!",
                             self.name, self.distance,
        match self.phase {
            Phase::Fly(s)  => format!("flying (for {}s)",  s),
            Phase::Rest(s) => format!("resting (for {}s)", s),
        }, self.points);

        write!(f, "{}", string)
    }
}

impl Reindeer {
    fn new(name: &String, speed: u16, duration: u16, cooldown: u16) -> Self {
        Reindeer {
            name: name.clone(), speed: speed, duration: duration,
            cooldown: cooldown, distance: 0, points: 0, phase: Phase::Fly(0)
        }
    }

    fn update(&mut self) {
        use Phase::*;

        self.phase = match self.phase {
            Fly(s)  => {
                self.distance += self.speed as u32;
                if s + 1 >= self.duration { Rest(0) } else { Fly(s + 1) }
            },

            Rest(s) => {
                if s +1 >= self.cooldown { Fly(0) } else { Rest(s + 1) }
            }
        };
    }
}

fn race(reindeers: &mut Vec<Reindeer>, duration: u16) {
    for _ in 0..duration {
        for reindeer in &mut *reindeers {
            reindeer.update();
        }

        let maximum = reindeers.iter()
                               .max_by_key(|r| r.distance)
                               .unwrap()
                               .distance;

        for winner in reindeers.iter_mut().filter(|r| r.distance == maximum) {
            winner.points += 1;
        }
    }
}

fn main() {
    let input = from_utf8(include_bytes!("../input.txt")).unwrap();
    let race_duration = 2503;

    let mut reindeers = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<_> = line.split(' ').collect();
        if tokens.len() < 15 {
            panic!(PARSE_ERROR);
        }

        let name = tokens[0];
        let speed: u16    = tokens[3].parse().expect(PARSE_ERROR);
        let duration: u16 = tokens[6].parse().expect(PARSE_ERROR);
        let cooldown: u16 = tokens[13].parse().expect(PARSE_ERROR);

        let r = Reindeer::new(&name.to_string(), speed, duration, cooldown);
        reindeers.push(r);
    }

    race(&mut reindeers, race_duration);
    println!("Winner by distance: {}", reindeers.iter()
                                                .max_by_key(|r| r.distance)
                                                .unwrap());
    println!("Winner by points: {}", reindeers.iter()
                                              .max_by_key(|r| r.points)
                                              .unwrap());
}

#[test]
fn part_one() {
    let mut reindeers = Vec::new();
    let a = Reindeer::new(&"Comet".to_string(),  14, 10, 127);
    let b = Reindeer::new(&"Dancer".to_string(), 16, 11, 162);

    reindeers.push(a);
    reindeers.push(b);

    race(&mut reindeers, 10);

    assert!(reindeers[0].distance == 140);
    assert!(reindeers[1].distance == 160);

    race(&mut reindeers, 11 - 10);

    assert!(reindeers[0].distance == 140);
    assert!(reindeers[1].distance == 176);

    race(&mut reindeers, 1000 - 11 - 10);
    assert!(reindeers[0].distance == 1120);
    assert!(reindeers[1].distance == 1056);

    let a = &reindeers[0];
    let b = &reindeers[1];

    if let Phase::Fly(_) = a.phase {
        panic!("{} should be resting.", a.name);
    }

    if let Phase::Fly(_) = b.phase {
        panic!("{} should be resting.", b.name);
    }
}

#[test]
fn part_two() {
    let mut reindeers = Vec::new();
    let a = Reindeer::new(&"Comet".to_string(),  14, 10, 127);
    let b = Reindeer::new(&"Dancer".to_string(), 16, 11, 162);

    reindeers.push(a);
    reindeers.push(b);

    race(&mut reindeers, 1);
    assert!(reindeers[0].points == 0);
    assert!(reindeers[1].points == 1);

    race(&mut reindeers, 140 - 1);
    assert!(reindeers[0].points == 1);
    assert!(reindeers[1].points == 139);

    race(&mut reindeers, 1000 - 140);
    assert!(reindeers[0].points == 312);
    assert!(reindeers[1].points == 689);
}

