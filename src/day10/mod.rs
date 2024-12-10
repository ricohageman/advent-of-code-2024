use itertools::*;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Grid {
    inner: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let inner = input
            .lines()
            .filter(|line| !line.is_empty())
            .flat_map(|line| {
                line.chars().map(|c| {
                    if c == '.' {
                        return 10;
                    }

                    c.to_digit(10).unwrap() as u8
                })
            })
            .collect::<Vec<_>>();

        let width = input.lines().next().unwrap().len();
        let height = inner.len() / width;

        Self {
            inner,
            width,
            height,
        }
    }

    fn at(&self, index: usize) -> u8 {
        self.inner[index]
    }

    fn move_towards(&self, index: usize, direction: Direction) -> Option<usize> {
        match direction {
            Direction::Up => {
                if index < self.width {
                    return None;
                }

                Some(index - self.width)
            }
            Direction::Down => {
                if index >= (self.height - 1) * self.width {
                    return None;
                }

                Some(index + self.width)
            }
            Direction::Left => {
                if index.rem_euclid(self.width) == 0 {
                    return None;
                }

                Some(index - 1)
            }
            Direction::Right => {
                if index.rem_euclid(self.width) == self.width - 1 {
                    return None;
                }

                Some(index + 1)
            }
        }
    }

    fn trail_heads(&self) -> impl Iterator<Item = usize> + use<'_> {
        self.inner.iter().positions(|height| *height == 0)
    }

    fn find_trails_starting_at(&self, trail_head: usize, mut trail_end: impl FnMut(usize)) {
        let mut queue = VecDeque::new();
        queue.push_front((trail_head, 0));

        while let Some((index, height)) = queue.pop_front() {
            if height == 9 {
                trail_end(index);
                continue;
            }

            for direction in Direction::iter() {
                let Some(next_index) = self.move_towards(index, direction) else {
                    continue;
                };

                let next_height = self.at(next_index);

                if next_height != height + 1 {
                    continue;
                }

                queue.push_front((next_index, next_height));
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::from_input(input);

    grid.trail_heads()
        .map(|trail_head| {
            let mut trail_ends: FxHashSet<usize> = FxHashSet::default();

            grid.find_trails_starting_at(trail_head, |trail_end| {
                trail_ends.insert(trail_end);
            });

            trail_ends.len()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let grid = Grid::from_input(input);

    grid.trail_heads()
        .map(|trail_head| {
            let mut rating = 0;

            grid.find_trails_starting_at(trail_head, |_| {
                rating += 1;
            });

            rating
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../examples/2024/day10.txt");

    #[test]
    fn test_part1() {
        assert_eq!(36, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(81, part2(TEST_INPUT));
    }
}
