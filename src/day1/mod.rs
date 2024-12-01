use itertools::*;

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
pub fn solve_part_1(input: &Input) -> Output {
    let (left, right) = input;

    left.into_iter()
        .sorted()
        .zip_eq(right.into_iter().sorted())
        .map(|(left, right)| left.abs_diff(*right))
        .sum::<Output>()
}

#[aoc(day1, part2)]
pub fn solve_part_2(input: &Input) -> Output {
    let (left, right) = input;

    let frequencies = right.into_iter().counts();

    left.into_iter()
        .map(|value| *frequencies.get(value).unwrap_or(&0) as u32 * value)
        .sum::<Output>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_1() {
        assert_eq!(11, solve_part_1(&input_generator(TEST_INPUT)));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(31, solve_part_2(&input_generator(TEST_INPUT)));
    }
}
