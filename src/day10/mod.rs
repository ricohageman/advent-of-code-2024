use itertools::*;
use std::collections::VecDeque;
use fixedbitset::FixedBitSet;
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
    inner: [u8; 1600],
    width: usize,
    height: usize,
}

static mut QUEUE: VecDeque<(usize, u8)> = VecDeque::new();
static mut TRAIL_ENDS: FixedBitSet = FixedBitSet::new();

impl Grid {
    fn from_input(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let mut height = 0;

        let mut inner: [u8; 1600] = [u8::MAX; 1600];

        input
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.is_empty())
            .for_each(|(y, line)| {
                height += 1;
                line.chars().enumerate().for_each(|(x, c)| {
                    inner[y * width + x] = c.to_digit(10).unwrap() as u8;
                })
            });

        unsafe {
            TRAIL_ENDS.grow(width * height);
            QUEUE.reserve(width * height);
        }

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

    unsafe fn find_trails_starting_at(&self, trail_head: usize, mut trail_end: impl FnMut(usize)) {
        QUEUE.clear();
        QUEUE.push_front((trail_head, 0));

        while let Some((index, height)) = QUEUE.pop_front() {
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

                QUEUE.push_front((next_index, next_height));
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    unsafe {
        let grid = Grid::from_input(input);

        grid.trail_heads()
            .map(|trail_head| {
                TRAIL_ENDS.clear();
                grid.find_trails_starting_at(trail_head, |trail_end| {
                    TRAIL_ENDS.insert(trail_end);
                });

                TRAIL_ENDS.count_ones(..)
            })
            .sum()
    }
}

pub fn part2(input: &str) -> usize {
    unsafe {
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
