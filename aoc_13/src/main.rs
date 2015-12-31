
use std::cmp::max;
use std::str::from_utf8;
use std::collections::{HashSet, HashMap};

type Costs = i16;

fn salesman<'a>(start: &'a str, dists: &HashMap<(&'a str, &'a str), Costs>,
                current: &'a str, remaining: &HashSet<&'a str>)
    -> (Costs, &'a str) {

    assert!(!remaining.contains(current));
    assert!(!remaining.contains(start));

    if remaining.is_empty() {
        return (dists[&(start, current)], start);
    }

    let mut sub_solutions = Vec::new();
    for person in remaining.iter() {
        let mut next = remaining.clone();

        assert!(next.remove(person));
        assert!(next.len() == remaining.len() - 1);

        let sub_problem = salesman(start, dists, person, &next);
        let mut distance = dists[&(*person, current)];

            distance += sub_problem.0;

        sub_solutions.push((distance, *person));
    }

    let solution = *sub_solutions.iter().max_by_key(|&&(d, _)| d).unwrap();
    solution
}

fn travelling_salesman<'a>(start: &'a str,
                           dists: &HashMap<(&'a str, &'a str), Costs>,
                           people: &HashSet<&'a str>)
    -> Option<Vec<(Costs, &'a str)>> {

    let mut remaining = people.clone();
    assert!(remaining.remove(start));

    let mut current = start;
    let mut path = Vec::new();

    loop {
        let (distance, person) = salesman(start, &dists, current, &remaining);

        path.push((distance, current));
        if person == start {
            return Some(path);
        }

        current = &person;
        remaining.remove(person);
    }
}

fn golden_sphere<'a>(dists: &HashMap<(&'a str, &'a str), Costs>,
                     people: &HashSet<&'a str>) -> i16 {
    let mut happiness = 0;
    for person in people {
        if let Some(path) = travelling_salesman(person, &dists, &people) {
            happiness = max(path[0].0, happiness);
        }
    }

    happiness
}

fn main() {
    let input = from_utf8(include_bytes!("../input.txt")).unwrap();
    let mut people = HashSet::new();
    let mut costs  = HashMap::new();

    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }

        let line = &line[..line.len() - 1];
        let tokens: Vec<_> = line.split(" ").collect();

        if tokens.len() < 11 {
            panic!("Invalid input format.");
        }

        people.insert(tokens[0]);
        people.insert(tokens[10]);

        let mut cost: Costs = tokens[3].parse()
                                       .expect("Invalid input format.");
        if tokens[2] == "lose" {
            cost = -cost;
        }

        costs.insert((tokens[0], tokens[10]), cost);
    }


    let me_mysqelf_and_i = "Redrick";
    people.insert(&me_mysqelf_and_i);

    for p in &people {
        costs.insert((&me_mysqelf_and_i, p), 0);
        costs.insert((p, &me_mysqelf_and_i), 0);
    }

    let mut i = people.iter();
    while let Some(u) = i.next() {
        for v in i.clone() {
            let a = costs[&(*u, *v)];
            let b = costs[&(*v, *u)];

            costs.insert((u, v), a + b);
            costs.insert((v, u), a + b);
        }
    }

    let mut dists = HashMap::new();
    for u in &people {
        for v in &people {
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

    println!("Maximizing happiness to level {}.", golden_sphere(&dists,
                                                                &people));
}

