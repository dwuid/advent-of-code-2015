
use super::types::*;

use topological_sort::TopologicalSort;
use std::collections::HashMap;

macro_rules! get(
    ($e:expr) => (match $e { Some(e) => e, None => return None })
);

pub type State = HashMap<Operand, Concrete>;

trait Evaluatable {
    fn evaluate(&self, state: &State) -> Option<Concrete>;
}

impl Evaluatable for Operand {
    fn evaluate(&self, state: &State) -> Option<Concrete> {
        use super::Operand::*;

        match *self {
            Constant(concrete) => Some(concrete),
            Variable(_) => state.get(self).and_then(|&x| Some(x))
        }
    }
}

impl Evaluatable for UnaryExpression {
    fn evaluate(&self, state: &State) -> Option<Concrete> {
        use super::UnaryOperation::*;

        match self.operation {
            Id => self.operand.evaluate(state),
            Not => self.operand.evaluate(state).and_then(|x| Some(!x))
        }
    }
}

impl Evaluatable for BinaryExpression {
    fn evaluate(&self, state: &State) -> Option<Concrete> {
        use super::BinaryOperation::*;

        let concrete_left = get!(self.left.evaluate(state));
        let concrete_right = get!(self.right.evaluate(state));

        Some(match self.operation {
            And => concrete_left & concrete_right,
            Or => concrete_left | concrete_right,
            LShift => concrete_left << concrete_right,
            RShift => concrete_left >> concrete_right
        })
    }
}

impl Evaluatable for Expression {
    fn evaluate(&self, state: &State) -> Option<Concrete> {
        use super::Expression::*;

        match *self {
            Unary(ref expression) => expression.evaluate(state),
            Binary(ref expression) => expression.evaluate(state)
        }
    }
}

impl Evaluatable for Statement {
    fn evaluate(&self, state: &State) -> Option<Concrete> {
        self.input.evaluate(state)
    }
}

pub fn evaluate_circuit(circuit: &Vec<Statement>) -> Option<State> {
    let mut toposort = TopologicalSort::<Statement>::new();

    let mut inputs  = HashMap::new();
    let mut outputs = HashMap::new();

    for statement in circuit {
        for variable in statement.dependencies() {
            let entry = inputs.entry(variable).or_insert(Vec::new());
            entry.push(statement.clone());
        }

        outputs.insert(statement.output.clone(), statement.clone());
    }

    for (variable, statement) in outputs {
        match inputs.get(&variable) {
            Some(statements) => {
                for s in statements {
                    toposort.add_dependency(statement.clone(), s.clone());
                }
            },
            _ => ()
        }

    }

    let mut sequence = Vec::new();
    loop {
        let statements = toposort.pop_all();
        if statements.is_empty() {
            break;
        }

        sequence.extend(statements);
    }

    let mut state = State::new();
    for statement in sequence {
        match statement.evaluate(&state) {
            Some(result) => { state.insert(statement.output, result); },
            _ => return None
        }
    }

    Some(state)
}

