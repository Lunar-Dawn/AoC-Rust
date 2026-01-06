use crate::runner;
use crate::util::point2i::Point2i;
use crate::util::vec2i::Vec2i;
use crate::util::{parse, DynResult};

runner!();

struct Machine {
    button_a: Vec2i,
    button_b: Vec2i,
    price_pos: Point2i,
}
impl Machine {
    fn new<'a, I>(lines: &mut I) -> DynResult<Machine>
    where
        I: Iterator<Item = &'a str>,
    {
        let (_, [x, y]) = parse::take_integers(lines.next().unwrap())?;
        let button_a = Vec2i::new(x, y);

        let (_, [x, y]) = parse::take_integers(lines.next().unwrap())?;
        let button_b = Vec2i::new(x, y);

        let (_, [x, y]) = parse::take_integers(lines.next().unwrap())?;
        let price_pos = Point2i::new(x, y);

        Ok(Machine {
            button_a,
            button_b,
            price_pos,
        })
    }

    fn det(a: i64, b: i64, c: i64, d: i64) -> i64 {
        (a * d) - (b * c)
    }

    // Basic implementation of Cramer's rule since 2x2 determinants are so easy to calculate
    fn presses(&self, correct_error: bool) -> Option<(i64, i64)> {
        let target = if correct_error {
            self.price_pos + Vec2i::new(10_000_000_000_000, 10_000_000_000_000)
        } else {
            self.price_pos
        };

        let matrix_det = Self::det(
            self.button_a.x,
            self.button_b.x,
            self.button_a.y,
            self.button_b.y,
        );
        let a_det = Self::det(target.x, self.button_b.x, target.y, self.button_b.y);
        let b_det = Self::det(self.button_a.x, target.x, self.button_a.y, target.y);

        let a_presses = a_det / matrix_det;
        let b_presses = b_det / matrix_det;

        // Checks that things make sense after the implicit flooring from integer division
        if (Point2i::ORIGIN + (self.button_a * a_presses) + (self.button_b * b_presses)) == target {
            Some((a_presses, b_presses))
        } else {
            None
        }
    }
}

type ParsedData = Vec<Machine>;
fn parse(input: &str) -> DynResult<ParsedData> {
    let mut lines = input.lines();

    let mut machines = Vec::new();

    loop {
        machines.push(Machine::new(&mut lines)?);
        if lines.next().is_none() {
            break;
        }
    }

    Ok(machines)
}

fn part1(machines: &ParsedData) -> i64 {
    machines
        .iter()
        .filter_map(|m| m.presses(false))
        .filter(|(a, b)| a <= &100 && b <= &100)
        .map(|(a, b)| a * 3 + b)
        .sum()
}
fn part2(machines: &ParsedData) -> i64 {
    machines
        .iter()
        .filter_map(|m| m.presses(true))
        .map(|(a, b)| a * 3 + b)
        .sum()
}
