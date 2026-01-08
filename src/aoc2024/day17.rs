use crate::runner;
use crate::util::parse::{scan_integers, take_integers};
use crate::util::DynResult;

runner!();

#[derive(Clone)]
struct Program {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,

    numbers: Vec<u8>,
}
impl Program {
    fn new(s: &str) -> Self {
        let (s, [reg_a, reg_b, reg_c]) = take_integers(s).unwrap();
        let numbers = scan_integers(s);

        Program {
            reg_a,
            reg_b,
            reg_c,

            numbers,
        }
    }
    fn combo(&self, op: u8) -> u64 {
        match op {
            n @ 0..=3 => n as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            op => unreachable!("Invalid op: {}", op),
        }
    }

    fn run_once(&mut self) -> (bool, u8) {
        let mut out = 0;

        for pos in 0..self.numbers.len() / 2 {
            match (self.numbers[pos * 2], self.numbers[pos * 2 + 1]) {
                (0, op) => self.reg_a = self.reg_a / (1 << self.combo(op)),
                (1, op) => self.reg_b ^= op as u64,
                (2, op) => self.reg_b = self.combo(op) & 0b111,
                (3, _) => return (self.reg_a == 0, out),
                (4, _) => self.reg_b ^= self.reg_c,
                (5, op) => out = (self.combo(op) & 0b111) as u8,
                (6, op) => self.reg_b = self.reg_a / (1 << self.combo(op)),
                (7, op) => self.reg_c = self.reg_a / (1 << self.combo(op)),
                _ => unreachable!("Invalid op: {}", self.numbers[pos]),
            }
        }

        unreachable!("Did not encounter a jump")
    }
    fn run(&mut self) -> Vec<u8> {
        let mut out = Vec::new();

        loop {
            let (exited, ret) = self.run_once();
            out.push(ret);

            if exited {
                break;
            }
        }

        out
    }

    // The way my program worked looked *quite* intentional, with only one output,
    // only one modification of A, and with B & C being completely determined by A on each loop.
    // So this doesn't generalise at all beyond that.
    fn find_part2(&mut self, final_a: u64, target_index: usize) -> Option<u64> {
        let taget = self.numbers[target_index];

        for i in final_a..final_a + 8 {
            self.reg_a = i;

            let (_, out) = self.run_once();
            if out == taget {
                if target_index == 0 {
                    return Some(i);
                }

                let next = self.find_part2(i * 8, target_index - 1);
                if next.is_some() {
                    return next;
                }
            }
        }

        None
    }

    fn part1(&mut self) -> Vec<u8> {
        self.run()
    }
    fn part2(&mut self) -> u64 {
        self.find_part2(0, self.numbers.len() - 1).unwrap()
    }
}

type ParsedData = Program;
fn parse(input: &str) -> DynResult<ParsedData> {
    Ok(Program::new(input))
}

fn part1(program: &ParsedData) -> String {
    let mut program = program.clone();
    let out: Vec<_> = program.part1().into_iter().map(|i| i.to_string()).collect();
    out.join(",")
}
fn part2(program: &ParsedData) -> u64 {
    let mut program = program.clone();
    program.part2()
}
