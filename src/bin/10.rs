#![allow(unused_imports)]
advent_of_code::solution!(10);

use advent_of_code::template::RunType;

use anyhow::{Context, Result, anyhow};
use aoc_lib::grid::{CountingMap, Direction, Location, Map};
use aoc_lib::parse::Parser;
use aoc_lib::parse::preamble::*;
use std::collections::BinaryHeap;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Buttons {
    buttons: Vec<bool>,
}

impl Buttons {
    fn apply_to(&self, mut other: Vec<bool>) -> Vec<bool> {
        for (idx, toggle) in self.buttons.iter().enumerate() {
            if *toggle {
                other[idx] = !other[idx];
            }
        }
        other
    }

    fn useful_for(&self, idx: usize) -> bool {
        self.buttons[idx]
    }
}

#[derive(Debug, Clone)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Buttons>,
    joltages: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Item {
    cost: i64,
    lights: Vec<bool>,
    path: Vec<usize>,
}

impl Machine {
    fn press_combinations(&self, max_cost: usize) -> Result<Item> {
        let mut next = BinaryHeap::new();
        let mut seen = BTreeSet::new();

        next.push(Item {
            cost: 0,
            lights: vec![false; self.lights.len()],
            path: Vec::new(),
        });

        while let Some(item) = next.pop() {
            seen.insert(item.lights.clone());
            if item.lights == self.lights {
                return Ok(item);
            }

            if item.cost < -(max_cost as i64) {
                return Err(anyhow!("reached max cost!"));
            }

            let delta: Vec<usize> = self
                .lights
                .iter()
                .zip(item.lights.iter())
                .enumerate()
                .filter_map(|(idx, (a, b))| if a != b { Some(idx) } else { None })
                .collect();

            for (i, buttons) in self.buttons.iter().enumerate() {
                if delta.iter().any(|idx| buttons.useful_for(*idx)) {
                    let mut new = item.clone();
                    new.cost -= 1;
                    new.lights = buttons.apply_to(new.lights);
                    if seen.contains(&new.lights) {
                        continue;
                    }
                    new.path.push(i);

                    next.push(new);
                }
            }
        }

        Err(anyhow!("ran out of options!"))
    }
}

impl std::str::FromStr for Machine {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.split(" ");

        let first: &str = iter.next().context("expected to find lights for line")?;
        let raw_lights: Vec<char> = StripPrefix(StripSuffix(Chars(Identity), "]"), "[")
            .parse_section(first)
            .context("failed to parse lights")?;
        let lights: Vec<bool> = raw_lights.into_iter().map(|c| c == '#').collect();

        let mut buttons = Vec::new();
        let mut joltages = Vec::new();
        for section in iter {
            match section
                .chars()
                .next()
                .context("expected non-empty section")?
            {
                '(' => {
                    let parser = StripPrefix(StripSuffix(SplitDelim(ParseFromStr, ","), ")"), "(");
                    let raw_buttons: Vec<usize> = parser
                        .parse_section(section)
                        .context("failed to parse buttons section")?;

                    let mut new_buttons = vec![false; lights.len()];
                    for button in raw_buttons {
                        new_buttons[button] = true;
                    }

                    buttons.push(Buttons {
                        buttons: new_buttons,
                    });
                }
                '{' => {
                    let parser = StripPrefix(StripSuffix(SplitDelim(ParseFromStr, ","), "}"), "{");
                    let raw_joltage: Vec<usize> = parser
                        .parse_section(section)
                        .context("failed to parse joltage section")?;
                    joltages.push(raw_joltage);
                }
                _ => {
                    return Err(anyhow!("got unexpected section: {}", section));
                }
            }
        }

        Ok(Machine {
            lights,
            buttons,
            joltages,
        })
    }
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<i64>, anyhow::Error> {
    let machines: Vec<Machine> =
        parse_input(LineSplitter, ParseFromStr, input).context("failed to parse input")?;

    println!("machines={machines:?}");

    let mut out = 0;
    for machine in machines {
        let combo = machine
            .press_combinations(20)
            .context("failed to find combo")?;

        out += -combo.cost;
        println!("combo={combo:?}");
    }

    Ok(Some(out))
}

pub fn part_two(_input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    Ok(None)
}

#[cfg(test)]
mod tests_day_10 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(7);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(33);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
