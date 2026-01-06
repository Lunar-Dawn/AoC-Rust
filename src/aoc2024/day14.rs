use std::fmt::Write;
use std::fs;

use crate::runner;
use crate::util::grid::{Grid, VectorGrid};
use crate::util::parse::take_integers_signed;
use crate::util::point2i::Point2i;
use crate::util::vec2i::Vec2i;
use crate::util::DynResult;

runner!();

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

struct Robot {
    pos: Point2i,
    vel: Vec2i,
}
impl Robot {
    fn new(line: &str) -> DynResult<Robot> {
        let (_, [px, py, vx, vy]) = take_integers_signed(line)?;

        Ok(Robot {
            pos: Point2i::new(px, py),
            vel: Vec2i::new(vx, vy),
        })
    }

    fn pos_after(&self, width: i64, height: i64, seconds: i64) -> Point2i {
        let mut pos = self.pos + self.vel * seconds;
        pos.x = pos.x.rem_euclid(width);
        pos.y = pos.y.rem_euclid(height);

        pos
    }
}

type ParsedData = Vec<Robot>;
fn parse(input: &str) -> DynResult<ParsedData> {
    Ok(input
        .lines()
        .map(|l| Robot::new(l))
        .collect::<Result<_, _>>()?)
}

fn calc_safety_score(robots: &ParsedData, seconds: i64) -> i64 {
    let mut quadrants = [0; 4];

    for robot in robots {
        let end_pos = robot.pos_after(WIDTH, HEIGHT, seconds);

        if end_pos.x == WIDTH / 2 || end_pos.y == HEIGHT / 2 {
            continue;
        }

        let mut index = 0;
        if end_pos.x > WIDTH / 2 {
            index += 1;
        }
        if end_pos.y > HEIGHT / 2 {
            index += 2;
        }

        quadrants[index] += 1;
    }

    quadrants.iter().product()
}

fn part1(robots: &ParsedData) -> i64 {
    calc_safety_score(robots, 100)
}

fn tree_str(robots: &ParsedData, seconds: i64) -> String {
    let mut grid = VectorGrid::new(WIDTH as usize, HEIGHT as usize, 0);

    for robot in robots {
        let end_pos = robot.pos_after(WIDTH, HEIGHT, seconds);

        *grid.get_mut(&end_pos).unwrap() += 1;
    }

    // Grid spaces + newlines
    let mut ret = String::with_capacity((WIDTH * HEIGHT + HEIGHT) as usize);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            match grid.get(&Point2i::new(x, y)) {
                None => unreachable!(),
                Some(0) => write!(ret, ".").unwrap(),
                Some(i) => write!(ret, "{i}").unwrap(),
            };
        }
        writeln!(ret).unwrap();
    }

    ret
}

fn part2(robots: &ParsedData) -> String {
    let mut lowest_score = i64::MAX;
    let mut lowest_index = 0;

    for i in 0..WIDTH * HEIGHT {
        let score = calc_safety_score(&robots, i);

        if score < lowest_score {
            lowest_score = score;
            lowest_index = i;
        }
    }

    match fs::write("2024_14.txt", tree_str(&robots, lowest_index)) {
        Ok(_) => format!("{lowest_index} (My heuristic happens to be correct for my input, check 2024_14.txt to confirm)"),
        Err(e) => format!("{lowest_index} (Failed to write \"image\" to disk {e})"),
    }
}
