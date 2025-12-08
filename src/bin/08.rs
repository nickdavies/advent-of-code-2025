#![allow(unused_imports)]
advent_of_code::solution!(8);

use advent_of_code::template::RunType;
use aoc_lib::parse::preamble::*;
use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result, anyhow};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    // Not the actual distance but avoids the sqrt
    fn dist_relative(&self, other: &Self) -> u64 {
        let mut out = 0;
        out += other.x.abs_diff(self.x).pow(2);
        out += other.y.abs_diff(self.y).pow(2);
        out += other.z.abs_diff(self.z).pow(2);

        out
    }

    fn dist(&self, other: &Self) -> u64 {
        self.dist_relative(other).isqrt()
    }
}

pub fn part_one(input: &str, run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let max_connections = match run_type {
        RunType::Example => 10,
        RunType::Real => 1000,
    };
    let raw: Vec<(u64, u64, u64)> = parse_input(
        LineSplitter,
        ParseTuple3(ParseFromStr, ParseFromStr, ParseFromStr, ","),
        input,
    )
    .context("failed to parse input")?;

    let mut points: Vec<Point> = raw.into_iter().map(|(x, y, z)| Point { x, y, z }).collect();
    points.sort();

    // println!("points={points:?}");

    let mut dists = Vec::new();
    for a in points.iter() {
        for b in points.iter() {
            if a <= b {
                break;
            }
            dists.push((b, a, a.dist_relative(b)));
        }
    }

    dists.sort_by_key(|(_, _, d)| *d);
    let dists_count = dists.len();
    println!("dists={dists_count:?}");

    let mut connections = 0;
    let mut next_id = 0;
    let mut circuits = BTreeMap::new();
    let mut point_to_circuit: BTreeMap<Point, usize> = BTreeMap::new();
    for (a, b, dist) in dists {
        connections += 1;
        if connections > max_connections {
            break;
        }
        println!("a={a:?}, b={b:?}, dist={dist}, connections={connections}");

        match (point_to_circuit.get(a), point_to_circuit.get(b)) {
            // New circuit!
            (None, None) => {
                println!("new circuit {next_id} for: {a:?}, {b:?}");
                point_to_circuit.insert(a.clone(), next_id);
                point_to_circuit.insert(b.clone(), next_id);

                let mut circuit = BTreeSet::new();
                circuit.insert(a.clone());
                circuit.insert(b.clone());
                circuits.insert(next_id, circuit);
                next_id += 1;
            }
            // One missing
            (Some(idx), None) => {
                println!("add {b:?} to circuit: {idx}");
                circuits.get_mut(idx).unwrap().insert(b.clone());
                point_to_circuit.insert(b.clone(), *idx);
            }
            (None, Some(idx)) => {
                println!("add {a:?} to circuit: {idx}");
                circuits.get_mut(idx).unwrap().insert(a.clone());
                point_to_circuit.insert(a.clone(), *idx);
            }
            // Merge circuits
            (Some(a_idx), Some(b_idx)) => {
                if a_idx == b_idx {
                    println!("Points already in existing circuit {a_idx}");
                    continue;
                }
                println!("merging circuits: {a_idx} and {b_idx}");
                let mut c1 = circuits.remove(a_idx).unwrap();
                println!("{a_idx}: a={c1:?}");
                let mut c2 = circuits.remove(b_idx).unwrap();
                println!("{b_idx}: b={c2:?}");

                c1.append(&mut c2);
                println!("{next_id}: b={c1:?}");
                for p in c1.iter() {
                    point_to_circuit.insert(p.clone(), next_id);
                }
                circuits.insert(next_id, c1);
                next_id += 1;
            }
        }
    }

    let mut lengths: Vec<usize> = circuits.values().map(|points| points.len()).collect();
    lengths.sort();
    lengths.reverse();

    for (id, circuit) in circuits.iter() {
        println!("id={id}, circuit={circuit:?}");
    }
    println!("lengths={lengths:?}");

    Ok(Some(
        lengths.get(0).unwrap_or(&1) * lengths.get(1).unwrap_or(&1) * lengths.get(2).unwrap_or(&1),
    ))
}

pub fn part_two(_input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    Ok(None)
}

#[cfg(test)]
mod tests_day_8 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(40);
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
