use itertools::{cloned, Itertools};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::HashMap;
use std::io::BufRead;

type Element = u8;
type Output = u16;

fn parse_input(input: &str) -> (HashMap<Element, Vec<Element>>, Vec<Vec<Element>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").unwrap();
            (b.parse::<Element>().unwrap(), a.parse::<Element>().unwrap())
        })
        .into_group_map();

    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|element| element.parse::<Element>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    (rules, updates)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> Output {
    let (rules, updates) = parse_input(input);

    updates
        .iter()
        .filter(|update| {
            let includes: FxHashSet<Element> = update.iter().copied().collect();
            let mut seen: FxHashSet<Element> = FxHashSet::default();

            for element in update.iter() {
                if let Some(rule) = rules.get(element) {
                    let predecessor_not_seen = rule
                        .iter()
                        .filter(|element| includes.contains(element))
                        .any(|element| !seen.contains(element));

                    if predecessor_not_seen {
                        return false;
                    }
                }

                seen.insert(*element);
            }

            true
        })
        .map(|update| update[update.len() / 2] as Output)
        .sum()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> Output {
    let (rules, mut updates) = parse_input(input);
    let mut index_of_element = [None; 100];

    updates
        .iter_mut()
        .filter_map(|update| {
            index_of_element = [None; 100];

            for (index, element) in update.iter().enumerate() {
                index_of_element[*element as usize] = Some(index);
            }

            let mut changed = false;
            let mut index = 0;

            while index < update.len() {
                let element = update[index];

                let Some(rule) = rules.get(&element) else {
                    index += 1;
                    continue;
                };

                let minimum_index = rule
                    .iter()
                    .filter_map(|element| Some((index_of_element[*element as usize]?, element)))
                    .max_by_key(|(index, _)| *index);

                let Some((minimum_index, other_element)) = minimum_index else {
                    index += 1;
                    continue;
                };

                if index > minimum_index {
                    index += 1;
                    continue;
                }

                changed = true;
                index_of_element[element as usize] = Some(minimum_index);
                update[minimum_index] = element;

                index_of_element[*other_element as usize] = Some(index);
                update[index] = *other_element;
            }

            if !changed {
                return None;
            }

            Some(update[update.len() / 2] as Output)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../examples/2024/day5.txt");

    #[test]
    fn test_part1() {
        assert_eq!(143, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(123, part2(TEST_INPUT));
    }
}
