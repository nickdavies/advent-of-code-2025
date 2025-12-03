#![allow(unused_imports)]
advent_of_code::solution!(3);

use advent_of_code::template::RunType;

use anyhow::{Context, Result, anyhow};
use aoc_lib::grid::{Location, Map};
use aoc_lib::parse::preamble::*;
use std::collections::BTreeSet;

fn cached_seek(
    input: &[u64],
    position: usize,
    remaining: usize,
    cache: &mut Map<Option<u64>>,
) -> u64 {
    if remaining == 0 || position + remaining > input.len() {
        return 0;
    }
    let cache_key = Location(position, remaining);

    if let Some(cached) = cache.get(&cache_key) {
        return *cached;
    }

    // we either select this element or don't
    let selected = input[position]
        * 10_u64.pow((remaining - 1).try_into().expect("this will always fit"))
        + cached_seek(input, position + 1, remaining - 1, cache);

    let best = if input[position] == 9 {
        selected
    } else {
        std::cmp::max(selected, cached_seek(input, position + 1, remaining, cache))
    };
    *cache.get_mut(&cache_key) = Some(best);

    best
}

fn run(input: &str, digits: usize) -> Result<Option<u64>, anyhow::Error> {
    let data: Vec<Vec<u64>> = parse_input(
        LineSplitter,
        Chars(ParseFn(|c: char| {
            c.to_digit(10).map(|d| d.into()).context("expected digit")
        })),
        input,
    )
    .context("failed to parse input")?;

    let mut out = 0;
    for row in data {
        let mut cache = Map::from_dimensions(row.len(), digits + 1, |_| None::<u64>);
        let best = cached_seek(&row, 0, digits, &mut cache);
        out += best;
    }

    Ok(Some(out))
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    run(input, 2)
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    run(input, 12)
}

#[cfg(test)]
mod tests_day_3 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(357);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(3121910778619);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
