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
struct LightButtons {
    buttons: Vec<bool>,
}

impl LightButtons {
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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct JoltageButtons {
    buttons: Vec<usize>,
}

impl JoltageButtons {
    fn apply_to(&self, mut other: Vec<usize>) -> Vec<usize> {
        for (idx, value) in self.buttons.iter().enumerate() {
            other[idx] += value;
        }
        other
    }
}

#[derive(Debug, Clone)]
struct Machine {
    lights: Vec<bool>,
    light_buttons: Vec<LightButtons>,

    joltages: Vec<usize>,
    joltage_buttons: Vec<JoltageButtons>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Item<T> {
    cost: i64,
    target: T,
    path: Vec<usize>,
}

impl Machine {
    fn press_light_combinations(&self, max_cost: usize) -> Result<Item<Vec<bool>>> {
        let mut next = BinaryHeap::new();
        let mut seen = BTreeSet::new();

        next.push(Item {
            cost: 0,
            target: vec![false; self.lights.len()],
            path: Vec::new(),
        });

        while let Some(item) = next.pop() {
            seen.insert(item.target.clone());
            if item.target == self.lights {
                return Ok(item);
            }

            if item.cost < -(max_cost as i64) {
                return Err(anyhow!("reached max cost!"));
            }

            let delta: Vec<usize> = self
                .lights
                .iter()
                .zip(item.target.iter())
                .enumerate()
                .filter_map(|(idx, (a, b))| if a != b { Some(idx) } else { None })
                .collect();

            for (i, buttons) in self.light_buttons.iter().enumerate() {
                if delta.iter().any(|idx| buttons.useful_for(*idx)) {
                    let mut new = item.clone();
                    new.cost -= 1;
                    new.target = buttons.apply_to(new.target);
                    if seen.contains(&new.target) {
                        continue;
                    }
                    new.path.push(i);

                    next.push(new);
                }
            }
        }

        Err(anyhow!("ran out of options!"))
    }

    fn press_joltage_combinations(&self, max_cost: usize) -> Result<Item<Vec<usize>>> {
        let mut next = BinaryHeap::new();
        let mut seen = BTreeSet::new();

        next.push(Item {
            cost: 0,
            target: vec![0; self.joltages.len()],
            path: Vec::new(),
        });

        while let Some(item) = next.pop() {
            seen.insert(item.target.clone());
            if item.target == self.joltages {
                return Ok(item);
            }

            if item.cost < -(max_cost as i64) {
                return Err(anyhow!("reached max cost!"));
            }

            let mut delta = Vec::new();
            let mut at_target = Vec::new();
            let mut invalid = false;
            for (idx, (a, b)) in self.joltages.iter().zip(item.target.iter()).enumerate() {
                if a == b {
                    at_target.push(idx);
                } else if b < a {
                    delta.push(idx);
                } else if b > a {
                    invalid = true;
                    break;
                }
            }
            if invalid {
                continue;
            }

            for (i, buttons) in self.joltage_buttons.iter().enumerate() {
                let any_helpful = delta.iter().any(|idx| buttons.buttons[*idx] != 0);
                let any_harmful = at_target.iter().any(|idx| buttons.buttons[*idx] != 0);

                if any_helpful && !any_harmful {
                    let mut new = item.clone();
                    new.cost -= 1;
                    new.target = buttons.apply_to(new.target);
                    if seen.contains(&new.target) {
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
        let (first, rest) = input
            .split_once(" ")
            .context("expected to be able to split off lights")?;
        let (rest, last) = rest
            .rsplit_once(" ")
            .context("expected to be able to split off joltages")?;

        let raw_lights: Vec<char> = StripPrefix(StripSuffix(Chars(Identity), "]"), "[")
            .parse_section(first)
            .context("failed to parse lights")?;
        let lights: Vec<bool> = raw_lights.into_iter().map(|c| c == '#').collect();

        let joltages: Vec<usize> =
            StripPrefix(StripSuffix(SplitDelim(ParseFromStr, ","), "}"), "{")
                .parse_section(last)
                .context("failed to parse joltages")?;

        let mut light_buttons = Vec::new();
        let mut joltage_buttons = Vec::new();
        for section in rest.split(" ") {
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

                    let mut new_light_buttons = vec![false; lights.len()];
                    let mut new_joltage_buttons = vec![0; joltages.len()];
                    for button in raw_buttons.iter() {
                        new_light_buttons[*button] = true;
                        new_joltage_buttons[*button] = 1;
                    }

                    light_buttons.push(LightButtons {
                        buttons: new_light_buttons,
                    });

                    new_joltage_buttons.resize(joltages.len(), 0);
                    joltage_buttons.push(JoltageButtons {
                        buttons: new_joltage_buttons,
                    });
                }
                _ => {
                    return Err(anyhow!("got unexpected section: {}", section));
                }
            }
        }

        Ok(Machine {
            lights,
            joltages,
            light_buttons,
            joltage_buttons,
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
            .press_light_combinations(20)
            .context("failed to find combo")?;

        out += -combo.cost;
        println!("combo={combo:?}");
    }

    Ok(Some(out))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<i64>, anyhow::Error> {
    let machines: Vec<Machine> =
        parse_input(LineSplitter, ParseFromStr, input).context("failed to parse input")?;

    let mut out = 0;
    for machine in machines {
        println!("machine={machine:?}");
        let combo = machine
            .press_joltage_combinations(20)
            .context("failed to find combo")?;

        out += -combo.cost;
        println!("combo={combo:?}");
    }

    Ok(Some(out))
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
