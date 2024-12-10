use itertools::*;
use std::cmp::Ordering;
use std::ops::RangeInclusive;

type Output = usize;

pub fn input_generator(input: &str) -> impl Iterator<Item = Vec<u32>> + use<'_> {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|item| item.parse().unwrap())
            .collect::<Vec<_>>()
    })
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

pub fn part1(input: &str) -> Output {
    input_generator(input)
        .filter(|report| is_valid_report(report))
        .count()
}

pub fn part2(input: &str) -> Output {
    let mut workhorse = Vec::with_capacity(5);

    input_generator(input)
        .filter(|report| {
            if is_valid_report(report) {
                return true;
            }

            workhorse.clear();
            workhorse.extend_from_slice(report);

            let mut removed_element = workhorse.pop().unwrap();

            if is_valid_report(&workhorse) {
                return true;
            }

            for index in (0..workhorse.len()).rev() {
                let temp = workhorse[index];
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
        assert_eq!(2, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, part2(TEST_INPUT));
    }
}
