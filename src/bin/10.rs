advent_of_code::solution!(10);

use advent_of_code::template::RunType;

use microlp::{ComparisonOp, OptimizationDirection, Problem};

use anyhow::{Context, Result, anyhow};
use aoc_lib::parse::Parser;
use aoc_lib::parse::preamble::*;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;

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

    let mut out = 0;
    for machine in machines {
        let combo = machine
            .press_light_combinations(20)
            .context("failed to find combo")?;

        out += -combo.cost;
    }

    Ok(Some(out))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<i64>, anyhow::Error> {
    let machines: Vec<Machine> =
        parse_input(LineSplitter, ParseFromStr, input).context("failed to parse input")?;

    let max_cost = 1000;

    let mut out = 0;
    for machine in machines.iter() {
        let mut problem = Problem::new(OptimizationDirection::Minimize);

        // We need 1 variable per button we can possibly press
        let mut press_count_vars = Vec::new();
        for _ in machine.joltage_buttons.iter() {
            press_count_vars.push(problem.add_integer_var(1.0, (0, max_cost)));
        }

        // We then constrain the sum of the button presses (times if they are 1 or 0) to be less
        // than the target joltage.
        for (idx, target_joltage) in machine.joltages.iter().enumerate() {
            let mut constraints = Vec::new();
            for (buttons, &var) in machine.joltage_buttons.iter().zip(press_count_vars.iter()) {
                constraints.push((var, buttons.buttons[idx] as f64));
            }
            problem.add_constraint(constraints, ComparisonOp::Eq, *target_joltage as f64);
        }

        let solution = problem.solve().context("failed to solve problem")?;
        // Because the solver is all in floats we round back to an int.
        out += solution.objective().round() as usize;
    }

    Ok(Some(out as i64))
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
