type Element = u8;
type Output = u16;

#[derive(Copy, Clone, Debug)]
struct BitSet {
    inner: u128,
}

impl BitSet {
    pub fn zeros() -> Self {
        Self { inner: 0 }
    }

    pub fn insert(&mut self, n: u8) {
        self.inner = self.inner | (1 << n)
    }

    pub fn is_set(&self, n: u8) -> bool {
        (self.inner >> n) & 1 > 0
    }
}

pub struct BitSetIterator {
    value: u128,
}

impl BitSetIterator {
    pub fn new(value: u128) -> Self {
        Self {
            value,
        }
    }
}

impl Iterator for BitSetIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.value {
            0 => None,
            val => {
                let index = val.trailing_zeros();
                self.value ^= 1 << index;
                Some(index as u8)
            }
        }
    }
}

fn parse_input(input: &str) -> ([BitSet; 100], Vec<Vec<Element>>) {
    let (rules_input, updates) = input.split_once("\n\n").unwrap();

    let mut rules: [BitSet; 100] = [BitSet::zeros(); 100];

    rules_input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").unwrap();
            (b.parse::<Element>().unwrap(), a.parse::<Element>().unwrap())
        })
        .for_each(|(b, a)| {
            rules[b as usize].insert(a);
        });

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

pub fn part1(input: &str) -> Output {
    let (rules, updates) = parse_input(input);

    updates
        .iter()
        .filter(|update| {
            let mut includes: BitSet = BitSet::zeros();

            for element in update.iter() {
                includes.insert(*element);
            }

            let mut seen: BitSet = BitSet::zeros();

            for element in update.iter() {
                let rule = rules[*element as usize];

                let predecessor_not_seen = BitSetIterator::new(rule.inner)
                    .into_iter()
                    .any(|element| includes.is_set(element) && !seen.is_set(element));

                if predecessor_not_seen {
                    return false;
                }

                seen.insert(*element);
            }

            true
        })
        .map(|update| update[update.len() / 2] as Output)
        .sum()
}

pub fn part2(input: &str) -> Output {
    let (rules, mut updates) = parse_input(input);
    let mut index_of_element: [Option<u8>; 100] = [None; 100];

    updates
        .iter_mut()
        .filter_map(|update| {
            index_of_element = [None; 100];

            for (index, element) in update.iter().enumerate() {
                index_of_element[*element as usize] = Some(index as u8);
            }

            let mut changed = false;
            let mut index: u8 = 0;
            let target = update.len() as u8;

            while index < target {
                let element = update[index as usize];

                let rule = rules[element as usize];

                if rule.inner == 0 {
                    index += 1;
                    continue;
                }

                let minimum_index = BitSetIterator::new(rule.inner)
                    .into_iter()
                    .filter_map(|element| Some((index_of_element[element as usize]?, element)))
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
                update[minimum_index as usize] = element;

                index_of_element[other_element as usize] = Some(index);
                update[index as usize] = other_element;
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
