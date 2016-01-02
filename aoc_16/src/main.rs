
use std::str::from_utf8;

struct Aunt {
    number: u16,

    children:    Option<u8>,
    cats:        Option<u8>,
    samoyeds:    Option<u8>,
    pomeranians: Option<u8>,
    akitas:      Option<u8>,
    vizslas:     Option<u8>,
    goldfish:    Option<u8>,
    trees:       Option<u8>,
    cars:        Option<u8>,
    perfumes:   Option<u8>
}

static PARSE_ERROR: &'static str = "Invalid input format.";

fn parse(input: &String) -> Vec<Aunt> {
    let mut aunts = Vec::new();

    for line in input.split('\n').filter(|l| !l.is_empty()) {
        let first = line.find(':').expect(PARSE_ERROR);
        let (aunt, properties) = line.split_at(first);

        let prefix = aunt.split(' ').collect::<Vec<_>>();
        if prefix.len() < 2 {
            panic!(PARSE_ERROR);
        }

        let number: u16 = prefix[1].parse().expect(PARSE_ERROR);
        let properties = properties[2..].split(',');

        let mut children = None;
        let mut cats = None;
        let mut samoyeds = None;
        let mut pomeranians = None;
        let mut akitas = None;
        let mut vizslas = None;
        let mut goldfish = None;
        let mut trees = None;
        let mut cars = None;
        let mut perfumes = None;

        for property in properties {
            let property = property.split(':').collect::<Vec<_>>();
            if property.len() < 2 {
                panic!(PARSE_ERROR);
            }

            let which = property[0].trim();
            let number = property[1].trim().parse().expect(PARSE_ERROR);

            match (which, number) {
                ("children", amount)    => children = Some(amount),
                ("cats", amount)        => cats = Some(amount),
                ("samoyeds", amount)    => samoyeds = Some(amount),
                ("pomeranians", amount) => pomeranians = Some(amount),
                ("akitas", amount)      => akitas = Some(amount),
                ("vizslas", amount)     => vizslas = Some(amount),
                ("goldfish", amount)    => goldfish = Some(amount),
                ("trees", amount)       => trees = Some(amount),
                ("cars", amount)        => cars = Some(amount),
                ("perfumes", amount)    => perfumes = Some(amount),
                _ => panic!(PARSE_ERROR)
            }
        }

        aunts.push(Aunt {
            number: number,
            children: children, cats: cats, samoyeds: samoyeds,
            pomeranians: pomeranians, akitas: akitas, vizslas: vizslas,
            goldfish: goldfish, trees: trees, cars: cars, perfumes: perfumes
        });
    }

    aunts
}

fn main() {
    let input = from_utf8(include_bytes!("../input.txt")).unwrap().to_string();
    let aunts = parse(&input);

    let predicate_one = |a: &&Aunt| -> bool {
        let properties = [a.children, a.cats, a.samoyeds, a.pomeranians,
            a.akitas, a.vizslas, a.goldfish, a.trees, a.cars, a.perfumes];
        let values = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];

        properties.iter().zip(values.iter()).all(|(p, v)| {
            if let Some(x) = *p { x == *v } else { true }
        })
    };

    let predicate_two = |a: &&Aunt| -> bool {
        let properties = [a.children, a.cats, a.samoyeds, a.pomeranians,
            a.akitas, a.vizslas, a.goldfish, a.trees, a.cars, a.perfumes];
        let values = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];

        let predicates: Vec<Box<Fn(u8, u8) -> bool>> = vec![
            Box::new(|x, y| x == y),
            Box::new(|x, y| x > y),
            Box::new(|x, y| x == y),
            Box::new(|x, y| x < y),
            Box::new(|x, y| x == y),
            Box::new(|x, y| x == y),
            Box::new(|x, y| x < y),
            Box::new(|x, y| x > y),
            Box::new(|x, y| x == y),
            Box::new(|x, y| x == y)
        ];

        predicates.iter().zip(properties.iter().zip(values.iter())).all(
            |(f, (x, y))| {
            if let Some(x) = *x { f(x, *y) } else { true }
        })
    };

    let candidates_0 = aunts.iter().filter(predicate_one).collect::<Vec<_>>();
    let candidates_1 = aunts.iter().filter(predicate_two).collect::<Vec<_>>();

    assert!(candidates_0.len() == 1, "No unique candidate has been found.");
    assert!(candidates_1.len() == 1, "No unique candidate has been found.");

    println!("Thank Aunt Sue {}.", candidates_0[0].number);
    println!("No wait, thank Aunt Sue {}.", candidates_1[0].number);
}

