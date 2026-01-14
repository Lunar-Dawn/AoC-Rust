use std::collections::HashMap;
use std::mem::swap;

use crate::runner;
use crate::util::DynResult;

runner!();

enum Gate<'a> {
    AND(&'a str, &'a str),
    OR(&'a str, &'a str),
    XOR(&'a str, &'a str),
    WIRE(bool),
}
impl<'a> Gate<'a> {
    fn new_wire(line: &'a str) -> DynResult<(&'a str, Self)> {
        let mut split = line.split_whitespace();
        let name = split.next().unwrap();
        let name = &name[..name.len() - 1];

        let state = split.next().unwrap().parse::<u8>()? == 1;

        Ok((name, Gate::WIRE(state)))
    }
    fn new_composite(line: &'a str) -> DynResult<(&'a str, Self)> {
        let mut split = line.split_whitespace();
        let mut in1 = split.next().unwrap();
        let gate = split.next().unwrap();
        let mut in2 = split.next().unwrap();
        let name = split.skip(1).next().unwrap();

        if in1 > in2 {
            swap(&mut in1, &mut in2)
        }

        if gate == "AND" {
            Ok((name, Gate::AND(in1, in2)))
        } else if gate == "OR" {
            Ok((name, Gate::OR(in1, in2)))
        } else {
            Ok((name, Gate::XOR(in1, in2)))
        }
    }

    fn calc(&self, gates: &'a ParsedData) -> bool {
        match self {
            Gate::AND(in1, in2) => gates[in1].calc(gates) && gates[in2].calc(gates),
            Gate::OR(in1, in2) => gates[in1].calc(gates) || gates[in2].calc(gates),
            Gate::XOR(in1, in2) => gates[in1].calc(gates) ^ gates[in2].calc(gates),
            Gate::WIRE(s) => *s,
        }
    }
    fn inputs(&self) -> Vec<&'a str> {
        match self {
            Gate::AND(n1, n2) => vec![n1, n2],
            Gate::OR(n1, n2) => vec![n1, n2],
            Gate::XOR(n1, n2) => vec![n1, n2],
            Gate::WIRE(_) => vec![],
        }
    }

    fn outputs_to_xor_and(gates: &'a ParsedData, outputs: &Vec<&str>) -> bool {
        if outputs.len() != 2 {
            return false;
        }

        match (&gates[outputs[0]], &gates[outputs[1]]) {
            (Gate::XOR(_, _), Gate::AND(_, _)) => true,
            (Gate::AND(_, _), Gate::XOR(_, _)) => true,
            _ => false,
        }
    }

    // I had this really elegant solution planned around simulating each bit's associated addition
    // and swapping chips until it worked bit-by-bit. And then I realised that it's probably just
    // an actual ripple-carry adder that's been messed up.
    // I was correct, so looking for the chips that break the rules of those is enough.
    fn follows_full_adder_rules(
        &self,
        name: &str,
        gates: &'a ParsedData,
        outputs: &HashMap<&str, Vec<&str>>,
    ) -> bool {
        let outputs = &outputs[name];

        match *self {
            Gate::AND(i1, i2) => {
                if i1 == "x00" && i2 == "y00" {
                    Self::outputs_to_xor_and(gates, outputs)
                } else {
                    outputs.len() == 1 && matches!(gates[outputs[0]], Gate::OR(_, _))
                }
            }
            Gate::OR(_, _) => {
                if outputs.len() != 2 {
                    name == "z45"
                } else {
                    Self::outputs_to_xor_and(gates, outputs)
                }
            }
            Gate::XOR(i1, i2) => {
                if i1 == "x00" && i2 == "y00" {
                    name.starts_with('z')
                } else if i1.starts_with('x') && i2.starts_with('y') {
                    Self::outputs_to_xor_and(gates, outputs)
                } else {
                    name.starts_with('z')
                }
            }
            Gate::WIRE(_) => outputs.len() == 2,
        }
    }
}

type ParsedData<'a> = HashMap<&'a str, Gate<'a>>;
fn parse(input: &'_ str) -> DynResult<ParsedData<'_>> {
    let mut lines = input.lines();

    let mut gates = HashMap::new();

    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let (name, gate) = Gate::new_wire(line)?;
        gates.insert(name, gate);
    }
    for line in lines {
        let (name, gate) = Gate::new_composite(line)?;
        gates.insert(name, gate);
    }

    Ok(gates)
}

fn part1(gates: &ParsedData) -> u64 {
    let mut ret = 0;
    for i in 0.. {
        let Some(gate) = gates.get(format!("z{i:02}").as_str()) else {
            break;
        };

        if gate.calc(&gates) {
            ret |= 1 << i;
        }
    }

    ret
}

fn part2(gates: &ParsedData) -> String {
    let mut ret = Vec::new();

    let mut outputs = HashMap::new();

    for (&name, gate) in gates {
        outputs.entry(name).or_insert(Vec::new());
        for inputs_from in gate.inputs() {
            outputs.entry(inputs_from).or_insert(Vec::new()).push(name);
        }
    }

    for (&name, gate) in gates {
        if !gate.follows_full_adder_rules(name, gates, &outputs) {
            ret.push(name);
        }
    }

    ret.sort();

    ret.join(",")
}
