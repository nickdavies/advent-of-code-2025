advent_of_code::solution!(4);

use advent_of_code::template::RunType;

use aoc_lib::grid::{Direction, Location, Map};

use anyhow::{Context, Result, anyhow};

#[derive(PartialEq, Debug)]
enum Tile {
    Empty,
    Paper,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(other: char) -> Result<Self> {
        Ok(match other {
            '.' => Self::Empty,
            '@' => Self::Paper,
            unknown => {
                return Err(anyhow!("Hot unexpected char '{}'", unknown));
            }
        })
    }
}

fn get_left_right(map: &Map<Tile>, loc: Option<Location>, mid: bool) -> usize {
    let mut count = 0;

    let loc = match loc {
        Some(loc) => loc,
        None => return 0,
    };

    if mid && map.get(&loc) == &Tile::Paper {
        count += 1;
    }
    if let Some(upleft) = map.go_direction(&loc, &Direction::West)
        && map.get(&upleft) == &Tile::Paper
    {
        count += 1;
    }
    if let Some(upright) = map.go_direction(&loc, &Direction::East)
        && map.get(&upright) == &Tile::Paper
    {
        count += 1;
    }
    count
}
pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let map: Map<Tile> = input.try_into().context("failed to parse map")?;

    let mut out = 0;
    for row in map.iter() {
        for (loc, value) in row {
            if value == &Tile::Empty {
                continue;
            }

            let mut count = 0;
            count += get_left_right(&map, Some(loc.clone()), false);
            count += get_left_right(&map, map.go_direction(&loc, &Direction::North), true);
            count += get_left_right(&map, map.go_direction(&loc, &Direction::South), true);
            if count < 4 {
                out += 1;
            }
        }
    }

    Ok(Some(out))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let mut map: Map<Tile> = input.try_into().context("failed to parse map")?;

    let mut out = 0;
    let mut removed = true;
    while removed {
        let mut to_remove = Vec::new();
        for row in map.iter() {
            for (loc, value) in row {
                if value == &Tile::Empty {
                    continue;
                }

                let mut count = 0;
                count += get_left_right(&map, Some(loc.clone()), false);
                count += get_left_right(&map, map.go_direction(&loc, &Direction::North), true);
                count += get_left_right(&map, map.go_direction(&loc, &Direction::South), true);
                if count < 4 {
                    out += 1;
                    to_remove.push(loc.clone());
                }
            }
        }

        removed = !to_remove.is_empty();
        for loc in to_remove {
            *map.get_mut(&loc) = Tile::Empty;
        }
    }
    Ok(Some(out))
}

#[cfg(test)]
mod tests_day_4 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(13);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(43);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
