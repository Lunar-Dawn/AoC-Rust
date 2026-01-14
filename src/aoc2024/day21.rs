use std::borrow::Cow;
use std::collections::HashMap;

use crate::runner;
use crate::util::parse::take_integers;
use crate::util::point2i::Point2i;
use crate::util::DynResult;

runner!();

type MemoizeCache<'a> = HashMap<(Cow<'a, str>, i8), u64>;
fn best_paths(start: char, target: char) -> Vec<Cow<'static, str>> {
    match start {
        '^' => match target {
            '^' => vec!["A"],
            '>' => vec![">vA", "v>A"],
            'v' => vec!["vA"],
            '<' => vec!["v<A"],
            'A' => vec![">A"],
            _ => unreachable!(),
        },
        '>' => match target {
            '^' => vec!["<^A", "^<A"],
            '>' => vec!["A"],
            'v' => vec!["<A"],
            '<' => vec!["<<A"],
            'A' => vec!["^A"],
            _ => unreachable!(),
        },
        'v' => match target {
            '^' => vec!["^A"],
            '>' => vec![">A"],
            'v' => vec!["A"],
            '<' => vec!["<A"],
            'A' => vec![">^A", "^>A"],
            _ => unreachable!(),
        },
        '<' => match target {
            '^' => vec![">^A"],
            '>' => vec![">>A"],
            'v' => vec![">A"],
            '<' => vec!["A"],
            'A' => vec![">>^A"],
            _ => unreachable!(),
        },
        'A' => match target {
            '^' => vec!["<A"],
            '>' => vec!["vA"],
            'v' => vec!["<vA", "v<A"],
            '<' => vec!["v<<A"],
            'A' => vec!["A"],
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
    .into_iter()
    .map(|s| s.into())
    .collect()
}

fn dpad_cost<'a>(input: (Cow<'a, str>, i8), memory: &mut MemoizeCache<'a>) -> u64 {
    let (target_path, num_dpads) = &input;

    if *num_dpads == 0 {
        return target_path.len() as u64;
    }
    if let Some(ret) = memory.get(&input) {
        return *ret;
    }

    let mut pos = 'A';

    let mut total = 0;

    for dir in target_path.chars() {
        total += best_paths(pos, dir)
            .into_iter()
            .map(|path| dpad_cost((path, num_dpads - 1), memory))
            .min()
            .unwrap();
        pos = dir;
    }

    memory.insert(input, total);

    total
}

fn char_to_pos(c: char) -> Point2i {
    match c {
        '7' => Point2i::new(0, 0),
        '8' => Point2i::new(1, 0),
        '9' => Point2i::new(2, 0),
        '4' => Point2i::new(0, 1),
        '5' => Point2i::new(1, 1),
        '6' => Point2i::new(2, 1),
        '1' => Point2i::new(0, 2),
        '2' => Point2i::new(1, 2),
        '3' => Point2i::new(2, 2),

        '0' => Point2i::new(1, 3),
        'A' => Point2i::new(2, 3),
        _ => unreachable!(),
    }
}
fn numpad_path<'a>(start: char, target: char) -> Vec<Cow<'a, str>> {
    let start_pos = char_to_pos(start);
    let target_pos = char_to_pos(target);

    let horizontal = if start_pos.x < target_pos.x {
        ">".repeat(start_pos.x.abs_diff(target_pos.x) as usize)
    } else {
        "<".repeat(start_pos.x.abs_diff(target_pos.x) as usize)
    };

    let vertical = if start_pos.y < target_pos.y {
        "v".repeat(start_pos.y.abs_diff(target_pos.y) as usize)
    } else {
        "^".repeat(start_pos.y.abs_diff(target_pos.y) as usize)
    };

    let mut ret = Vec::new();

    // Going down then right would cross the blank space, so don't
    if !(start_pos.x == 0 && target_pos.y == 3) {
        ret.push(format!("{vertical}{horizontal}A").into());
    }

    // Going left then up would cross the blank space, so don't
    if !(start_pos.y == 3 && target_pos.x == 0) {
        ret.push(format!("{horizontal}{vertical}A").into());
    }

    ret
}

fn code_cost<'a>(goal: &str, num_dpads: i8, memory: &mut MemoizeCache<'a>) -> u64 {
    let mut total = 0;

    let mut pos = 'A';

    for c in goal.chars() {
        let paths = numpad_path(pos, c);
        total += paths
            .into_iter()
            .map(|p| dpad_cost((p, num_dpads), memory))
            .min()
            .unwrap();
        pos = c;
    }

    total
}

type ParsedData = (u64, u64);
fn parse(input: &str) -> DynResult<ParsedData> {
    let mut memory = HashMap::new();

    let mut part1 = 0;
    let mut part2 = 0;

    for code in input.lines() {
        let (_, [numeric_part]) = take_integers::<1, u64>(code)?;
        part1 += code_cost(code, 2, &mut memory) * numeric_part;
        part2 += code_cost(code, 25, &mut memory) * numeric_part;
    }

    Ok((part1, part2))
}

fn part1((ret, _): &ParsedData) -> u64 {
    *ret
}

fn part2((_, ret): &ParsedData) -> u64 {
    *ret
}
