use itertools::*;
use rustc_hash::FxHashSet;
use std::collections::HashMap;

fn parse_input(input: &str) -> (isize, isize, HashMap<char, Vec<(isize, isize)>>) {
    let mut width: isize = 0;
    let mut height: isize = 0;

    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            height += 1;

            width = line.len() as isize;

            line.char_indices()
                .filter(|(_, element)| *element != '.')
                .map(move |(x, element)| (element, (x as isize, y as isize)))
        })
        .into_group_map();

    (width, height, antennas)
}

pub fn part1(input: &str) -> usize {
    let (width, height, antennas) = parse_input(input);

    let mut antinodes: FxHashSet<(isize, isize)> = FxHashSet::default();

    antennas.values().for_each(|group| {
        group
            .iter()
            .tuple_combinations()
            .for_each(|((x1, y1), (x2, y2))| {
                let dx = x1 - x2;
                let dy = y1 - y2;

                let n1 = (x1 + dx, y1 + dy);
                let n2 = (x2 - dx, y2 - dy);

                if (0..width).contains(&n1.0) && (0..height).contains(&n1.1) {
                    antinodes.insert(n1);
                }

                if (0..width).contains(&n2.0) && (0..height).contains(&n2.1) {
                    antinodes.insert(n2);
                }
            })
    });

    antinodes.len()
}

pub fn part2(input: &str) -> usize {
    let (width, height, antennas) = parse_input(input);

    let mut antinodes: FxHashSet<(isize, isize)> = FxHashSet::default();

    antennas.values().for_each(|group| {
        if group.len() <= 1 {
            return;
        }

        antinodes.extend(group);

        group
            .iter()
            .tuple_combinations()
            .for_each(|((x1, y1), (x2, y2))| {
                let dx = x1 - x2;
                let dy = y1 - y2;

                let mut nx = x1 + dx;
                let mut ny = y1 + dy;

                while (0..width).contains(&nx) && (0..height).contains(&ny) {
                    antinodes.insert((nx, ny));
                    nx += dx;
                    ny += dy;
                }

                let mut nx = x1 - dx;
                let mut ny = y1 - dy;

                while (0..width).contains(&nx) && (0..height).contains(&ny) {
                    nx -= dx;
                    ny -= dy;
                }

                nx = x2 - dx;
                ny = y2 - dy;

                while (0..width).contains(&nx) && (0..height).contains(&ny) {
                    antinodes.insert((nx, ny));
                    nx -= dx;
                    ny -= dy;
                }

                nx = x2 + dx;
                ny = y2 + dy;

                while (0..width).contains(&nx) && (0..height).contains(&ny) {
                    antinodes.insert((nx, ny));
                    nx += dx;
                    ny += dy;
                }
            })
    });

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../examples/2024/day8.txt");
    const TEST_INPUT_PART_2_EXAMPLE: &str = include_str!("../../examples/2024/day8_part_2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(14, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(9, part2(TEST_INPUT_PART_2_EXAMPLE));
        assert_eq!(34, part2(TEST_INPUT));
    }
}
