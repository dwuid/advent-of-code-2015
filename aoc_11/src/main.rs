
fn contains_increasing_triple(string: &String) -> bool {
    string.as_bytes().windows(3).any(|w| {
        w[0] + 1 == w[1] && w[1] + 1 == w[2]
    })
}

fn contains_nonoverlapping_pairs(string: &String) -> bool {
    let a = string.as_bytes().iter();
    let b = string.as_bytes()[1..].iter();

    let mut i = a.zip(b);
    let mut c = 0;

    // More idiomatic way in O(1)?
    while c < 2 {
        if let Some((x, y)) = i.next() {
            if x == y {
                c += 1;
                i.next();
                i.next();
            }
        } else {
            break;
        }
    }

    c >= 2
}

fn fulfills_criteria(string: &String) -> bool {
    let mut good = !string.chars().any(|c| match c {
        'i' | 'o' | 'l' => true, _ => false
    });

    good = good && contains_increasing_triple(&string);
    good = good && contains_nonoverlapping_pairs(&string);

    good
}

fn increase(string: &String, index: usize) -> String {
    let mut result = String::new();
    assert!(index < string.len());

    for (i, c) in string.chars().rev().enumerate() {
        match c {
            'z' => result.push('a'),
            _   => {
                result.push(((c as u8) + 1) as char);
                result.extend(string.chars().rev()
                                    .skip(i + 1)
                                    .take(string.len() - i)
                                    .collect::<Vec<_>>());
                break;
            }
        }
    }

    result.chars().rev().collect::<String>()
}

fn main() {
    let mut challenge = "hxbxwxba".to_string();

    for _ in 0..2 {
        while !fulfills_criteria(&challenge) {
            challenge = increase(&challenge, challenge.len() - 1);
        }

        println!("Your next password is: {}.", challenge);
        challenge = increase(&challenge, challenge.len() - 1);
    }
}

