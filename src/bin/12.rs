#![allow(unused_imports)]
advent_of_code::solution!(12);

use advent_of_code::template::RunType;

use aoc_lib::grid::{Direction, Location, Map};
use aoc_lib::parse::preamble::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use anyhow::{Context, Result, anyhow};

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
        let (_, char_grid) = section
            .split_once(":\n")
            .context("failed to split present")?;
        let total_area = char_grid.len();
        let count = char_grid.chars().filter(|c| *c == '#').count();
        presents.push((count, total_area));
    }

    let mut out = 0;
    for ((x, y), counts) in ranges {
        let total = x * y;
        let mut min_needed = 0;
        let mut max_needed = 0;
        for (count, (present_count, total_area)) in counts.iter().zip(presents.iter()) {
            min_needed += count * present_count;
            max_needed += total_area;
            if min_needed > total {
                break;
            }
        }
        // If the total present area is > total available then it doesn't matter
        // how we arrange things it won't fit.
        if min_needed > total {
            continue;
        // If we place gifts without any overlap at all will it fit?
        } else if max_needed < total {
            out += 1;
        } else {
            return Err(anyhow!(
                "found ambigious range: total={total}, min_needed={min_needed}, max_needed={max_needed}, {counts:?}"
            ));
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
        let expected = Some(528);
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
}
