use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Output = usize;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum XMAS {
    X,
    M,
    A,
    S,
}

impl XMAS {
    fn next(&self) -> Option<Self> {
        match self {
            XMAS::X => Some(XMAS::M),
            XMAS::M => Some(XMAS::A),
            XMAS::A => Some(XMAS::S),
            XMAS::S => None,
        }
    }
}

#[derive(EnumIter, Debug, Copy, Clone)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

#[derive(Debug)]
struct Grid {
    inner: Vec<XMAS>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let inner = input
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    'X' => XMAS::X,
                    'M' => XMAS::M,
                    'A' => XMAS::A,
                    'S' => XMAS::S,
                    _ => panic!("Unknown character '{}'", c),
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

    fn at(&self, index: usize) -> XMAS {
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
            Direction::UpRight => {
                self.move_towards(self.move_towards(index, Direction::Up)?, Direction::Right)
            }
            Direction::DownRight => {
                self.move_towards(self.move_towards(index, Direction::Down)?, Direction::Right)
            }
            Direction::DownLeft => {
                self.move_towards(self.move_towards(index, Direction::Down)?, Direction::Left)
            }
            Direction::UpLeft => {
                self.move_towards(self.move_towards(index, Direction::Up)?, Direction::Left)
            }
        }
    }
}

pub fn part1(input: &str) -> Output {
    let grid = Grid::from_input(input);

    grid.inner
        .iter()
        .enumerate()
        .filter(|(_, element)| **element == XMAS::X)
        .map(|(index, _)| {
            Direction::iter()
                .filter(|direction| {
                    let mut current_index = index;
                    let mut current_element = XMAS::X;

                    for _ in 0..3 {
                        let target_element = current_element.next().unwrap();
                        let Some(next_index) = grid.move_towards(current_index, *direction) else {
                            return false;
                        };

                        if grid.at(next_index) != target_element {
                            return false;
                        }

                        current_index = next_index;
                        current_element = target_element;
                    }

                    true
                })
                .count()
        })
        .sum()
}

pub fn part2(input: &str) -> Output {
    let grid = Grid::from_input(input);

    grid.inner
        .iter()
        .enumerate()
        .filter(|(_, element)| **element == XMAS::A)
        .filter_map(|(index, _)| {
            let top_left = grid.at(grid.move_towards(index, Direction::UpLeft)?);
            if top_left == XMAS::A || top_left == XMAS::X {
                return None;
            }

            let bottom_right = grid.at(grid.move_towards(index, Direction::DownRight)?);
            if bottom_right == XMAS::A || bottom_right == XMAS::X || bottom_right == top_left {
                return None;
            }

            let top_right = grid.at(grid.move_towards(index, Direction::UpRight)?);
            if top_right == XMAS::A || top_right == XMAS::X {
                return None;
            }

            let bottom_left = grid.at(grid.move_towards(index, Direction::DownLeft)?);
            if bottom_left == XMAS::A || bottom_left == XMAS::X || bottom_left == top_right {
                return None;
            }

            Some(())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../examples/2024/day4.txt");

    #[test]
    fn test_part1() {
        assert_eq!(18, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(9, part2(TEST_INPUT));
    }
}
