use crate::dyn_result::DynResult;
use crate::runner;

runner!();

enum Operation {
    Addition,
    Multiplication,
}
struct Problem {
    numbers_normal: Vec<u64>,
    numbers_cephalopod: Vec<u64>,
    operator: Operation,
}
impl Problem {
    fn evaluate1(&self) -> u64 {
        match self.operator {
            Operation::Addition => self.numbers_normal.iter().sum(),
            Operation::Multiplication => self.numbers_normal.iter().product(),
        }
    }
    fn evaluate2(&self) -> u64 {
        match self.operator {
            Operation::Addition => self.numbers_cephalopod.iter().sum(),
            Operation::Multiplication => self.numbers_cephalopod.iter().product(),
        }
    }
}

fn parse(input: &str) -> DynResult<Vec<Problem>> {
    let mut lines: Vec<_> = input.lines().collect();
    let Some(operator_row) = lines.pop() else {
        return Err("Missing 'operator' line".into());
    };

    let mut current_operation_start_pos = 0;
    let mut problems = Vec::new();

    while current_operation_start_pos < operator_row.len() {
        let operator = operator_row[current_operation_start_pos..]
            .chars()
            .next()
            .ok_or("Scan overran operator line length")?;
        let operator = match operator {
            '+' => Operation::Addition,
            '*' => Operation::Multiplication,
            op => return Err(format!("Unknown operator {op}").into()),
        };

        let column_width =
            match operator_row[current_operation_start_pos + 1..].find(|c| c == '+' || c == '*') {
                Some(column_width) => column_width,
                None => operator_row.len() - current_operation_start_pos,
            };

        let number_strings = lines
            .iter()
            .map(|l| &l[current_operation_start_pos..current_operation_start_pos + column_width])
            .collect::<Vec<_>>();
        current_operation_start_pos += column_width + 1;

        let numbers_cephalopod = (0..column_width)
            .into_iter()
            .map(|i| {
                number_strings
                    .iter()
                    .map(|s| s.chars().nth(i).unwrap_or(' '))
                    .collect::<String>()
                    .trim()
                    .parse()
            })
            .collect::<Result<_, _>>()?;

        problems.push(Problem {
            numbers_normal: number_strings
                .into_iter()
                .map(|l| l.trim().parse())
                .collect::<Result<_, _>>()?,
            numbers_cephalopod,
            operator,
        })
    }

    Ok(problems)
}

fn part1(problems: &Vec<Problem>) -> u64 {
    problems.iter().map(|p| p.evaluate1()).sum()
}

fn part2(problems: &Vec<Problem>) -> u64 {
    problems.iter().map(|p| p.evaluate2()).sum()
}
