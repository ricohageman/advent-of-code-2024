use itertools::{Either, Itertools};

fn get_heights<'a>(lines: impl Iterator<Item = &'a str>) -> [u8; 5] {
    let mut element = [0; 5];

    lines
        .filter(|line| !line.is_empty())
        .skip(1)
        .for_each(|line| {
            line.char_indices()
                .filter(|(_, element)| *element == '#')
                .for_each(|(index, _)| element[index] += 1);
        });

    element
}

pub fn part1(input: &str) -> usize {
    let (locks, keys): (Vec<[u8; 5]>, Vec<[u8; 5]>) = input.split("\n\n").partition_map(|input| {
        let mut is_key = !input.starts_with("#");

        if is_key {
            Either::Right(get_heights(input.lines().rev()))
        } else {
            Either::Left(get_heights(input.lines()))
        }
    });

    locks
        .iter()
        .cartesian_product(keys)
        .filter(|(lock, key)| {
            lock[0] + key[0] <= 5
                && lock[1] + key[1] <= 5
                && lock[2] + key[2] <= 5
                && lock[3] + key[3] <= 5
                && lock[4] + key[4] <= 5
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../examples/2024/day25.txt");

    #[test]
    fn test_part1() {
        assert_eq!(3, part1(TEST_INPUT));
    }
}
