use crate::runner;
use crate::util::grid::{Grid, VectorGrid};
use crate::util::DynResult;
use std::collections::BTreeSet;

runner!();

#[derive(Copy, Clone)]
enum SchematicCell {
    Number(usize),
    Symbol(char),
    Empty,
}

type ParsedData = (VectorGrid<SchematicCell>, Vec<u64>);
fn parse(input: &str) -> DynResult<ParsedData> {
    let lines: Vec<_> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut schematic = VectorGrid::new(width, height, SchematicCell::Empty);
    let mut numbers = Vec::new();

    for (y, line) in lines.into_iter().enumerate() {
        let mut line = line.chars().enumerate().peekable();

        while let Some((mut x, mut c)) = line.next() {
            if c == '.' {
                continue;
            } else if c.is_ascii_digit() {
                numbers.push(0);
                let index = numbers.len() - 1;
                let num = &mut numbers[index];

                loop {
                    *num *= 10;
                    *num += c.to_digit(10).unwrap() as u64;
                    *schematic.get_xy_mut(x as i64, y as i64).unwrap() =
                        SchematicCell::Number(index);

                    if line.peek().is_none_or(|(_, c)| !c.is_ascii_digit()) {
                        break;
                    }

                    (x, c) = line.next().unwrap();
                }
            } else {
                *schematic.get_xy_mut(x as i64, y as i64).unwrap() = SchematicCell::Symbol(c);
            }
        }
    }

    Ok((schematic, numbers))
}

fn part1((schematic, numbers): &ParsedData) -> u64 {
    let mut neighbours_symbol = vec![false; numbers.len()];

    let symbols = schematic
        .pos_iter()
        .filter(|p| matches!(schematic.get(&p), Some(SchematicCell::Symbol(_))));

    for pos in symbols {
        for neighbour in pos.neighbours() {
            match schematic.get(&neighbour) {
                Some(SchematicCell::Number(i)) => neighbours_symbol[*i] = true,
                _ => {}
            }
        }
    }

    numbers
        .iter()
        .enumerate()
        .filter(|(i, _)| neighbours_symbol[*i])
        .map(|(_, v)| v)
        .sum()
}
fn part2((schematic, numbers): &ParsedData) -> u64 {
    let mut sum = 0;

    let gears = schematic
        .pos_iter()
        .filter(|p| matches!(schematic.get(&p), Some(SchematicCell::Symbol('*'))));

    for pos in gears {
        let neighbouring_numbers: BTreeSet<_> = pos
            .neighbours()
            .filter_map(|p| match schematic.get(&p) {
                Some(SchematicCell::Number(i)) => Some(*i),
                _ => None,
            })
            .collect();

        if neighbouring_numbers.len() != 2 {
            continue;
        }

        sum += neighbouring_numbers
            .into_iter()
            .map(|i| numbers[i])
            .product::<u64>();
    }

    sum
}
