fn inner(patterns: &[&str], line: &str, index: usize, cache: &mut [Option<usize>]) -> usize {
    if let Some(count) = cache[index] {
        return count;
    }

    let mut possibilities = 0;

    for pattern in patterns {
        // TODO: line[index..index+pattern.len()] != pattern
        if !line[index..].starts_with(pattern) {
            continue;
        }

        if line.len() == pattern.len() + index {
            possibilities += 1;
            continue;
        }

        possibilities += inner(patterns, line, index + pattern.len(), cache);
    }

    cache[index] = Some(possibilities);

    possibilities
}

pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();

    let patterns: Vec<&str> = lines.next().unwrap().split(", ").collect();

    lines.next();

    lines
        .filter(|line| {
            if line.is_empty() {
                return false;
            }

            inner(&patterns, line, 0, &mut [None; 1000]) > 0
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();

    let patterns: Vec<&str> = lines.next().unwrap().split(", ").collect();

    lines.next();

    lines
        // .take(1)
        .map(|line| {
            if line.is_empty() {
                return 0;
            }

            inner(&patterns, line, 0, &mut [None; 1000])
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../examples/2024/day19.txt");

    #[test]
    fn test_part1() {
        assert_eq!(6, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(16, part2(TEST_INPUT));
    }
}