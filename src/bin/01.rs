advent_of_code::solution!(1);

use anyhow::{Context, Result, anyhow};

use advent_of_code::template::RunType;

pub fn parse_input(input: &str) -> Result<Vec<i32>> {
    let mut out = Vec::new();
    for line in input.lines() {
        let (lr, n_str) = line.split_at(1);

        let n: i32 = n_str.parse().context("failed to parse input int")?;
        if lr == "L" {
            out.push(-n);
        } else if lr == "R" {
            out.push(n);
        } else {
            return Err(anyhow!("got unexpected l/r value: {lr}"));
        }
    }

    Ok(out)
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let mut out = 0;
    let mut current = 50;
    for n in parse_input(input).context("failed to parse input")? {
        current = (current + n) % 100;
        if current == 0 {
            out += 1;
        }
    }
    Ok(Some(out))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let mut out: u32 = 0;
    let mut current = 50;
    for n in parse_input(input).context("failed to parse input")? {
        let start = current;
        let full_rotations: i32 = if n == 0 { 0 } else { n / 100 };

        let n_single = n - 100 * full_rotations;
        let current_no_wrap = current + n_single;
        current = (current + n_single) % 100;
        if current < 0 {
            current += 100;
        }

        let mut delta: u32 = 0;
        delta += full_rotations.unsigned_abs();
        if current == 0 || (start != 0 && current != current_no_wrap) {
            delta += 1;
        }
        out += delta;
    }
    Ok(Some(out))
}

#[cfg(test)]
mod tests_day_1 {
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
        let expected = Some(6);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two_custom_1() -> anyhow::Result<()> {
        let expected = Some(26);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 3);
        assert!(expected.is_none() || !input.is_empty(), "example 3 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
