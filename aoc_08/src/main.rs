
extern crate rustc_serialize;


use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::from_utf8;

use std::ops::Range;
use std::slice::Windows;
use std::iter::{Peekable, Zip};

use rustc_serialize::hex::FromHex;

type DecodeIterator<'a> = Peekable<Zip<Range<usize>, Windows<'a, u8>>>;

fn decode_hex_escape(result: &mut String, windows: &mut DecodeIterator)
    -> usize {
    windows.next();

    if let Some((i, byte)) = windows.next() {
        let byte_str = from_utf8(byte).unwrap();
        let byte_hex = byte_str.from_hex().unwrap();

        result.push(byte_hex[0] as char);
        i
    } else {
        unreachable!();
    }
}

fn decode(input: &String) -> String {
    let input = &input[1..input.len() - 1];
    if input.len() < 2 {
        return input.to_string();
    }

    /* We iterate over the input string in windows of two. Since windows only
     * yields full windows, we need to keep track of the position as not to
     * forget the last character. */
    let range       = 0..(input.len() - 1);
    let mut windows = range.zip(input.as_bytes().windows(2)).peekable();
    let mut result  = String::with_capacity(input.len());

    let mut last = 0;
    while let Some((i, w)) = windows.next() {
        last = i;

        match from_utf8(w).unwrap() {
            "\\\"" => result.push('"'),
            r"\\"  => result.push('\\'),
            r"\x"  => { last = decode_hex_escape(&mut result, &mut windows) },
            _ => {
                result.push(w[0] as char);

                // Consume the whole window, if it's the last.
                if windows.peek().is_none() {
                    result.push(w[1] as char);
                }

                continue;
            }
        }

        /* Advance to the next chunk (we processed the full two characters in
         * this window). */
        windows.next();
    }

    result.push_str(&input[last + 2..]);
    result
}

fn encode(input: &String) -> String {
    let mut result = String::with_capacity(input.len() * 1.5 as usize);
    result.push('"');

    for c in input.chars() {
        match c {
            '\\' => result.push_str(r"\\"),
            '"'  => result.push_str("\\\""),
            _    => result.push(c)
        }
    }

    result.push('"');
    result
}

fn main() {
    let f = File::open("../input.txt").expect("Cannot read challenge.");
    let file = BufReader::new(&f);

    let strings: Vec<_> = file.lines()
                              .map(|x| x.unwrap())
                              .collect();

    let solution_1 = strings.iter()
                            .fold(0, |acc, s| acc + s.chars().count()
                                                  - decode(s).chars().count());
    println!("Decoding saves {} characters.", solution_1);

    let solution_2 = strings.iter()
                            .fold(0, |acc, s| acc + encode(s).chars().count()
                                                  - s.chars().count());
    println!("Encoding requires {} more characters.", solution_2);
}

