advent_of_code::solution!(11);

use advent_of_code::template::RunType;

use aoc_lib::parse::preamble::*;
use std::collections::BTreeMap;

use anyhow::{Context, Result, anyhow};

fn count_paths_inner(
    graph: &BTreeMap<String, Vec<String>>,
    current: String,
    target: String,
    cache: &mut BTreeMap<String, usize>,
) -> usize {
    if current == target {
        return 1;
    }

    if let Some(cached) = cache.get(&current) {
        return *cached;
    }
    let mut out = 0;
    if let Some(options) = graph.get(&current) {
        for option in options {
            out += count_paths_inner(graph, option.clone(), target.clone(), cache);
        }
    }

    cache.insert(current, out);
    out
}

fn count_paths(graph: &BTreeMap<String, Vec<String>>, start: &str, target: &str) -> usize {
    count_paths_inner(
        graph,
        start.to_string(),
        target.to_string(),
        &mut BTreeMap::new(),
    )
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let raw: Vec<(String, Vec<String>)> = parse_input(
        LineSplitter,
        ParseTuple2(ParseFromStr, Trim(SplitDelim(ParseFromStr, " ")), ":"),
        input,
    )
    .context("failed to parse input")?;

    let graph: BTreeMap<String, Vec<String>> = raw.clone().into_iter().collect();
    let count = count_paths(&graph, "you", "out");

    Ok(Some(count))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let raw: Vec<(String, Vec<String>)> = parse_input(
        LineSplitter,
        ParseTuple2(ParseFromStr, Trim(SplitDelim(ParseFromStr, " ")), ":"),
        input,
    )
    .context("failed to parse input")?;

    let graph: BTreeMap<String, Vec<String>> = raw.clone().into_iter().collect();

    let special = ("dac", "fft");
    for (s1, s2) in [(special.0, special.1), (special.1, special.0)] {
        let between = count_paths(&graph, s1, s2);
        if between != 0 {
            let start = count_paths(&graph, "svr", s1);
            let end = count_paths(&graph, s2, "out");
            return Ok(Some(start * between * end));
        }
    }
    Err(anyhow!("didn't find zero-route ordering between s1 and s2"))
}

#[cfg(test)]
mod tests_day_11 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(5);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(2);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
