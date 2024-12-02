use itertools::*;
use std::cmp::Ordering;
use std::ops::RangeInclusive;

type Output = usize;
type Input = Vec<Vec<u32>>;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

const ALLOWED_RANGE: RangeInclusive<u32> = 1..=3;

fn is_valid_report(report: &[u32]) -> bool {
    let a = report[0];
    let b = report[1];
    let expected_ordering = a.cmp(&b);

    if expected_ordering == Ordering::Equal {
        return false;
    }

    if !ALLOWED_RANGE.contains(&a.abs_diff(b)) {
        return false;
    }

    report.iter().skip(1).tuple_windows().all(|(a, b)| {
        if a.cmp(b) != expected_ordering {
            return false;
        }

        if !ALLOWED_RANGE.contains(&a.abs_diff(*b)) {
            return false;
        }

        true
    })
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Input) -> Output {
    input
        .iter()
        .filter(|report| is_valid_report(report))
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Input) -> Output {
    input
        .iter()
        .filter(|report| {
            if is_valid_report(report) {
                return true;
            }

            let mut workhorse = report.to_vec();
            let mut removed_element = workhorse.pop().unwrap();
            let mut temp = removed_element;

            if is_valid_report(&workhorse) {
                return true;
            }

            for index in (0..workhorse.len()).rev() {
                temp = workhorse[index];
                workhorse[index] = removed_element;
                removed_element = temp;

                if is_valid_report(&workhorse) {
                    return true;
                }
            }

            false
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../examples/2024/day2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(2, solve_part1(&input_generator(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, solve_part2(&input_generator(TEST_INPUT)));
    }
}
