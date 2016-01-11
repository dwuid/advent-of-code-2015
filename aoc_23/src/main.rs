
#[macro_use]
extern crate nom;

use nom::{space, newline, is_digit};
use nom::IResult::*;

use std::str::{from_utf8, FromStr};

named!(constant<i32>,
    map_res!(
        map_res!(
            take_while1!(is_digit),
            from_utf8),
        FromStr::from_str));

named!(offset<Offset>,
    alt!(
        chain!(tag!("+") ~ c: constant, ||  c as Offset) |
        chain!(tag!("-") ~ c: constant, || -c as Offset)));

named!(register<Register>,
    alt!(
        tag!("a") => { |_| Register::A } |
        tag!("b") => { |_| Register::B }));

named!(instruction<Instruction>,
    alt!(
    chain!(tag!("hlf") ~ space ~ r: register,
        || Instruction::Hlf(r)) |

    chain!(tag!("tpl") ~ space ~ r: register,
        || Instruction::Tpl(r)) |

    chain!(tag!("inc") ~ space ~ r: register,
        || Instruction::Inc(r)) |

    chain!(tag!("jmp") ~ space ~ o: offset,
        || Instruction::Jmp(o)) |

    chain!(tag!("jie") ~ space ~ r: register ~ tag!(",") ~ space ~ o: offset,
        || Instruction::Jie(r, o)) |

    chain!(tag!("jio") ~ space ~ r: register ~ tag!(",") ~ space ~ o: offset,
        || Instruction::Jio(r, o))));

named!(program_parser<Vec<Instruction> >,
       separated_list!(newline, instruction));

fn parse_program(input: &[u8]) -> Option<Vec<Instruction>> {
    match program_parser(input) {
        Done(_, result) => Some(result),
        _ => None
    }
}

struct Program {
    a: u32,
    b: u32,

    instructions: Vec<Instruction>
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Program { a: 0, b: 0, instructions: instructions }
    }

    fn get(&self, r: &Register) -> u32 {
        use Register::*;

        match r {
            &A => self.a,
            &B => self.b
        }
    }

    fn update<F>(&mut self, r: &Register, f: F) where F: Fn(u32) -> u32 {
        use Register::*;

        match r {
            &A => { self.a = f(self.a); },
            &B => { self.b = f(self.b); }
        }
    }

    fn evaluate_instruction(&mut self, ip: usize, i: &Instruction)
        -> Option<usize> {
        use Instruction::*;
        let mut ip = ip;

        match i {
            &Hlf(ref r) => { self.update(r, |x| x / 2); ip += 1 },
            &Tpl(ref r) => { self.update(r, |x| x * 3); ip += 1 },
            &Inc(ref r) => { self.update(r, |x| x + 1); ip += 1 },
            &Jmp(ref o) => ip = ((ip as i32) + o) as usize,
            &Jie(ref r, ref o) => {
                ip = if self.get(r) & 1 == 0 {
                    ((ip as i32) + o) as usize
                } else {
                    ip + 1
                };
            },
            &Jio(ref r, ref o) => {
                ip = if self.get(r) == 1 {
                    ((ip as i32) + o) as usize
                } else {
                    ip + 1
                };
            }
        }

        if ip >= self.instructions.len() { None }
        else { Some(ip) }
    }

    pub fn evaluate(&mut self, a: u32, b: u32) {
        self.a = a;
        self.b = b;

        let mut ip = 0;
        while ip < self.instructions.len() {
            let current = self.instructions[ip].clone();
            if let Some(next_ip) = self.evaluate_instruction(ip, &current) {
                ip = next_ip;
                continue;
            }

            break;
        }
    }
}

#[derive(Clone)]
enum Register {
    A,
    B
}

type Offset = i32;

#[derive(Clone)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(Offset),
    Jie(Register, Offset),
    Jio(Register, Offset)
}

fn main() {
    let input = include_bytes!("../input.txt");
    if let Some(instructions) = parse_program(input) {
        let mut program = Program::new(instructions);

        program.evaluate(0, 0);
        println!("Initialization of a = 0 yields b = {}.",
                 program.get(&Register::B));

        program.evaluate(1, 0);
        println!("Initialization of a = 1 yields b = {}.",
                 program.get(&Register::B));
    }
}

