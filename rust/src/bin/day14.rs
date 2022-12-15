use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<Path>;

#[derive(Debug)]
struct Path {
    rocks: Vec<Pos>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

#[derive(Debug)]
struct Cave {
    rocks: HashSet<Pos>,
    rocks_max_y: i32,
    sand: HashSet<Pos>,
    floor_y: Option<i32>,
}

impl Cave {
    fn from_scan(scan: &Vec<Path>) -> Self {
        let mut rocks = HashSet::new();

        for Path { rocks: rs } in scan {
            let mut pos = rs[0].clone();
            for r in rs.iter().skip(1) {
                match (pos.x - r.x, pos.y - r.y) {
                    (_dx, 0) => {
                        let (sx, ex) = if pos.x > r.x {
                            (r.x, pos.x)
                        } else {
                            (pos.x, r.x)
                        };
                        let y = pos.y;
                        for x in sx..=ex {
                            rocks.insert(Pos { x, y });
                        }
                    }
                    (0, _dy) => {
                        let (sy, ey) = if pos.y > r.y {
                            (r.y, pos.y)
                        } else {
                            (pos.y, r.y)
                        };
                        let x = pos.x;
                        for y in sy..=ey {
                            rocks.insert(Pos { x, y });
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                }
                pos = r.clone();
            }
        }

        let rocks_max_y = rocks.iter().map(|r| r.y).max().unwrap();

        Self {
            rocks,
            rocks_max_y,
            sand: HashSet::new(),
            floor_y: None,
        }
    }

    fn with_floor(self) -> Self {
        let floor_y = Some(self.rocks_max_y + 2);
        Self { floor_y, ..self }
    }

    fn free(&self, pos: &Pos) -> bool {
        !(self.sand.contains(pos) || self.rocks.contains(pos))
            && self.floor_y.map(|fy| fy != pos.y).unwrap_or(true)
    }

    fn pour_sand(&mut self) -> bool {
        let mut sand_pos = Pos { x: 500, y: 0 };
        if self.sand.contains(&sand_pos) {
            return false;
        }
        let max_y = self.floor_y.unwrap_or(self.rocks_max_y);

        let at_rest = loop {
            sand_pos.translate(0, 1);
            if sand_pos.y > max_y {
                break self.floor_y.is_some();
            }

            if self.free(&sand_pos) {
                continue;
            }
            sand_pos.translate(-1, 0);
            if self.free(&sand_pos) {
                continue;
            }
            sand_pos.translate(2, 0);
            if self.free(&sand_pos) {
                continue;
            }
            sand_pos.translate(-1, -1);
            break true;
        };

        if at_rest {
            self.sand.insert(sand_pos);
        }

        at_rest
    }
}

fn solve(input: &Input) -> (usize, usize) {
    let mut cave = Cave::from_scan(input);

    let p1 = loop {
        if !cave.pour_sand() {
            break cave.sand.len();
        }
    };

    let mut cave = cave.with_floor();

    let p2 = loop {
        if !cave.pour_sand() {
            break cave.sand.len();
        }
    };

    (p1, p2)
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        let (part1, part2) = solve(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
        Ok(())
    })
}

impl FromStr for Pos {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().context("No x")?.parse()?;
        let y = parts.next().context("No y")?.parse()?;
        Ok(Pos { x, y })
    }
}

impl FromStr for Path {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" -> ");
        let rocks = parts
            .into_iter()
            .map(|p| p.parse::<Pos>())
            .collect::<Result<_>>()?;
        Ok(Path { rocks })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader.lines().map(|line| line?.parse::<Path>()).collect()
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9";

    fn as_input(s: &str) -> Result<Input> {
        read_input(BufReader::new(
            s.split('\n')
                .skip(1)
                .map(|s| s.trim())
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        ))
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(solve(&as_input(INPUT)?).0, 24);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(solve(&as_input(INPUT)?).1, 93);
        Ok(())
    }
}
