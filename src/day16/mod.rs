use arrayvec::ArrayVec;
use fixedbitset::FixedBitSet;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn neighbours(&self) -> (Self, Self) {
        match self {
            Direction::Down | Direction::Up => (Direction::Left, Direction::Right),
            Direction::Left | Direction::Right => (Direction::Up, Direction::Down),
        }
    }

    fn index(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Left => 1,
            Direction::Down => 2,
            Direction::Right => 3,
        }
    }
}

fn move_towards(index: usize, direction: Direction, grid: &Grid) -> Option<usize> {
    match direction {
        Direction::Up => {
            if index < grid.width {
                return None;
            }

            Some(index - grid.width)
        }
        Direction::Down => {
            if index >= (grid.height - 1) * grid.width {
                return None;
            }

            Some(index + grid.width)
        }
        Direction::Left => {
            if index.rem_euclid(grid.width) == 0 {
                return None;
            }

            Some(index - 1)
        }
        Direction::Right => {
            if index.rem_euclid(grid.width) == grid.width - 1 {
                return None;
            }

            Some(index + 1)
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
struct Position {
    index: usize,
    direction: Direction,
}

#[derive(Eq, PartialEq)]
struct HeapState {
    position: Position,
    previous_position: Option<Position>,
    value: usize,
}

impl Ord for HeapState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .value
            .cmp(&self.value)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for HeapState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    inner: FixedBitSet,
    width: usize,
    height: usize,
    source: usize,
    target: usize,
}

impl Grid {
    fn size(&self) -> usize {
        self.width * self.height
    }

    fn from_input(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut source = 0;
        let mut target = 0;

        let mut inner = FixedBitSet::with_capacity(width * height);

        for (y, line) in input.lines().enumerate() {
            for (x, bit) in line.chars().enumerate() {
                let index = y * width + x;

                match bit {
                    '#' => continue,
                    '.' => inner.set(index, true),
                    'S' => {
                        source = index;
                        inner.set(index, true)
                    }
                    'E' => {
                        target = index;
                        inner.set(index, true)
                    }
                    _ => panic!("Unknown value '{}'", bit),
                }
            }
        }

        Self {
            inner,
            width,
            height,
            source,
            target,
        }
    }
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::from_input(input);

    let (best_value, _) = inner(&grid);

    best_value
}

pub fn part2(input: &str) -> usize {
    let grid = Grid::from_input(input);
    let (_, previous) = inner(&grid);

    let mut visited = FixedBitSet::with_capacity(grid.size());
    visited.set(grid.target, true);

    let mut queue: VecDeque<Position> = VecDeque::new();

    for direction in Direction::iter() {
        for previous in &previous[direction.index()][grid.target] {
            queue.push_back(*previous);
        }
    }

    while let Some(current) = queue.pop_front() {
        visited.set(current.index, true);

        for previous in &previous[current.direction.index()][current.index] {
            queue.push_back(*previous);
        }
    }

    visited.count_ones(..)
}

fn inner(grid: &Grid) -> (usize, [Vec<ArrayVec<Position, 3>>; 4]) {
    // TODO: Do some preprocessing to make large steps in one go
    let mut heap: BinaryHeap<HeapState> = BinaryHeap::new();
    heap.push(HeapState {
        position: Position {
            index: grid.source,
            direction: Direction::Right,
        },
        previous_position: None,
        value: 0,
    });

    let mut best_value = usize::MAX;

    let mut best_value_per_position_direction: [Vec<usize>; 4] = [
        vec![usize::MAX; grid.size()],
        vec![usize::MAX; grid.size()],
        vec![usize::MAX; grid.size()],
        vec![usize::MAX; grid.size()],
    ];

    let mut previous: [Vec<ArrayVec<Position, 3>>; 4] = [
        vec![ArrayVec::new(); grid.size()],
        vec![ArrayVec::new(); grid.size()],
        vec![ArrayVec::new(); grid.size()],
        vec![ArrayVec::new(); grid.size()],
    ];

    while let Some(state) = heap.pop() {
        let direction_index = state.position.direction.index();

        let current_value =
            best_value_per_position_direction[direction_index][state.position.index];

        if current_value < state.value {
            continue;
        }

        if best_value < state.value {
            continue;
        }

        best_value_per_position_direction[direction_index][state.position.index] = state.value;

        if let Some(previous_position) = state.previous_position {
            if previous[state.position.direction.index()][state.position.index]
                .contains(&previous_position)
            {
                continue;
            }
            previous[state.position.direction.index()][state.position.index]
                .push(previous_position);
        }

        if state.position.index == grid.target {
            best_value = state.value;
            continue;
        }

        let (left, right) = state.position.direction.neighbours();

        heap.push(HeapState {
            value: state.value + 1000,
            position: Position {
                index: state.position.index,
                direction: left,
            },
            previous_position: Some(state.position),
        });
        heap.push(HeapState {
            value: state.value + 1000,
            position: Position {
                index: state.position.index,
                direction: right,
            },
            previous_position: Some(state.position),
        });

        let Some(next_position) =
            move_towards(state.position.index, state.position.direction, &grid)
        else {
            continue;
        };

        if !grid.inner.contains(next_position) {
            continue;
        }

        heap.push(HeapState {
            value: state.value + 1,
            position: Position {
                index: next_position,
                direction: state.position.direction,
            },
            previous_position: Some(state.position),
        });
    }

    (best_value, previous)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1A: &str = include_str!("../../examples/2024/day16_1a.txt");
    const TEST_INPUT_1B: &str = include_str!("../../examples/2024/day16_1b.txt");

    #[test]
    fn test_part1() {
        assert_eq!(7036, part1(TEST_INPUT_1A));
        assert_eq!(11048, part1(TEST_INPUT_1B));
    }

    #[test]
    fn test_part2() {
        assert_eq!(45, part2(TEST_INPUT_1A));
        assert_eq!(64, part2(TEST_INPUT_1B));
    }
}
