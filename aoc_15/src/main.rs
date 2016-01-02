
use std::cmp::max;
use std::str::from_utf8;

struct Ingredient {
    _name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}

// TODO: Make this a proper iterator. Also reduce allocations?
fn sum_permutations(n: usize, k: usize) -> Vec<Vec<usize>> {
    match (n, k) {
        (0, _) => vec![],
        (_, 0) => vec![vec![0; n]],
        (1, _) => vec![vec![k; 1]],
        _ => {
            let mut solutions = Vec::new();
            for tail in sum_permutations(n - 1, k) {
                let mut current = vec![0; 1];
                current.extend(tail);

                solutions.push(current);
            }

            for tail in sum_permutations(n, k - 1) {
                let mut current = vec![tail[0] + 1; 1];
                current.extend(&tail[1..]);

                solutions.push(current);
            }

            solutions
        }
    }
}

fn solve(ingredients: &Vec<Ingredient>, teaspoons: usize) -> u32 {
    let mut perfect_score = 0;

    for distribution in sum_permutations(ingredients.len(), teaspoons) {
        let mut scores = [0i32; 4];
        let mut calories = 0;

        for (amount, ingredient) in distribution.iter()
                                                .zip(ingredients.iter()) {
            let amount = *amount as i32;

            scores[0] += ingredient.capacity * amount;
            scores[1] += ingredient.durability * amount;
            scores[2] += ingredient.flavor * amount;
            scores[3] += ingredient.texture * amount;

            calories += ingredient.calories * amount;
        }

        if calories != 500 {
            continue;
        }

        let total_score = if scores.iter().any(|i| *i < 0) {
            0
        } else {
            scores.iter().fold(1u32, |acc, i| acc * (*i as u32))
        };

        perfect_score = max(perfect_score, total_score);
    }

    perfect_score
}

static PARSE_ERROR: &'static str = "Invalid input format.";

fn main() {
    let input = from_utf8(include_bytes!("../input.txt")).unwrap();
    let mut ingredients = Vec::new();

    for line in input.split('\n').filter(|l| !l.is_empty()) {
        let tokens: Vec<_> = line.split(' ').collect();
        if tokens.len() < 11 {
            panic!(PARSE_ERROR);
        }

        // String::pop()? This is pretty ugly.
        let capacity = &tokens[2][..tokens[2].len() - 1];
        let durability = &tokens[4][..tokens[4].len() - 1];
        let flavor = &tokens[6][..tokens[6].len() - 1];
        let texture = &tokens[8][..tokens[8].len() - 1];
        let calories = tokens[10];

        let name = tokens[0][..tokens[0].len() - 1].to_string();
        let capacity: i32   = capacity.parse().expect(PARSE_ERROR);
        let durability: i32 = durability.parse().expect(PARSE_ERROR);
        let flavor: i32     = flavor.parse().expect(PARSE_ERROR);
        let texture: i32    = texture.parse().expect(PARSE_ERROR);
        let calories: i32   = calories.parse().expect(PARSE_ERROR);

        ingredients.push(Ingredient {
            _name: name, capacity: capacity, durability: durability,
            flavor: flavor, texture: texture, calories: calories
        });
    }

    println!("The tastiest 500 calorie cookie is worth {} points.",
             solve(&ingredients, 100));
}

