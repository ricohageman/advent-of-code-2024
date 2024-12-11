use rustc_hash::FxHashMap;

type Stone = usize;
type Iteration = usize;

fn blink(
    stone: Stone,
    iterations_left: Iteration,
    cache: &mut FxHashMap<(Stone, Iteration), usize>,
) -> usize {
    // Base case: If there are no more iterations there is just one stone
    if iterations_left == 0 {
        return 1;
    }

    // Base case: If there is a cache hit, use that value
    if let Some(output) = cache.get(&(stone, iterations_left)) {
        return *output;
    }

    // Rule 1: If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
    if stone == 0 {
        // Optimisation: This iteration zero is converted into one and in the next iteration it is
        //  converted into 2024. So let's skip ahead a little bit.
        if iterations_left <= 2 {
            return 1;
        }

        let value = blink(2024, iterations_left - 2, cache);
        cache.insert((0, iterations_left), value);
        cache.insert((1, iterations_left - 1), value);
        return value;
    }

    // Rule 2: If the stone is engraved with a number that has an even number of digits, it is replaced
    //          by two stones. The left half of the digits are engraved on the new left stone, and the
    //          right half of the digits are engraved on the new right stone.
    let number_of_digits = stone.ilog10() + 1;
    if number_of_digits % 2 == 0 {
        // Optimisation: Don't perform the actual manipulation of numbers if it's not required
        if iterations_left == 1 {
            return 2;
        }

        let number_of_digits = number_of_digits / 2;

        let value = blink(
            stone % 10_usize.pow(number_of_digits),
            iterations_left - 1,
            cache,
        ) + blink(
            stone / 10_usize.pow(number_of_digits),
            iterations_left - 1,
            cache,
        );

        cache.insert((stone, iterations_left), value);
        return value;
    }

    // Rule 3: The stone is replaced by a new stone; the old stone's number multiplied by 2024 is
    //          engraved on the new stone.
    let value = blink(stone * 2024, iterations_left - 1, cache);
    cache.insert((stone, iterations_left), value);
    value
}

pub fn part1(input: &str) -> usize {
    inner(input, 25)
}

pub fn part2(input: &str) -> usize {
    inner(input, 75)
}

fn inner(input: &str, iterations: usize) -> usize {
    let mut cache: FxHashMap<(Stone, Iteration), usize> = FxHashMap::default();

    input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|stone| stone.parse::<Stone>().unwrap())
        .map(|stone| blink(stone, iterations, &mut cache))
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(55312, part1("125 17"));
    }
}
