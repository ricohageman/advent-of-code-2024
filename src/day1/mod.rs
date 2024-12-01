use itertools::*;
use rustc_hash::{FxBuildHasher, FxHashMap};

type Output = u32;
type Input = (Vec<u32>, Vec<u32>);

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(first, second)| {
            (
                first.parse::<u32>().unwrap(),
                second.parse::<u32>().unwrap(),
            )
        })
        .unzip()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let (left, right) = input;

    left.into_iter()
        .sorted()
        .zip_eq(right.iter().sorted())
        .map(|(left, right)| left.abs_diff(*right))
        .sum::<Output>()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Input) -> Output {
    let (left, right) = input;

    let mut counts = FxHashMap::with_capacity_and_hasher(right.len(), FxBuildHasher);
    right.iter().for_each(|item| *counts.entry(item).or_default() += 1);

    left.iter()
        .map(|value| *counts.get(value).unwrap_or(&0) as u32 * value)
        .sum::<Output>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../examples/2024/day1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(11, solve_part1(&input_generator(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(31, solve_part2(&input_generator(TEST_INPUT)));
    }
}
