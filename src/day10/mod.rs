use arrayvec::ArrayVec;
use fixedbitset::FixedBitSet;
use itertools::*;
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
    inner: [u8; 1600],
    trail_heads: ArrayVec<u16, 320>,
    width: usize,
    height: usize,
}

struct GridFinder {
    queue: VecDeque<(usize, u8)>,
}

impl GridFinder {
    pub fn new(grid: &Grid) -> Self {
        Self {
            queue: VecDeque::with_capacity(grid.width * grid.height),
        }
    }

    fn find_trails_starting_at(
        &mut self,
        grid: &Grid,
        trail_head: u16,
        mut trail_end: impl FnMut(usize),
    ) {
        self.queue.clear();
        self.queue.push_front((trail_head as usize, 0));

        while let Some((index, height)) = self.queue.pop_front() {
            if height == 9 {
                trail_end(index);
                continue;
            }

            for direction in Direction::iter() {
                let Some(next_index) = grid.move_towards(index, direction) else {
                    continue;
                };

                let next_height = grid.at(next_index);

                if next_height != height + 1 {
                    continue;
                }

                self.queue.push_front((next_index, next_height));
            }
        }
    }
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let mut height = 0;

        let mut inner = [u8::MAX; 1600];
        let mut trail_heads = ArrayVec::new();

        input
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.is_empty())
            .for_each(|(y, line)| {
                height += 1;
                line.chars().enumerate().for_each(|(x, c)| {
                    let value = c.to_digit(10).unwrap() as u8;
                    let index = y * width + x;

                    inner[index] = value;

                    if value == 0 {
                        trail_heads.push(index as u16);
                    }
                })
            });

        Self {
            inner,
            width,
            height,
            trail_heads,
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
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::from_input(input);
    let mut finder = GridFinder::new(&grid);
    let mut trail_ends = FixedBitSet::with_capacity(grid.width * grid.height);

    grid.trail_heads
        .iter()
        .map(|trail_head| {
            trail_ends.clear();
            finder.find_trails_starting_at(&grid, *trail_head, |trail_end| {
                trail_ends.insert(trail_end);
            });

            trail_ends.count_ones(..)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let grid = Grid::from_input(input);
    let mut finder = GridFinder::new(&grid);

    grid.trail_heads
        .iter()
        .map(|trail_head| {
            let mut rating = 0;

            finder.find_trails_starting_at(&grid, *trail_head, |_| {
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
