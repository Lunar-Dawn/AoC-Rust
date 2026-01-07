use std::cmp::PartialEq;
use std::fmt::Display;

use crate::runner;
use crate::util::grid::{Grid, VectorGrid};
use crate::util::point2i::Point2i;
use crate::util::vec2i::Vec2i;
use crate::util::DynResult;

runner!();

#[derive(Clone, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Box,
    Robot,
    BoxLeft,
    BoxRight,
}
impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'O' => Tile::Box,
            '@' => Tile::Robot,
            _ => unreachable!("Invalid tile char: {}", c),
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Box => 'O',
            Tile::Robot => '@',
            Tile::BoxLeft => '[',
            Tile::BoxRight => ']',
        };
        write!(f, "{c}")
    }
}

#[derive(Clone)]
struct Warehouse {
    tiles: VectorGrid<Tile>,
}

impl Warehouse {
    fn move_robot(&mut self, pos: &Point2i, dir: &Vec2i) -> bool {
        if !self.can_move_tile(pos, dir) {
            return false;
        }
        self.move_tile(pos, dir);

        true
    }

    fn move_tile(&mut self, pos: &Point2i, dir: &Vec2i) {
        match self.tiles.get(&pos).unwrap() {
            Tile::Empty => (),
            Tile::Wall => unreachable!("Walls should not be movable"),
            Tile::Box | Tile::Robot => {
                let dest = pos + dir;
                self.move_tile(&dest, dir);

                self.tiles.swap(&pos, &dest);
            }
            Tile::BoxRight => {
                self.move_tile(&(pos + Vec2i::LEFT), dir);
            }
            Tile::BoxLeft => {
                let dest = pos + dir;
                let right_pos = pos + Vec2i::RIGHT;
                let right_dest = right_pos + dir;

                if dir.x == 0 {
                    self.move_tile(&dest, dir);
                    self.move_tile(&right_dest, dir);

                    self.tiles.swap(&pos, &dest);
                    self.tiles.swap(&right_pos, &right_dest);
                } else if *dir == Vec2i::LEFT {
                    self.move_tile(&dest, dir);

                    self.tiles.swap(&pos, &dest);
                    self.tiles.swap(&right_pos, &right_dest);
                } else {
                    self.move_tile(&right_dest, dir);

                    self.tiles.swap(&right_pos, &right_dest);
                    self.tiles.swap(&pos, &dest);
                }
            }
        }
    }
    fn can_move_tile(&self, pos: &Point2i, dir: &Vec2i) -> bool {
        match self.tiles.get(&pos).unwrap() {
            Tile::Empty => true,
            Tile::Wall => false,
            Tile::Box | Tile::Robot => {
                let dest = pos + dir;
                self.can_move_tile(&dest, dir)
            }
            Tile::BoxRight => self.can_move_tile(&(pos + Vec2i::LEFT), dir),
            Tile::BoxLeft => {
                let right_pos = pos + Vec2i::RIGHT;
                if dir.x == 0 {
                    self.can_move_tile(&(pos + dir), dir)
                        && self.can_move_tile(&(right_pos + dir), dir)
                } else if *dir == Vec2i::LEFT {
                    self.can_move_tile(&(pos + dir), dir)
                } else {
                    self.can_move_tile(&(right_pos + dir), dir)
                }
            }
        }
    }
    fn gps_sum(&self) -> i64 {
        self.tiles
            .pos_iter()
            .map(|p| (p, self.tiles.get(&p).unwrap()))
            .filter(|(_, t)| **t == Tile::Box || **t == Tile::BoxLeft)
            .map(|(p, _)| p.x + p.y * 100)
            .sum()
    }

    fn widen(self) -> Self {
        let data = self
            .tiles
            .pos_iter()
            .flat_map(|p| match self.tiles.get(&p).unwrap() {
                Tile::Empty => [Tile::Empty, Tile::Empty],
                Tile::Wall => [Tile::Wall, Tile::Wall],
                Tile::Box => [Tile::BoxLeft, Tile::BoxRight],
                Tile::Robot => [Tile::Robot, Tile::Empty],
                Tile::BoxLeft => unreachable!(),
                Tile::BoxRight => unreachable!(),
            })
            .collect();

        let new_grid = VectorGrid::from(self.tiles.width() * 2, self.tiles.height(), data);
        Warehouse { tiles: new_grid }
    }
}

fn to_dir(c: char) -> Vec2i {
    match c {
        '^' => Vec2i::UP,
        '>' => Vec2i::RIGHT,
        'v' => Vec2i::DOWN,
        '<' => Vec2i::LEFT,
        _ => unreachable!("Invalid direction: {}", c),
    }
}
type ParsedData = (Warehouse, Vec<Vec2i>, Point2i);
fn parse(input: &str) -> DynResult<ParsedData> {
    let mut lines = input.lines();

    let grid_lines: Vec<_> = lines.by_ref().take_while(|l| !l.is_empty()).collect();
    let width = grid_lines[0].len();
    let height = grid_lines.len();

    let tiles = VectorGrid::from(
        width,
        height,
        grid_lines
            .into_iter()
            .flat_map(str::chars)
            .map(Tile::from_char)
            .collect(),
    );

    let moves = lines.flat_map(str::chars).map(to_dir).collect();

    let (robot_pos, _) = tiles
        .pos_iter()
        .map(|p| (p, tiles.get(&p).unwrap()))
        .find(|(_, t)| **t == Tile::Robot)
        .unwrap();

    Ok((Warehouse { tiles }, moves, robot_pos))
}

fn part1((warehouse, moves, mut robot_pos): &ParsedData) -> i64 {
    let mut warehouse = warehouse.clone();

    for dir in moves {
        if warehouse.move_robot(&robot_pos, dir) {
            robot_pos += dir;
        }
    }

    warehouse.gps_sum()
}
fn part2((warehouse, moves, mut robot_pos): &ParsedData) -> i64 {
    let mut warehouse = warehouse.clone().widen();
    robot_pos.x *= 2;

    for dir in moves {
        if warehouse.move_robot(&robot_pos, dir) {
            robot_pos += dir;
        }
    }

    warehouse.gps_sum()
}
