
extern crate itertools;

use itertools::Itertools;

fn look_and_say(input: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for (key, group) in input.into_iter().group_by(|&x| x) {
        result.push(group.len() as i32);
        result.push(*key);
    }

    result
}

fn main() {
    let mut challenge = vec![3, 1, 1, 3, 3, 2, 2, 1, 1, 3];
    for i in 1..51 {
        challenge = look_and_say(&challenge);

        match i {
            40 | 50 => println!("Length at iteration {}: {}.", i,
                                challenge.len()),
            _ => ()
        }
    }
}

