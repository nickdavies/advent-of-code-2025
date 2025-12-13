advent_of_code::solution!(2);

use anyhow::{Context, Result, anyhow};
use aoc_lib::parse::preamble::*;
use std::collections::BTreeSet;

use advent_of_code::template::RunType;

pub fn make_pattern(n: u64, copies: u32) -> u64 {
    let mut out = 0;
    let digits = n.ilog10() + 1;
    let mult = 10_u64.pow(digits);
    for _ in 0..copies {
        out *= mult;
        out += n;
    }
    out
}

pub fn make_patterns_in_range(start: u64, end: u64, pattern_len: u32) -> Vec<u64> {
    let mut out = Vec::new();
    let mut last: u64;

    let start_digits = if start == 0 {
        0
    } else {
        start.ilog10() / pattern_len
    };
    let mut i: u64 = 10_u64.pow(start_digits);
    loop {
        last = make_pattern(i, pattern_len);
        if last > end {
            break;
        }
        i += 1;
        if last >= start {
            out.push(last);
        }
    }
    out
}

pub fn make_doubles(start: u64, end: u64) -> Vec<u64> {
    make_patterns_in_range(start, end, 2)
}

pub fn make_patterns(start: u64, end: u64, out: &mut BTreeSet<u64>) {
    let start_digits = if start == 0 { 0 } else { start.ilog10() } + 1;
    let end_digits = if end == 0 { 0 } else { end.ilog10() } + 1;

    for len in 2..=end_digits {
        if start_digits.is_multiple_of(len) || end_digits.is_multiple_of(len) {
            for pattern in make_patterns_in_range(start, end, len) {
                out.insert(pattern);
            }
        }
    }
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    let data: Vec<Vec<(u64, u64)>> = parse_input(
        LineSplitter,
        SplitDelim(ParseTuple2(ParseFromStr, ParseFromStr, "-"), ","),
        input,
    )
    .context("failed to parse input")?;

    if data.len() != 1 {
        return Err(anyhow!(
            "invalid input, found {} lines, expected 1",
            data.len()
        ));
    }
    let data = data.into_iter().next().unwrap();

    let mut out: u64 = 0;
    for (x, y) in data {
        let doubles = make_doubles(x, y);
        out += doubles.iter().sum::<u64>();
    }
    Ok(Some(out))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    let data: Vec<Vec<(u64, u64)>> = parse_input(
        LineSplitter,
        SplitDelim(ParseTuple2(ParseFromStr, ParseFromStr, "-"), ","),
        input,
    )
    .context("failed to parse input")?;

    if data.len() != 1 {
        return Err(anyhow!(
            "invalid input, found {} lines, expected 1",
            data.len()
        ));
    }
    let data = data.into_iter().next().unwrap();

    let mut out: BTreeSet<u64> = BTreeSet::new();
    for (x, y) in data {
        make_patterns(x, y, &mut out);
    }
    Ok(Some(out.iter().sum::<u64>()))
}

#[cfg(test)]
mod tests_day_2 {
    use super::*;

    #[test]
    fn test_make_doubles() -> anyhow::Result<()> {
        assert_eq!(make_doubles(0, 10), vec![]);
        assert_eq!(
            make_doubles(0, 100),
            vec![11, 22, 33, 44, 55, 66, 77, 88, 99]
        );
        assert_eq!(make_doubles(50, 100), vec![55, 66, 77, 88, 99]);
        assert_eq!(make_doubles(99, 100), vec![99]);
        assert_eq!(make_doubles(100, 100), vec![]);
        assert_eq!(
            make_doubles(0, 1000),
            vec![11, 22, 33, 44, 55, 66, 77, 88, 99]
        );
        assert_eq!(
            make_doubles(0, 10000),
            vec![
                11, 22, 33, 44, 55, 66, 77, 88, 99, 1010, 1111, 1212, 1313, 1414, 1515, 1616, 1717,
                1818, 1919, 2020, 2121, 2222, 2323, 2424, 2525, 2626, 2727, 2828, 2929, 3030, 3131,
                3232, 3333, 3434, 3535, 3636, 3737, 3838, 3939, 4040, 4141, 4242, 4343, 4444, 4545,
                4646, 4747, 4848, 4949, 5050, 5151, 5252, 5353, 5454, 5555, 5656, 5757, 5858, 5959,
                6060, 6161, 6262, 6363, 6464, 6565, 6666, 6767, 6868, 6969, 7070, 7171, 7272, 7373,
                7474, 7575, 7676, 7777, 7878, 7979, 8080, 8181, 8282, 8383, 8484, 8585, 8686, 8787,
                8888, 8989, 9090, 9191, 9292, 9393, 9494, 9595, 9696, 9797, 9898, 9999
            ]
        );
        assert_eq!(
            make_doubles(7777, 10000),
            vec![
                7777, 7878, 7979, 8080, 8181, 8282, 8383, 8484, 8585, 8686, 8787, 8888, 8989, 9090,
                9191, 9292, 9393, 9494, 9595, 9696, 9797, 9898, 9999
            ]
        );

        Ok(())
    }

    #[test]
    fn test_make_pattern() -> anyhow::Result<()> {
        assert_eq!(make_pattern(1, 3), 111);
        assert_eq!(make_pattern(12, 3), 121212);
        assert_eq!(make_pattern(123, 3), 123123123);

        Ok(())
    }

    #[test]
    fn test_make_patterns_in_range() -> anyhow::Result<()> {
        assert_eq!(make_patterns_in_range(50, 100, 2), vec![55, 66, 77, 88, 99]);
        assert_eq!(
            make_patterns_in_range(50, 1000, 2),
            vec![55, 66, 77, 88, 99]
        );
        assert_eq!(
            make_patterns_in_range(50, 1000, 3),
            vec![111, 222, 333, 444, 555, 666, 777, 888, 999]
        );
        assert_eq!(
            make_patterns_in_range(500, 1000, 3),
            vec![555, 666, 777, 888, 999]
        );

        Ok(())
    }

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(1227775554);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(4174379265);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
