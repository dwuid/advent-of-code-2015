
#[macro_use]
extern crate nom;
extern crate topological_sort;

mod wires;
use wires::*;

fn inspect_wire(state: &State, wire_name: &str) -> Option<Concrete> {
    let wire = Operand::Variable(wire_name.to_string());
    let signal;

    match state.get(&wire) {
        Some(&s) => signal = s,
        _ => panic!("Circuit does not provide a signal to wire {}.",
                    wire_name)
    };

    println!("Signal {} is provided to wire {}.", signal, wire_name);
    Some(signal)
}

fn solve(circuit: &Vec<Statement>) -> Option<Concrete> {
    let state;
    match evaluate_circuit(&circuit) {
        Some(s) => state = s,
        None => panic!("Cannot evaluate circuit.")
    }

    if let signal@Some(_) = inspect_wire(&state, "a") {
        return signal
    }

    panic!("Cannot inspect wire.");
}

fn main() {
    let input = include_bytes!("../input.txt");
    let mut circuit;

    match parse_circuit(input) {
        Some(result) => circuit = result,
        _ => panic!("Cannot parse circuit.")
    }

    let signal_a = solve(&circuit).unwrap();
    let wire_b = Operand::Variable("b".to_string());

    let connect_a_b = Statement {
        input: Expression::Unary(UnaryExpression {
            operation: UnaryOperation::Id,
            operand:   Operand::Constant(signal_a)
        }),
        output: wire_b.clone()
    };

    circuit.push(connect_a_b);
    solve(&circuit);
}

