
extern crate serde_json;

use serde_json::de;
use serde_json::Value;

fn helper(accumulator: i64, value: &Value, do_filter: bool) -> i64 {
    let mut accumulator = accumulator;

    if let Some(values) = value.as_array() {
        for value in values {
            accumulator = helper(accumulator, value, do_filter);
        }

    } else if value.is_object() {
        let tree = value.as_object().unwrap();
        let mut intermediate = accumulator;

        for value in tree.values() {
            if do_filter && value.is_string() &&
                value.as_string().unwrap() == "red" {
                return accumulator;
            }

            intermediate = helper(intermediate, value, do_filter);
        }

        accumulator = intermediate;

    } else if value.is_number() {
        assert!(!value.is_f64(), "Cannot handle floating point values yet.");
        accumulator += value.as_i64().unwrap();
    }

    accumulator
}

fn visitor(value: &Value, do_filter: bool) -> i64 {
    helper(0, value, do_filter)
}

fn main() {
    let input = include_bytes!("../input.txt");
    let deserialized: Value = match de::from_slice(input) {
        Ok(values) => values,
        _ => panic!("Cannot parse challenge.")
    };

    println!("Sum of all encountered total billing amounts: {}.",
             visitor(&deserialized, false));

    println!("Sum of all valid encountered total billing amounts: {}.",
             visitor(&deserialized, true));
}

