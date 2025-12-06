#![allow(unused_imports)]
advent_of_code::solution!(6);

use advent_of_code::template::RunType;

use anyhow::{Context, Result, anyhow};
use aoc_lib::parse::preamble::*;
use itertools::{EitherOrBoth, Itertools};
use std::str::FromStr;

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

impl FromStr for Operator {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        match input {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Mul),
            other => Err(anyhow!("got unexpected operator {}", other)),
        }
    }
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<i64>, anyhow::Error> {
    let mut lines: Vec<&str> =
        parse_input(LineSplitter, Identity, input).context("failed to parse input")?;

    let operators = lines
        .pop()
        .context("required to have 1 operators row at the end")?;
    let operators: Vec<Operator> = operators
        .split_ascii_whitespace()
        .map(|o| o.parse())
        .collect::<Result<Vec<Operator>>>()
        .context("failed to parse operators")?;

    let mut table = Vec::new();
    for line in lines {
        let mut fields: Vec<i64> = Vec::new();
        for field in line.split_ascii_whitespace() {
            fields.push(field.parse().context("failed to parse input number")?);
        }

        table.push(fields);
    }

    let mut iters = Vec::new();
    for row in &table {
        iters.push(row.iter());
    }

    let mut out = 0;
    for op in &operators {
        let mut value = match op {
            Operator::Add => 0,
            Operator::Mul => 1,
        };
        for iter in &mut iters {
            let column_value = iter
                .next()
                .context("expected to get next column from iter")?;
            value = match op {
                Operator::Add => value + column_value,
                Operator::Mul => value * column_value,
            };
        }
        out += value;
    }

    Ok(Some(out))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    let mut lines: Vec<&str> =
        parse_input(LineSplitter, Identity, input).context("failed to parse input")?;

    let operators = lines
        .pop()
        .context("required to have 1 operators row at the end")?;

    let mut iters = Vec::new();
    for line in &lines {
        iters.push(line.chars().rev());
    }

    let mut out = 0;
    let mut col = Vec::new();
    for op_char in operators.chars().rev() {
        let mut value = 0;
        let mut empty = true;
        for iter in &mut iters {
            let column_char = iter
                .next()
                .context("expected to get next column from iter")?;
            if column_char == ' ' {
                continue;
            }
            empty = false;
            let digit: u64 = column_char
                .to_digit(10)
                .context("failed to convert digit to int")?
                .into();

            value *= 10;
            value += digit;
        }
        if empty {
            continue;
        }
        col.push(value);

        let op = match op_char {
            ' ' => continue,
            '+' => Operator::Add,
            '*' => Operator::Mul,
            other => {
                return Err(anyhow!("got unexpected operator char: {}", other));
            }
        };

        let mut total = match op {
            Operator::Add => 0,
            Operator::Mul => 1,
        };
        for column_value in col {
            total = match op {
                Operator::Add => total + column_value,
                Operator::Mul => total * column_value,
            };
        }
        out += total;
        col = Vec::new();
    }

    Ok(Some(out))
}

#[cfg(test)]
mod tests_day_6 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(4277556);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(3263827);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
