
use std::cmp::{min, max};
use std::str::from_utf8;
use std::collections::{HashSet, HashMap};

type Costs = u16;

fn salesman<'a>(start: &'a str, dists: &HashMap<(&'a str, &'a str), Costs>,
                current: &'a str, remaining: &HashSet<&'a str>, minimize: bool)
    -> (Costs, &'a str) {

    assert!(!remaining.contains(current));
    assert!(!remaining.contains(start));

    if remaining.is_empty() {
        // return (dists[&(start, current)], start); -- Traditional TSP.
        return (0, start);
    }

    let mut sub_solutions = Vec::new();
    for town in remaining.iter() {
        let mut next = remaining.clone();

        assert!(next.remove(town));
        assert!(next.len() == remaining.len() - 1);

        let sub_problem = salesman(start, dists, town, &next, minimize);
        let mut distance = dists[&(*town, current)];

        if distance != Costs::max_value() &&
            sub_problem.0 != Costs::max_value() {
            distance += sub_problem.0;
        }

        sub_solutions.push((distance, *town));
    }

    // We probably can do better than this.
    let solution = if minimize {
        *sub_solutions.iter().min_by_key(|&&(d, _)| d).unwrap()
    } else {
        *sub_solutions.iter().max_by_key(|&&(d, _)| d).unwrap()
    };

    solution
}

fn travelling_salesman<'a>(start: &'a str,
                           dists: &HashMap<(&'a str, &'a str), Costs>,
                           towns: &HashSet<&'a str>, minimize: bool)
    -> Option<Vec<(Costs, &'a str)>> {

    let mut remaining = towns.clone();
    assert!(remaining.remove(start));

    let mut current = start;
    let mut path = Vec::new();

    loop {
        let (distance, town) = salesman(start, &dists, current, &remaining,
                                        minimize);
        if distance == Costs::max_value() {
            return None;
        }

        path.push((distance, current));
        if town == start {
            return Some(path);
        }

        current = &town;
        remaining.remove(town);
    }
}

fn shortest_route<'a>(dists: &HashMap<(&'a str, &'a str), Costs>,
                      towns: &HashSet<&'a str>) -> u16 {
    let mut shortest = u16::max_value();
    for town in towns {
        if let Some(path) = travelling_salesman(town, &dists, &towns, true) {
            shortest = min(path[0].0, shortest);
        }
    }

    shortest
}

fn longest_route<'a>(dists: &HashMap<(&'a str, &'a str), Costs>,
                     towns: &HashSet<&'a str>) -> u16 {
    let mut longest = 0;
    for town in towns {
        if let Some(path) = travelling_salesman(town, &dists, &towns, false) {
            longest = max(path[0].0, longest);
        }
    }

    longest
}

fn main() {
    let input = from_utf8(include_bytes!("../input.txt")).unwrap();
    let mut towns = HashSet::new();
    let mut costs = HashMap::new();

    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<_> = line.split(" ").collect();
        if tokens.len() < 5 {
            panic!("Invalid input format.");
        }

        towns.insert(tokens[0]);
        towns.insert(tokens[2]);

        let cost = tokens[4].parse::<Costs>().expect("Invalid input format.");

        costs.insert((tokens[0], tokens[2]), cost);
        costs.insert((tokens[2], tokens[0]), cost);
    }

    let mut dists = HashMap::new();
    for u in &towns {
        for v in &towns {
            let key = (*u, *v);
            dists.insert(key, if u == v {
                0
            } else {
                Costs::max_value()
            });
        }
    }

    for element in &costs {
        dists.insert(*element.0, *element.1);
    }

    println!("The shortest route has distance {}.",
             shortest_route(&dists, &towns));
    println!("The longes route has distance {}",
             longest_route(&dists, &towns));
}

