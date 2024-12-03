use regex::Regex;

type Output = usize;

fn parse_multiplication(input: &str) -> Output {
    let (left, right) = input
        .strip_prefix("mul(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .split_once(",")
        .unwrap();

    left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap()
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> Output {
    let regex = Regex::new(r"(mul\(\d*,\d*\))").unwrap();
    regex
        .captures_iter(input)
        .map(|capture| parse_multiplication(&capture[0]))
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> Output {
    let regex = Regex::new(r"(mul\(\d*,\d*\))|(do\(\))|(don\'t\(\))").unwrap();
    let (_, sum) = regex
        .captures_iter(input)
        .fold((true, 0), |(enabled, acc), capture| {
            let capture = &capture[0];

            if capture.starts_with("do(") {
                return (true, acc);
            }

            if capture.starts_with("don") {
                return (false, acc);
            }

            if !enabled {
                return (false, acc);
            }

            (true, acc + parse_multiplication(capture))
        });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            161,
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            48,
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
        );
    }
}
