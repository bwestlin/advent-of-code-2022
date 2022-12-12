use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

type Input = Heightmap;

#[derive(Debug)]
struct Heightmap {
    rows: Vec<Vec<u8>>,
    start: Pos,
    best_signal: Pos,
}

impl Heightmap {
    fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.rows[0].len() as i32 && y >= 0 && y < self.rows.len() as i32
    }

    fn at(&self, x: i32, y: i32) -> u8 {
        self.rows[y as usize][x as usize]
    }

    fn width(&self) -> i32 {
        self.rows[0].len() as i32
    }

    fn height(&self) -> i32 {
        self.rows.len() as i32
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn adjacent(&self) -> Vec<Pos> {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| Pos::new(self.x + dx, self.y + dy))
            .collect()
    }
}

fn least_steps_to_signal(map: &Heightmap, start: Pos) -> Option<usize> {
    let mut queue = VecDeque::<(Pos, usize)>::new();
    let mut visited = HashMap::<Pos, usize>::new();

    queue.push_back((start, 0));
    visited.insert(start, 0);
    let mut least_steps = 0;

    while let Some((pos, steps)) = queue.pop_front() {
        if pos == map.best_signal {
            least_steps = steps;
            break;
        }

        let curr_height = map.at(pos.x, pos.y);

        for pos in pos.adjacent() {
            if !map.is_inside(pos.x, pos.y) {
                continue;
            }
            if let Some(vis_steps) = visited.get(&pos) {
                if *vis_steps <= steps + 1 {
                    continue;
                }
            }
            let height = map.at(pos.x, pos.y);
            if height > curr_height + 1 {
                continue;
            }

            queue.push_back((pos, steps + 1));
            visited.insert(pos, steps + 1);
        }
    }

    if least_steps > 0 {
        Some(least_steps)
    } else {
        None
    }
}

fn part1(input: &Input) -> usize {
    least_steps_to_signal(input, input.start).unwrap_or_default()
}

fn part2(input: &Input) -> usize {
    let mut starting_points = vec![];
    for y in 0..input.height() {
        for x in 0..input.width() {
            if input.at(x, y) == b'a' {
                starting_points.push(Pos::new(x, y));
            }
        }
    }

    let mut steps = vec![];

    for start_pos in starting_points {
        if let Some(least_steps) = least_steps_to_signal(input, start_pos) {
            steps.push(least_steps);
        }
    }

    steps.into_iter().min().unwrap()
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let mut rows = vec![];
    let mut start = None;
    let mut best_signal = None;
    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        let mut row = vec![];
        for (x, mut c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some(Pos::new(x as i32, y as i32));
                c = 'a';
            } else if c == 'E' {
                best_signal = Some(Pos::new(x as i32, y as i32));
                c = 'z';
            }
            row.push(c as u8);
        }
        rows.push(row);
    }

    let start = start.unwrap();
    let best_signal = best_signal.unwrap();

    Ok(Heightmap {
        rows,
        start,
        best_signal,
    })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi";

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
        assert_eq!(part1(&as_input(INPUT)?), 31);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 29);
        Ok(())
    }
}
