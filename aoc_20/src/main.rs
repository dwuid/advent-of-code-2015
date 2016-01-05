
#![feature(step_by)]

fn solve_one(challenge: usize) -> Option<usize> {
    let mut houses = vec![10; challenge / 10];
    let mut boundary = houses.len() - 1;

    for e in 2..boundary {
        for h in (e..boundary).step_by(e) {
            if h >= boundary {
                break;
            }

            houses[h] += e * 10;
            if houses[h] >= challenge {
                boundary = h;
            }
        }
    }

    houses.iter().enumerate().filter(|&(_, &h)| h >= challenge)
          .next().map(|x| x.0)
}

fn solve_two(challenge: usize) -> Option<usize> {
    let mut houses = vec![10; challenge / 10];
    let mut boundary = houses.len() - 1;

    for e in 2..boundary {
        for (i, h) in (e..boundary).step_by(e).enumerate() {
            if i >= 50 || h >= boundary {
                break;
            }

            houses[h] += e * 11;
            if houses[h] >= challenge {
                boundary = h;
            }
        }
    }

    houses.iter().enumerate().filter(|&(_, &h)| h >= challenge)
          .next().map(|x| x.0)
}

fn format(solution: Option<usize>) {
    match solution {
        Some(house) => println!("House #{} is lucky.", house),
        None        => panic!("No solution found.")
    }
}

fn main() {
    const CHALLENGE: usize = 34000000;
    format(solve_one(CHALLENGE));
    format(solve_two(CHALLENGE));
}

