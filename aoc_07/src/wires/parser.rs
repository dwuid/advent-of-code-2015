
use super::types::*;

use nom::{multispace, newline, is_digit, is_alphabetic};
use nom::IResult::*;

use std::str::{from_utf8, FromStr};

named!(unary_operation<UnaryOperation>, map!(
        tag!("NOT"), { |_| UnaryOperation::Not }));

named!(binary_operation<BinaryOperation>, alt!(
        tag!("AND")    => { |_| BinaryOperation::And } |
        tag!("OR")     => { |_| BinaryOperation::Or } |
        tag!("LSHIFT") => { |_| BinaryOperation::LShift } |
        tag!("RSHIFT") => { |_| BinaryOperation::RShift }));

named!(constant<Concrete>,
       map_res!(
           map_res!(
               take_while1!(is_digit),
               from_utf8),
           FromStr::from_str));

named!(variable<String>,
       map_res!(
           map_res!(
               take_while1!(is_alphabetic),
               from_utf8),
            FromStr::from_str));

named!(operand<Operand>, alt!(
        constant => { Operand::Constant } |
        variable => { Operand::Variable }));

named!(unary_expression<UnaryExpression>, chain!(
        operation: unary_operation ~ multispace ~
        operand: operand,

        || UnaryExpression::new(operation, operand)));

named!(binary_expression<BinaryExpression>, chain!(
        left: operand ~ multispace ~
        operation: binary_operation ~ multispace ~
        right: operand,

        || BinaryExpression::new(left, operation, right)));

fn construct_id(operand: Operand) -> Expression {
    let inner = UnaryExpression::new(UnaryOperation::Id, operand);
    Expression::Unary(inner)
}

named!(expression<Expression>, alt!(
        binary_expression => { Expression::Binary } |
        unary_expression  => { Expression::Unary } |
        operand           => { construct_id }));

named!(statement<Statement>, chain!(
        input: expression ~ multispace ~
        tag!("->") ~ multispace ~
        output: operand,

        || Statement::new(input, output)));

named!(circuit_parser<Vec<Statement> >, separated_list!(newline,
                                                        statement));

pub fn parse_circuit(input: &[u8]) -> Option<Vec<Statement>> {
    match circuit_parser(input) {
        Done(_, result) => Some(result),
        _ => None
    }
}

