
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum UnaryOperation {
    Not, Id
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum BinaryOperation {
    And, Or, LShift, RShift
}

pub type Concrete = u16;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Operand {
    Constant(Concrete),
    Variable(String)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct UnaryExpression {
    pub operation: UnaryOperation,
    pub operand: Operand
}

impl UnaryExpression {
    pub fn new(operation: UnaryOperation, operand: Operand) -> Self {
        UnaryExpression { operation: operation, operand: operand }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct BinaryExpression {
    pub left: Operand,
    pub operation: BinaryOperation,
    pub right: Operand
}

impl BinaryExpression {
    pub fn new(left: Operand, operation: BinaryOperation, right: Operand)
        -> Self {
        BinaryExpression { left: left, operation: operation, right: right }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Expression {
    Unary(UnaryExpression),
    Binary(BinaryExpression)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Statement {
    pub input: Expression,
    pub output: Operand
}

impl Statement {
    pub fn new(input: Expression, output: Operand) -> Self {
        Statement { input: input, output: output }
    }

    pub fn dependencies(&self) -> Vec<Operand> {
        use self::Expression::*;
        use self::Operand::*;

        let mut operands = Vec::new();
        match self.input {
            Unary(ref e) => operands.push(e.operand.clone()),
            Binary(ref e) => {
                operands.push(e.left.clone());
                operands.push(e.right.clone());
            }
        };

        operands.into_iter().filter(|o| match *o {
            Variable(_) => true,
            _ => false
        }).collect()
    }
}

