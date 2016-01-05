
#![feature(convert)]
extern crate revord;

use revord::RevOrd;
use std::str::from_utf8;
use std::collections::{HashSet, BinaryHeap};

fn calibrate(challenge: &str, productions: &Vec<(&str, &str)>) -> usize {
    let mut words = HashSet::new();

    for &(symbol, replacement) in productions {
        for (index, _) in challenge.match_indices(symbol) {
            let (head, tail) = challenge.split_at(index);
            let result = format!("{}{}{}", head, replacement,
                                 &tail[symbol.len()..]);
            words.insert(result);
        }
    }

    words.len()
}

// There is pretty much going on for part two. I should clean this mess up.
fn reduce(challenge: &String, productions: &Vec<(&str, &str)>, steps: usize)
    -> (String, usize) {

    let mut queue = BinaryHeap::new();
    let mut seen  = HashSet::new();

    let mut latest = (challenge.clone(), steps);

    queue.push((RevOrd(challenge.clone()), steps));
    while let Some((RevOrd(string), steps)) = queue.pop() {
        if seen.contains(&string) {
            continue;
        }

        seen.insert(string.clone());
        latest = (string.clone(), steps);

        for &(symbol, replacement) in productions {
            for (index, _) in string.rmatch_indices(replacement) {
                let (head, tail) = string.split_at(index);
                let result = format!("{}{}{}", head, symbol,
                                     &tail[replacement.len()..]);

                queue.push((RevOrd(result), steps + 1));
            }
        }
    }

    latest
}

fn helper(challenge: &String, productions: &Vec<(&str, &str)>, steps: usize)
    -> (String, usize) {

    let mut sub_challenges: Vec<String> = challenge.split("Ar")
                                                   .map(|x| x.to_string())
                                                   .collect();

    for i in 0..(sub_challenges.len() - 1) {
        sub_challenges[i].push_str("Ar");
    }

    if sub_challenges.last().unwrap().is_empty() {
        sub_challenges.pop();
    }

    if sub_challenges.len() == 1 {
        return reduce(challenge, productions, steps);
    }

    let mut total_steps = steps;
    let mut result = String::with_capacity(challenge.len());

    for sub in sub_challenges {
        let (string, steps) = helper(&sub, productions, 0);
        total_steps += steps;

        result.push_str(string.as_str());
    }

    (result, total_steps)
}

fn fabricate(medicine: &str, productions: &Vec<(&str, &str)>)
    -> Option<usize> {

    let mut productions = productions.clone();
    productions.sort_by(|&(_, a), &(_, b)| b.len().cmp(&a.len()));

    let mut challenge = medicine.to_string();
    let mut steps = 0;

    while challenge != "e" {
        let (new_challenge, total) = helper(&challenge, &productions, steps);
        challenge = new_challenge;
        steps = total;
    }

    Some(steps)
}

static PARSE_ERROR: &'static str = "Invalid input format.";

fn main() {
    let input = from_utf8(include_bytes!("../input.txt")).unwrap();
    let mut lines: Vec<_> = input.split('\n')
                                 .filter(|l| !l.is_empty())
                                 .collect();

    let mut productions = Vec::new();
    let challenge = lines.pop().expect(PARSE_ERROR);

    for line in &lines {
        let tokens: Vec<_> = line.split(' ').collect();
        if tokens.len() < 3 || tokens[0].len() > tokens[2].len() {
            // We do not allow productions that reduce the input length.
            panic!(PARSE_ERROR);
        }

        productions.push((tokens[0], tokens[2]));
    }

    let solution = calibrate(challenge, &productions);
    println!("{} molecules can be created.", solution);

    match fabricate(challenge, &productions) {
        Some(steps) => println!("The medicine can be created in {} steps.",
                                steps),
        _ => println!("We seem to be unable to produce the medicine.")
    }
}

