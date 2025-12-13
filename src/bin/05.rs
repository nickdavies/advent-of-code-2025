advent_of_code::solution!(5);

use advent_of_code::template::RunType;
use anyhow::{Context, Result};
use aoc_lib::parse::preamble::*;

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let (sec1, sec2) = input.split_once("\n\n").context("failed to split input")?;

    let mut ranges: Vec<(u64, u64)> = parse_input(
        LineSplitter,
        ParseTuple2(ParseFromStr, ParseFromStr, "-"),
        sec1,
    )
    .context("failed to parse input")?;

    let ids: Vec<u64> =
        parse_input(LineSplitter, ParseFromStr, sec2).context("failed to parse input")?;

    ranges.sort();

    let mut out = 0;
    for id in ids {
        for (start, end) in &ranges {
            if id >= *start && id <= *end {
                out += 1;
                break;
            }
        }
    }

    Ok(Some(out))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    let (sec1, _) = input.split_once("\n\n").context("failed to split input")?;

    let mut ranges: Vec<(u64, u64)> = parse_input(
        LineSplitter,
        ParseTuple2(ParseFromStr, ParseFromStr, "-"),
        sec1,
    )
    .context("failed to parse input")?;
    ranges.sort();

    let mut out = 0;
    let mut current = None;
    for (start, end) in ranges {
        if let Some((current_start, current_end)) = &mut current {
            if start <= *current_end {
                *current_end = std::cmp::max(end, *current_end);
            } else {
                let range_size = *current_end - *current_start + 1;
                out += range_size;
                current = Some((start, end));
            }
        } else {
            current = Some((start, end));
        }
    }

    if let Some((current_start, current_end)) = current {
        out += current_end - current_start + 1;
    }

    Ok(Some(out))
}

#[cfg(test)]
mod tests_day_5 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(3);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(14);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two_2() -> anyhow::Result<()> {
        let expected = Some(14);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 3);
        assert!(expected.is_none() || !input.is_empty(), "example 3 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
