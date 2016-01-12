
#![feature(convert)]

use std::cmp::min;
use std::collections::HashSet;

fn subset_sum(sum: u32, target: u32, weights: &[u32], subset: Vec<u32>,
              minimal_length: usize)
    -> Vec<Vec<u32>> {

    let mut results        = Vec::new();
    let mut minimal_length = minimal_length;

    if subset.len() > minimal_length {
        return results;
    }

    if sum == target {
        results.push(subset.clone());
    } else {
        for (index, &w) in weights.iter().enumerate() {
            if sum + w > target {
                break;
            }

            let mut next = subset.clone();
            next.push(w);

            let other = subset_sum(sum + w, target,
                                   &weights[(index + 1)..], next,
                                   minimal_length);

            minimal_length = other.iter().fold(minimal_length,
                                               |acc, x| min(acc, x.len()));
            results.extend(other);
        }
    }

    results.into_iter().filter(|x| x.len() <= minimal_length).collect()
}

fn difference(a: &[u32], b: &[u32]) -> Vec<u32> {
    let a = a.iter().cloned().collect::<HashSet<_>>();
    let b = b.iter().cloned().collect::<HashSet<_>>();

    a.difference(&b).cloned().collect()
}

fn quantum_entaglement(v: &Vec<u32>) -> usize {
    v.iter().fold(1usize, |acc, x| acc * (*x as usize))
}

fn verify_solution(target: u32, groups: usize, weights: &[u32], subset: &[u32])
    -> bool {
    if groups == 1 {
        return subset.iter().fold(0, |acc, x| acc + x) == target;
    }

    let remaining  = difference(weights, subset);
    let subsets = subset_sum(0, target, remaining.as_slice(),
                             Vec::new(), remaining.len() / groups);

    for subset in subsets {
        if verify_solution(target, groups - 1, remaining.as_slice(),
                           subset.as_slice()) {
            return true;
        }
    }

    false
}

fn solve(weights: &[u32], target: u32) -> Option<usize> {
    let shortest_subsets = subset_sum(0, target, weights, Vec::new(),
                                      weights.len() / GROUPS);

    let quantum = shortest_subsets.iter()
                                  .map(|s| quantum_entaglement(s))
                                  .collect::<Vec<_>>();

    let mut combined: Vec<_> = shortest_subsets.into_iter().zip(
        quantum.into_iter()).collect();

    combined.sort_by(|&(_, q0), &(_, q1)| q0.cmp(&q1));
    for (subset, quantum) in combined {
        let remaining = weights.clone();

        if verify_solution(target, GROUPS - 1, remaining, subset.as_slice()) {
            return Some(quantum);
        }
    }

    None
}

static GROUPS: usize = 3;

fn main() {
    let lines = include_str!("../input.txt").split('\n');
    let weights: Result<Vec<u32>, _> = lines.filter(|l| !l.is_empty())
                                            .map(|l| l.parse())
                                            .collect();

    let mut weights = weights.expect("Invalid input format.");
    let target = weights.iter().fold(0, |acc, x| acc + x) / (GROUPS as u32);

    weights.sort();
    match solve(weights.as_slice(), target) {
        Some(solution) => {
            println!("Santa is happy with a quantum entanglement of {:?} for \
                     the first of {} groups.", solution, GROUPS);
        },

        _ => println!("The given input has no solution.")
    }
}

