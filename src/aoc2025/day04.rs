use std::collections::HashSet;
use std::fmt;

use crate::dyn_result::DynResult;
use crate::runner;

runner!();

fn parse(input: &str) -> DynResult<Wall> {
    Ok(Wall::new(input.lines().collect()))
}

type Position = (usize, usize);
#[derive(Clone)]
enum Space {
    Full(usize),
    Empty,
}

impl Space {
    fn removable(&self) -> bool {
        match self {
            Self::Full(n) => *n <= 3,
            Self::Empty => false,
        }
    }
}

#[derive(Clone)]
struct Wall {
    width: usize,
    height: usize,
    spaces: Vec<Space>,
}

impl Wall {
    fn new(lines: Vec<&str>) -> Wall {
        let width = lines[0].len();
        let height = lines.len();

        let mut wall = Wall {
            width,
            height,
            spaces: vec![Space::Empty; width * height],
        };

        for (y, line) in lines.into_iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '@' {
                    continue;
                }

                wall.insert((x, y));
            }
        }

        wall
    }
    fn pos(&self, p: Position) -> usize {
        p.1 * self.height + p.0
    }
    fn at(&self, p: Position) -> &Space {
        let pos = self.pos(p);
        &self.spaces[pos]
    }
    fn at_mut(&mut self, p: Position) -> &mut Space {
        let pos = self.pos(p);
        &mut self.spaces[pos]
    }

    fn valid_neighbours(&self, p: Position) -> impl Iterator<Item = Position> + use<'_> {
        (-1..=1)
            .flat_map(|x| (-1..=1).map(move |y| (x, y)))
            .filter(|p| p.0 != 0 || p.1 != 0)
            .filter_map(move |dp| {
                let (dx, dy) = dp;
                let x = match p.0.checked_add_signed(dx) {
                    Some(x) => x,
                    None => return None,
                };
                let y = match p.1.checked_add_signed(dy) {
                    Some(y) => y,
                    None => return None,
                };
                if x >= self.width || y >= self.height {
                    return None;
                }
                Some((x, y))
            })
    }

    fn insert(&mut self, pos: Position) {
        let mut num_neighbours = 0;

        for neighbour in self.valid_neighbours(pos).collect::<Vec<_>>() {
            let Space::Full(n) = self.at_mut(neighbour) else {
                continue;
            };
            num_neighbours += 1;
            *n += 1;
        }

        let space = self.at_mut(pos);
        if matches!(space, Space::Full(_)) {
            panic!("attempted to insert duplicate roll");
        }

        *space = Space::Full(num_neighbours);
    }
    fn remove(&mut self, pos: Position) -> HashSet<Position> {
        let mut new_removals = HashSet::new();

        for neighbour_pos in self.valid_neighbours(pos).collect::<Vec<_>>() {
            let space = self.at_mut(neighbour_pos);
            let Space::Full(n) = space else {
                continue;
            };
            *n -= 1;
            if space.removable() {
                new_removals.insert(neighbour_pos);
            }
        }

        let space = self.at_mut(pos);
        if matches!(space, Space::Empty) {
            panic!("attempted to remove non-existent roll");
        }

        *space = Space::Empty;
        new_removals
    }
}
impl fmt::Display for Wall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                output.push(match self.at((x, y)) {
                    Space::Full(n) => n.to_string().chars().nth(0).unwrap(),
                    Space::Empty => '.',
                })
            }
            output.push('\n');
        }

        write!(f, "{output}")
    }
}

fn part1(wall: &Wall) -> usize {
    let movable = wall
        .spaces
        .iter()
        .filter(|s| s.removable())
        .collect::<Vec<_>>();

    movable.len()
}

fn part2(wall: &Wall) -> u64 {
    let mut wall = wall.clone();

    let mut removable: HashSet<Position> = (0..wall.height)
        .flat_map(|x| (0..wall.width).map(move |y| (x, y)))
        .filter(|p| wall.at(*p).removable())
        .collect();

    let mut num_removals = 0;

    while !removable.is_empty() {
        let p = *removable.iter().next().unwrap();
        removable.remove(&p);

        let new_removals = wall.remove(p);
        removable.extend(new_removals.into_iter());

        num_removals += 1;
    }

    num_removals
}
