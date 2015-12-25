
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn vowel_count(string: &String) -> usize {
    string.chars().filter(|&c| match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }).count()
}

fn contains_repeating_letter(string: &String) -> bool {
    let skip_one = string.chars().skip(1);
    let mut pairs = string.chars().zip(skip_one);

    pairs.any(|(a, b)| a == b)
}

fn contains_blacklisted(string: &String) -> bool {
    ["ab", "cd", "pq", "xy"].iter().any(|bad| string.contains(bad))
}

fn is_nice_one(string: &String) -> bool {
    vowel_count(&string) >= 3 &&
    contains_repeating_letter(&string) &&
    !contains_blacklisted(&string)
}

fn contains_repeating_pair(string: &String) -> bool {
    let skip_one = string.chars().skip(1);
    let pairs = string.chars().zip(skip_one);

    for (index, pair) in pairs.enumerate() {
        let mut needle = String::new();
        needle.push(pair.0);
        needle.push(pair.1);

        let tail = &string[index + 2..];
        if tail.contains(&needle) {
            return true;
        }
    }

    false
}

fn repeats_one_apart(string: &String) -> bool {
    let skip_two = string.chars().skip(2);
    let mut pairs = string.chars().zip(skip_two);

    pairs.any(|(a, b)| a == b)
}

fn is_nice_two(string: &String) -> bool {
    contains_repeating_pair(string) && repeats_one_apart(string)
}

fn main() {
    let f = File::open("../input.txt").expect("Cannot read challenge.");
    let file = BufReader::new(&f);

    let strings: Vec<_> = file.lines().map(|x| x.unwrap()).collect();

    let solution_one = strings.iter().filter(|s| is_nice_one(s)).count();
    println!("Strategy one yields {} nice strings.", solution_one);

    let solution_two = strings.iter().filter(|s| is_nice_two(s)).count();
    println!("Strategy two yields {} nice strings.", solution_two);
}

