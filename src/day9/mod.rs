use itertools::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::{Display, Formatter};

fn format_disk(disk: &[Option<usize>]) {
    for element in disk {
        let element = match element {
            None => ".".to_string(),
            Some(index) => index.to_string(),
        };
        print!("{}", element);
    }
    println!();
}

fn parse_input(input: &str) -> Disk {
    let mut disk = Disk::default();
    let mut empty = false;

    for count in input.lines().next().unwrap().chars() {
        let length = count.to_digit(10).unwrap() as usize;

        if empty {
            empty = false;
            disk.add_gap(length);
        } else {
            empty = true;
            disk.add_block(length);
        }
    }

    disk
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let mut disk: Vec<Option<usize>> = Vec::new();
    let mut next_id = 0;
    let mut empty = false;

    for count in input.lines().next().unwrap().chars() {
        let element = match empty {
            true => {
                empty = false;
                None
            }
            false => {
                empty = true;
                next_id += 1;
                Some(next_id - 1)
            }
        };

        let count = count.to_digit(10).unwrap() as usize;
        disk.extend((0..count).map(|_| element));
    }

    let mut first_empty_index = disk.iter().position(|element| element.is_none()).unwrap();
    let mut last_used_index = disk.iter().rposition(|element| element.is_some()).unwrap();

    while first_empty_index < last_used_index {
        disk.swap(first_empty_index, last_used_index);
        first_empty_index += 1;
        last_used_index -= 1;

        // TODO: We can know how much to proceed by means of some preprocessing such that we don't have to check this too much
        while disk[first_empty_index].is_some() {
            first_empty_index += 1;
        }

        while disk[last_used_index].is_none() {
            last_used_index -= 1;
        }
    }

    disk.iter()
        .enumerate()
        .map(|(index, element)| match element {
            None => 0,
            Some(element) => element * index,
        })
        .sum()
}

#[derive(Default)]
struct Disk {
    index: usize,

    blocks: Vec<Block>,

    // Key: start index of the gap
    // Value: length of the gap
    gaps: [BinaryHeap<Reverse<usize>>; 10],
}

impl Disk {
    fn checksum(&self) -> usize {
        self.blocks.iter().map(|block| block.checksum()).sum()
    }

    fn add_block(&mut self, length: usize) {
        if length == 0 {
            return;
        }

        self.blocks.push(Block {
            id: self.blocks.len(),
            start_index: self.index,
            length,
        });
        self.index += length;
    }

    fn add_gap(&mut self, length: usize) {
        if length == 0 {
            return;
        }

        self.gaps[length].push(Reverse(self.index));
        self.index += length;
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let elements = self
            .blocks
            .iter()
            .map(|block| (block.start_index, block.length, block.id.to_string()))
            .chain(self.gaps.iter().enumerate().flat_map(|(length, gaps)| {
                gaps.iter()
                    .map(move |index| (index.0, length, ".".to_string()))
            }))
            .sorted_by_key(|(index, _, _)| *index);

        for (_, length, output) in elements {
            for _ in 0..length {
                write!(f, "{}", output)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Block {
    id: usize,
    start_index: usize,
    length: usize,
}

impl Block {
    fn checksum(&self) -> usize {
        (0..self.length)
            .map(|offset| (offset + self.start_index) * self.id)
            .sum()
    }
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let mut disk = parse_input(input);

    for block_index in (0..disk.blocks.len()).rev() {
        let block = &mut disk.blocks[block_index];
        let mut earliest_large_enough_gap_index = block.start_index;
        let mut earliest_large_enough_gap_size = None;

        for gap_size in block.length..10 {
            let Some(Reverse(gap_index)) = disk.gaps[gap_size].peek() else {
                continue;
            };

            if *gap_index >= earliest_large_enough_gap_index {
                continue;
            }

            earliest_large_enough_gap_index = *gap_index;
            earliest_large_enough_gap_size = Some(gap_size);
        }

        let Some(earliest_large_enough_gap_size) = earliest_large_enough_gap_size else {
            continue;
        };

        disk.gaps[earliest_large_enough_gap_size].pop();
        disk.gaps[block.length].push(Reverse(block.start_index));

        block.start_index = earliest_large_enough_gap_index;

        if earliest_large_enough_gap_size > block.length {
            disk.gaps[earliest_large_enough_gap_size - block.length]
                .push(Reverse(earliest_large_enough_gap_index + block.length));
        }
    }

    disk.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../examples/2024/day9.txt");

    #[test]
    fn test_part1() {
        assert_eq!(1928, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2858, part2(TEST_INPUT));
    }
}
