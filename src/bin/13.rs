use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<i64> {
    process_input(input, None).into()
}

pub fn part_two(input: &str) -> Option<i64> {
    process_input(input, Some(10_000_000_000_000)).into()
}

fn process_input(input: &str, prize_adjustment: Option<i64>) -> i64 {
    input
        .split("\n\n")
        .filter_map(|chunk| {
            let mut machine = chunk.parse::<ClawMachine>().ok()?;

            if let Some(adjustment) = prize_adjustment {
                machine.prize.0 += adjustment;
                machine.prize.1 += adjustment;
            }

            solve_min_cost(machine)
        })
        .sum()
}

fn solve_min_cost(
    ClawMachine {
        button_a: (a_dx, a_dy),
        button_b: (b_dx, b_dy),
        prize: (prize_x, prize_y),
    }: ClawMachine,
) -> Option<i64> {
    let determinant = a_dx * b_dy - b_dx * a_dy;
    if determinant == 0 {
        return None;
    }

    let (x_num, y_num) = (
        b_dy * prize_x - b_dx * prize_y,
        a_dx * prize_y - a_dy * prize_x,
    );

    if x_num % determinant != 0 || y_num % determinant != 0 {
        return None;
    }

    let (x, y) = (x_num / determinant, y_num / determinant);

    if x < 0 || y < 0 {
        return None;
    }

    Some(3 * x + y)
}

struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl FromStr for ClawMachine {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut lines = input.lines();

        let parse_coords = |line: &str| -> Result<(i64, i64)> {
            let parts = line.split(['=', '+', ',']).collect_vec();
            if parts.len() != 4 {
                bail!("Invalid format");
            }

            let dx = parts[1].trim().parse::<i64>().context("Invalid X")?;
            let dy = parts[3].trim().parse::<i64>().context("Invalid Y")?;

            Ok((dx, dy))
        };

        let mut parse_line = |label: &str| {
            lines
                .next()
                .ok_or_else(|| anyhow!("Missing line for {label}"))
                .and_then(parse_coords)
        };

        let machine = ClawMachine {
            button_a: parse_line("Button A")?,
            button_b: parse_line("Button B")?,
            prize: parse_line("Prize")?,
        };

        if lines.next().is_some() {
            bail!("Too many lines in input");
        }

        Ok(machine)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
