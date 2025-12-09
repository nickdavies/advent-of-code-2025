#![allow(unused_imports)]
advent_of_code::solution!(9);

use advent_of_code::template::RunType;
use aoc_lib::grid::{Direction, Location, Map, UnboundLocation};
use aoc_lib::parse::preamble::*;
use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result, anyhow};

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    let raw: Vec<(i64, i64)> = parse_input(
        LineSplitter,
        ParseTuple2(ParseFromStr, ParseFromStr, ","),
        input,
    )
    .context("failed to parse input")?;

    let mut points: Vec<UnboundLocation> = raw
        .into_iter()
        .map(|(x, y)| UnboundLocation(x, y))
        .collect();
    points.sort();

    let mut dists = Vec::new();
    for a in points.iter() {
        for b in points.iter() {
            if a <= b {
                break;
            }
            dists.push((a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1));
        }
    }
    dists.sort();

    Ok(Some(*dists.last().context("failed to find last element")?))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Edge,
    Inner,
    Outer,
}

fn fill(map: &mut Map<Tile>, start: Location) {
    let mut to_visit = vec![start];

    while let Some(current) = to_visit.pop() {
        if map.get(&current) != &Tile::Inner {
            continue;
        }
        *map.get_mut(&current) = Tile::Outer;
        for dir in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            if let Some(next) = map.go_direction(&current, &dir) {
                to_visit.push(next);
            }
        }
    }
}

fn valid_path(map: &Map<Tile>, start: &Location, end: &Location) -> bool {
    let start_0 = std::cmp::min(start.0, end.0);
    let end_0 = std::cmp::max(start.0, end.0);
    let start_1 = std::cmp::min(start.1, end.1);
    let end_1 = std::cmp::max(start.1, end.1);

    for zero in start_0..=end_0 {
        if map.get(&Location(zero, start_1)) == &Tile::Outer {
            return false;
        }
        if map.get(&Location(zero, start_1)) == &Tile::Outer {
            return false;
        }
    }

    for zero in start_0..=end_0 {
        for one in start_1..=end_1 {
            if start == &Location(5, 9) || end == &Location(5, 9) {
                println!(
                    "{start:?}->{end:?}: {}, {} -> {:?}",
                    zero,
                    one,
                    map.get(&Location(zero, one)),
                );
            }
            if map.get(&Location(zero, one)) == &Tile::Outer {
                return false;
            }
        }
    }
    true
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let mut raw: Vec<(usize, usize)> = parse_input(
        LineSplitter,
        ParseTuple2(ParseFromStr, ParseFromStr, ","),
        input,
    )
    .context("failed to parse input")?;

    let mut points: Vec<Location> = raw.iter().map(|(x, y)| Location(*y, *x)).collect();

    let max_x = raw
        .iter()
        .map(|(x, _)| *x)
        .max()
        .context("must find max x")?;
    let max_y = raw
        .iter()
        .map(|(_, y)| *y)
        .max()
        .context("must find max y")?;

    // Adding +1 ensures that we have an outer spot to start from at the bottom right corner
    let mut map = Map::<Tile>::from_dimensions(max_y + 2, max_x + 2, |_| Tile::Inner);
    let mut points_extra = points.clone();
    points_extra.push(
        points
            .first()
            .context("required to have at least 1 element")?
            .clone(),
    );
    for slice in points_extra.windows(2) {
        let mut current = slice[0].clone();
        let end = slice[1].clone();
        let dir = current
            .direction_between(&end)
            .context("expected to find route between path points")?;
        *map.get_mut(&end) = Tile::Edge;
        while current != end {
            *map.get_mut(&current) = Tile::Edge;
            current = map
                .go_direction(&current, &dir)
                .context("unexpectedly ran out of bounds")?
                .clone();
        }
    }

    let bottom_right = map
        .bottom_right()
        .context("expected to find bottom right")?;

    // map.print(|v, _| match v {
    //     Tile::Empty => '.',
    //     Tile::Inner => 'I',
    //     Tile::Outer => 'o',
    //     Tile::Edge => '#',
    // });
    println!("{:?}", bottom_right);
    return Ok(Some(0));
    fill(&mut map, bottom_right);

    // map.print(|v, _| match v {
    //     Tile::Empty => '.',
    //     Tile::Inner => 'I',
    //     Tile::Outer => 'o',
    //     Tile::Edge => '#',
    // });

    // let mut dists = Vec::new();
    let mut max_dist: Option<(usize, Location, Location)> = None;
    points.sort();
    for a in points.iter() {
        for b in points.iter() {
            if a <= b {
                break;
            }
            if valid_path(&map, a, b) {
                let dist = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
                max_dist = Some(match max_dist {
                    Some(existing) => {
                        if dist >= existing.0 {
                            (dist, a.clone(), b.clone())
                        } else {
                            existing
                        }
                    }
                    None => (dist, a.clone(), b.clone()),
                });
            }
        }
    }
    println!("max_dist={max_dist:?}");

    Ok(Some(max_dist.context("failed to find last element")?.0))
}

#[cfg(test)]
mod tests_day_9 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(50);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(24);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
