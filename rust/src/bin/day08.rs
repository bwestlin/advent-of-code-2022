use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

type Input = Map;

#[derive(Debug)]
struct Map {
    rows: Vec<Vec<u8>>,
}

impl Map {
    fn at(&self, x: usize, y: usize) -> u8 {
        self.rows[y][x]
    }

    fn width(&self) -> usize {
        self.rows[0].len()
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width() as i32 && y >= 0 && y < self.height() as i32
    }

    fn is_inside_edge(&self, x: i32, y: i32) -> bool {
        x >= 1 && x < self.width() as i32 - 1 && y >= 1 && y < self.height() as i32 - 1
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let h = self.at(x, y);
        let mut score = 1;
        for (xd, yd) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let mut x = x as i32 + xd;
            let mut y = y as i32 + yd;
            let mut n_trees = 0;

            while self.is_inside(x, y) {
                n_trees += 1;
                if self.at(x as usize, y as usize) >= h {
                    break;
                }
                x += xd;
                y += yd;
            }

            score *= n_trees;
        }
        score
    }
}

fn part1(input: &Input) -> usize {
    let w = input.width();
    let h = input.height();

    let by_x = 1..(w - 1);
    let by_y = 1..(h - 1);

    let top = by_x.clone().map(|x| (x, 0, 0, 1));
    let bottom = by_x.map(|x| (x, h - 1, 0, -1));
    let left = by_y.clone().map(|y| (0, y, 1, 0));
    let right = by_y.map(|y| (w - 1, y, -1, 0));
    let all = top.chain(bottom).chain(left).chain(right);

    let mut visible = HashSet::new();

    for (start_x, start_y, dx, dy) in all {
        let mut x = start_x as i32;
        let mut y = start_y as i32;
        let mut max_h = input.at(x as usize, y as usize);
        x += dx;
        y += dy;

        while input.is_inside_edge(x, y) {
            let h = input.at(x as usize, y as usize);
            if h > max_h {
                visible.insert((x, y));
                max_h = h;
            }
            x += dx;
            y += dy;
        }
    }

    visible.len() + w * 2 + h * 2 - 4
}

fn part2(input: &Input) -> usize {
    let mut score = 0;

    for y in 0..input.height() {
        for x in 0..input.width() {
            let s = input.scenic_score(x, y);
            if s > score {
                score = s;
            }
        }
    }

    score
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
    let rows = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c as u8 - b'0')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok(Map { rows })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "
        30373
        25512
        65332
        33549
        35390";

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
        assert_eq!(part1(&as_input(INPUT)?), 21);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 8);
        Ok(())
    }
}
