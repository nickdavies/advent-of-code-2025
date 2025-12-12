#![allow(unused_imports)]
advent_of_code::solution!(12);

use advent_of_code::template::RunType;

use aoc_lib::grid::{Direction, Location, Map};
use aoc_lib::parse::preamble::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use anyhow::{Context, Result, anyhow};

fn rotate_map<T: Clone>(map: &Map<T>) -> Map<T> {
    let width = map.width().unwrap();
    map.transform(|loc, _| map.get(&Location(width - loc.1 - 1, loc.0)).clone())
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let (raw_presents, ranges) = input
        .rsplit_once("\n\n")
        .context("failed to split end region")?;

    let ranges: Vec<((usize, usize), Vec<usize>)> = parse_input(
        LineSplitter,
        ParseTuple2(
            ParseTuple2(ParseFromStr, ParseFromStr, "x"),
            Trim(SplitDelim(ParseFromStr, " ")),
            ":",
        ),
        ranges,
    )
    .context("failed to parse input")?;

    let mut presents = Vec::new();
    for section in raw_presents.split("\n\n") {
        let (id, char_grid) = section
            .split_once(":\n")
            .context("failed to split present")?;
        let north: Map<bool> = Map::parse(char_grid, |c| Ok::<bool, anyhow::Error>(c == '#'))
            .context("Failed to convert present to map")?;

        println!("id={id}");
        north.print(|v, _| if *v { '#' } else { '.' });

        let count = char_grid.chars().filter(|c| *c == '#').count();
        presents.push(count);
    }

    println!("{presents:?}");
    println!("{ranges:?}");
    let mut out = 0;
    for ((x, y), counts) in ranges {
        let total = x * y;
        let mut used = 0;
        for (count, present_count) in counts.iter().zip(presents.iter()) {
            used += count * present_count;
            if used > total {
                break;
            }
        }
        if used < total {
            out += 1;
        }
    }

    Ok(Some(out))
}

pub fn part_two(_input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    Ok(None)
}

#[cfg(test)]
mod tests_day_12 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(2);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = None;
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_rotate() -> anyhow::Result<()> {
        let north = "abc\nd..\nefg";
        let east = "eda\nf.b\ng.c";
        let south = "gfe\n..d\ncba";
        let west = "c.g\nb.f\nade";

        let map_north: Map<char> = Map::try_from(north)?;
        let map_east: Map<char> = Map::try_from(east)?;
        let map_south: Map<char> = Map::try_from(south)?;
        let map_west: Map<char> = Map::try_from(west)?;

        println!("\nNorth");
        print_map(&map_north);
        println!("\nEast");
        print_map(&map_east);
        println!("\nSouth");
        print_map(&map_south);
        println!("\nwest");
        print_map(&map_west);

        println!("\nrotated");
        print_map(&rotate_map(&map_north));
        assert_eq!(map_east.0, rotate_map(&map_north).0);

        assert_eq!(map_south.0, rotate_map(&map_east).0);
        assert_eq!(map_west.0, rotate_map(&map_south).0);
        assert_eq!(map_north.0, rotate_map(&map_west).0);
        Ok(())
    }

    fn print_map(map: &Map<char>) {
        map.print(|v, _| *v);
    }
}
