advent_of_code::solution!(9);

use advent_of_code::template::RunType;
use aoc_lib::grid::{Direction, Location, Map, UnboundLocation};
use aoc_lib::parse::preamble::*;
use std::collections::BTreeSet;

use anyhow::{Context, Result, anyhow};

#[derive(Debug, Clone, PartialEq, Eq)]
enum EdgeType {
    Vertical,
    Horizontal,
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

impl EdgeType {
    #[allow(dead_code)]
    fn to_char(&self) -> char {
        match self {
            Self::Vertical => '│',
            Self::Horizontal => '─',
            Self::TopLeft => '┌',
            Self::BottomLeft => '└',
            Self::TopRight => '┐',
            Self::BottomRight => '┘',
        }
    }

    fn from_directions(first: &Direction, second: &Direction) -> Self {
        match first {
            Direction::North => match second {
                Direction::North => EdgeType::Vertical,
                Direction::South => EdgeType::Vertical,
                Direction::East => EdgeType::TopLeft,
                Direction::West => EdgeType::TopRight,
            },
            Direction::South => match second {
                Direction::North => EdgeType::Vertical,
                Direction::South => EdgeType::Vertical,
                Direction::East => EdgeType::BottomLeft,
                Direction::West => EdgeType::BottomRight,
            },
            Direction::East => match second {
                Direction::North => EdgeType::BottomRight,
                Direction::South => EdgeType::TopRight,
                Direction::East => EdgeType::Horizontal,
                Direction::West => EdgeType::Horizontal,
            },
            Direction::West => match second {
                Direction::North => EdgeType::BottomLeft,
                Direction::South => EdgeType::TopLeft,
                Direction::East => EdgeType::Horizontal,
                Direction::West => EdgeType::Horizontal,
            },
        }
    }
}

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
enum Tile {
    Empty,
    Edge(EdgeType),
    Inner,
    Outer,
}

fn fill(map: &mut Map<Tile>) -> Result<()> {
    let width = map.width().context("failed to get width")?;
    for row in 0..map.0.len() {
        let mut inside = false;
        let mut inside_dir: Option<Direction> = None;
        for col in 0..width {
            let loc = Location(row, col);
            if let Tile::Edge(edge_type) = map.get(&loc) {
                match edge_type {
                    EdgeType::Vertical => inside = !inside,
                    EdgeType::Horizontal => {}
                    EdgeType::TopLeft => {
                        if inside {
                            inside_dir = Some(Direction::North);
                        } else {
                            inside_dir = Some(Direction::South);
                        }
                    }
                    EdgeType::BottomLeft => {
                        if inside {
                            inside_dir = Some(Direction::South);
                        } else {
                            inside_dir = Some(Direction::North);
                        }
                    }
                    EdgeType::TopRight => match inside_dir {
                        Some(Direction::North) => {
                            inside = true;
                        }
                        Some(Direction::South) => {
                            inside = false;
                        }
                        _ => {}
                    },
                    EdgeType::BottomRight => match inside_dir {
                        Some(Direction::North) => {
                            inside = false;
                        }
                        Some(Direction::South) => {
                            inside = true;
                        }
                        _ => {}
                    },
                }
            } else if inside {
                *map.get_mut(&loc) = Tile::Inner;
            } else {
                *map.get_mut(&loc) = Tile::Outer;
            }
        }
    }
    Ok(())
}
fn make_path(mut points: Vec<UnboundLocation>) -> Result<Vec<(UnboundLocation, EdgeType)>> {
    let first = points
        .first()
        .context("must have at least 2 element for make_path")?
        .clone();
    let last = points
        .last()
        .context("must have at least 2 element for make_path")?
        .clone();

    if first == last {
        return Err(anyhow!("first == last we need 2 elements"));
    }

    points.push(
        points
            .first()
            .context("must have at least 1 element for make_path")?
            .clone(),
    );

    let mut prev_direction = last
        .direction_between(&first)
        .context("failed to get initial direction")?;
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for slice in points.windows(2) {
        let mut current = slice[0].clone();
        let end = slice[1].clone();
        let dir = current
            .direction_between(&end)
            .context("expected to find route between path points")?;

        while current != end {
            if !seen.contains(&current) {
                let edge_type = EdgeType::from_directions(&prev_direction, &dir);
                out.push((current.clone(), edge_type));
                seen.insert(current.clone());
            }
            current = current.go_direction(&dir, 1).clone();
            prev_direction = dir.clone();
        }
    }

    Ok(out)
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
        if map.get(&Location(zero, end_1)) == &Tile::Outer {
            return false;
        }
    }
    for one in start_1..=end_1 {
        if map.get(&Location(start_0, one)) == &Tile::Outer {
            return false;
        }
        if map.get(&Location(end_0, one)) == &Tile::Outer {
            return false;
        }
    }
    true
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let raw: Vec<(i64, i64)> = parse_input(
        LineSplitter,
        ParseTuple2(ParseFromStr, ParseFromStr, ","),
        input,
    )
    .context("failed to parse input")?;

    let points: Vec<UnboundLocation> = raw.iter().map(|(x, y)| UnboundLocation(*y, *x)).collect();

    let max_x: usize = raw
        .iter()
        .map(|(x, _)| *x)
        .max()
        .context("must find max x")?
        .try_into()
        .context("expected max_x to be positive")?;
    let max_y: usize = raw
        .iter()
        .map(|(_, y)| *y)
        .max()
        .context("must find max y")?
        .try_into()
        .context("expected max_y to be positive")?;

    // Adding +1 ensures that we have an outer spot to start from at the bottom right corner
    let mut map = Map::<Tile>::from_dimensions(max_y + 2, max_x + 2, |_| Tile::Empty);

    let mut path = Vec::new();
    for (point, edge) in make_path(points.clone()).context("failed to make path")? {
        let loc = point
            .to_bounded(&map)
            .context("expected point to be inside map")?;

        *map.get_mut(&loc) = Tile::Edge(edge);
        path.push(loc);
    }

    println!("pre-fill");
    fill(&mut map).context("failed to fill")?;
    println!("filled!");
    // map.print(|v, _| match v {
    //     Tile::Empty => '.',
    //     Tile::Inner => 'I',
    //     Tile::Outer => 'o',
    //     Tile::Edge(e) => e.to_char(),
    // });

    let mut max_area: Option<usize> = None;
    let mut bound_points = Vec::new();
    for point in points {
        bound_points.push(
            point
                .to_bounded(&map)
                .context("expected point to be inside map")?,
        )
    }
    bound_points.sort();
    for a in bound_points.iter() {
        for b in bound_points.iter() {
            if a <= b {
                break;
            }
            let area = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
            if area <= max_area.unwrap_or(0) {
                continue;
            }
            if valid_path(&map, a, b) {
                max_area = Some(match max_area {
                    Some(existing) => {
                        if area >= existing {
                            area
                        } else {
                            existing
                        }
                    }
                    None => area,
                });
            }
        }
    }
    println!("max_area={max_area:?}");

    Ok(Some(max_area.context("failed to find last element")?))
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

    #[test]
    fn test_part_two_2() -> anyhow::Result<()> {
        let expected = Some(1220);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 3);
        assert!(expected.is_none() || !input.is_empty(), "example 3 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
