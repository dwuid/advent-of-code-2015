
extern crate md5;

use md5::compute;

fn valid_proof_of_work(zeroes: usize, index: usize, byte: u8) -> bool {
    let exceeding_index = zeroes / 2;
    let mut valid = false;

    if zeroes & 1 == 0 {
        valid |= index > exceeding_index - 1;
    } else {
        valid |= index > exceeding_index;
        valid |= index == exceeding_index && (byte & 0xf0 == 0)
    };

    valid
}

fn solve(challenge: &str, zeroes: usize, tries: usize)
    -> Option<(usize, String)> {

    let challenge = challenge.to_owned().into_bytes();
    let mut pepper = 0;

    loop {
        let appendix = pepper.to_string().into_bytes();
        let mut input = Vec::new();

        input.extend(challenge.iter());
        input.extend(appendix.iter());

        let slice: &[u8] = &input;
        let digest = compute(slice);

        let mut occurrence = digest.iter()
            .enumerate()
            .skip_while(|&(_, byte)| *byte == 0);

        let success = match occurrence.next() {
            Some((index, byte)) => valid_proof_of_work(zeroes, index, *byte),
            None                => true
        };

        if success {
            let printable_digest = digest.iter()
                .map(|c| format!("{:02X}", c))
                .collect::<Vec<String>>()
                .join(" ");

            return Some((pepper, printable_digest))
        }

        pepper += 1;
        if pepper > tries {
            break;
        }
    }

    None
}

fn solve_proof_of_work(challenge: &str, zeroes: usize, tries: usize) {
    println!("{}", match solve(challenge, zeroes, tries) {
        Some((pepper, digest)) => {
            format!("Digest {} with seed {} starts with {} zeroes.",
                    digest, pepper, zeroes)
        },

        None => {
            format!("Within {} tries, no digest could be found starting with \
                    {} zeroes.", tries, zeroes)
        }
    });
}

fn main() {
    const CHALLENGE: &'static str = "ckczppom";
    const TRIES: usize = !0;

    solve_proof_of_work(CHALLENGE, 5, TRIES);
    solve_proof_of_work(CHALLENGE, 6, TRIES);
}

