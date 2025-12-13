advent_of_code::solution!(7);

use advent_of_code::template::RunType;
use aoc_lib::grid::{CountingMap, Direction, Location, Map};

use anyhow::{Context, Result, anyhow};

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Start,
    Empty,
    Splitter,
}

impl Tile {}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(other: char) -> Result<Self> {
        Ok(match other {
            'S' => Tile::Start,
            '.' => Tile::Empty,
            '^' => Tile::Splitter,
            other => {
                return Err(anyhow!("found unexpected tile value {:?}", other));
            }
        })
    }
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    let map: Map<Tile> = Map::try_from(input).context("Failed to parse input")?;
    let start = map
        .find(|(_, value)| *value == &Tile::Start)
        .context("expected to find start tile")?;

    let mut seen = CountingMap::from(&map);
    let mut to_process = vec![start];

    let mut out = 0;
    while let Some(current) = to_process.pop() {
        if seen.get(&current) {
            continue;
        }
        seen.mark(&current);

        if map.get(&current) == &Tile::Splitter {
            out += 1;
            if let Some(next) = map.go_direction(&current, &Direction::East) {
                to_process.push(next);
            }
            if let Some(next) = map.go_direction(&current, &Direction::West) {
                to_process.push(next);
            }
        } else if let Some(next) = map.go_direction(&current, &Direction::South) {
            to_process.push(next);
        }
    }

    Ok(Some(out))
}

fn cached_go_path(map: &Map<Tile>, mut current: Location, cache: &mut Map<Option<u64>>) -> u64 {
    if let Some(cached) = cache.get(&current) {
        return *cached;
    }

    let count = if map.get(&current) == &Tile::Splitter {
        let mut count = 0;
        if let Some(next) = map.go_direction(&current, &Direction::East) {
            count += cached_go_path(map, next, cache);
        }
        if let Some(next) = map.go_direction(&current, &Direction::West) {
            count += cached_go_path(map, next, cache);
        }
        count
    } else {
        loop {
            if let Some(next) = map.go_direction(&current, &Direction::South) {
                if map.get(&next) == &Tile::Splitter {
                    let count = cached_go_path(map, next, cache);
                    break count;
                } else {
                    current = next;
                }
            } else {
                break 1;
            }
        }
    };

    *(cache.get_mut(&current)) = Some(count);
    count
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    let map: Map<Tile> = Map::try_from(input).context("Failed to parse input")?;
    let start = map
        .find(|(_, value)| *value == &Tile::Start)
        .context("expected to find start tile")?;

    let mut cache = map.transform(|_, _| None);
    Ok(Some(cached_go_path(&map, start, &mut cache)))
}

#[cfg(test)]
mod tests_day_7 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(21);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(40);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
