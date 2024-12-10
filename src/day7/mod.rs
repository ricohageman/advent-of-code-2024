use std::collections::VecDeque;

pub fn solve_with_operations(input: &str, allow_concatenation: bool) -> usize {
    let mut queue = VecDeque::new();

    input
        .lines()
        .filter_map(|line| {
            let (test, numbers) = line.split_once(": ").unwrap();
            let test = test.parse::<usize>().unwrap();
            let numbers = numbers
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            queue.clear();
            queue.push_front((numbers[0], 1));

            while let Some((total, index)) = queue.pop_back() {
                if index == numbers.len() {
                    if total == test {
                        return Some(total);
                    } else {
                        continue;
                    }
                }

                let next_number = numbers[index];
                let next = total + next_number;
                if next <= test {
                    queue.push_back((next, index + 1));
                }

                let next = total * next_number;
                if next <= test {
                    queue.push_back((next, index + 1));
                }

                if !allow_concatenation {
                    continue;
                }

                let next = total * 10_usize.pow(next_number.ilog10() + 1) + next_number;
                if next <= test {
                    queue.push_back((next, index + 1));
                }
            }

            None
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    solve_with_operations(input, false)
}

pub fn part2(input: &str) -> usize {
    solve_with_operations(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../examples/2024/day7.txt");

    #[test]
    fn test_part1() {
        assert_eq!(3749, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(11387, part2(TEST_INPUT));
    }
}
