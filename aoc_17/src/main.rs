
#![feature(convert)]

use std::slice::Iter;
use std::str::from_utf8;

fn subset_sum(containers: &[usize], sum: usize) -> usize {
    let mut f = vec![0; sum + 1];
    f[0] = 1;

    for current in containers {
        for fills_up in (0..(sum - current + 1)).rev() {
            f[fills_up + current] += f[fills_up];
        }
    }

    f[sum]
}


// github.com/Hoverbear/rust-rosetta/blob/master/src/power_set.rs
fn power_set<'a, T: Clone + 'a>(items: &mut Iter<'a,T>) -> Vec<Vec<T>> {
    let mut power = Vec::new();
    match items.next() {
        None       => power.push(Vec::new()),
        Some(item) => {
            for mut set in power_set(items).into_iter() {
                power.push(set.clone());
                set.push(item.clone());
                power.push(set);
            }
        }
    }

    power
}

fn minimal_subset_sum(containers: &[usize], sum: usize) -> usize {
    let mut power = power_set(&mut containers.iter());
    power.sort_by(|a, b| a.len().cmp(&b.len()));

    let mut minimum = 0;
    let mut solution = 0;

    for subset in power {
        if subset.len() > minimum {
            if solution > 0 {
                break;
            }

            solution = 0;
            minimum = subset.len();
        }

        let sub_solution = subset_sum(subset.as_slice(), sum);
        solution += sub_solution;
    }

    solution
}

fn main() {
    let input = from_utf8(include_bytes!("../input.txt"))
                .expect("Invalid input format.");

    let mut containers: Vec<usize> = Vec::new();

    for line in input.split('\n').filter(|l| !l.is_empty()) {
        let current = line.parse().expect("Invalid input format.");
        containers.push(current);
    }

    let solution = subset_sum(containers.as_slice(), 150);
    println!("There are {} combinations of containers that can fit \
             150 liters.", solution);

    println!("There are {} different ways to solve it using the minimal \
             amount of containers.",
             minimal_subset_sum(containers.as_slice(), 150));
}

