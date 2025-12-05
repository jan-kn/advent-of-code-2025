use rayon::prelude::*;
use std::fs;

/// Returns the base 10 length of a given integer.
fn num_length(num: u64) -> u32 {
    num.ilog10() + 1
}

/// Splits a given integer into `n_splits` equal size chunks (in reverse order).
/// This function does not validate that `n_splits` is a sensible split and may panic.
fn split_n_wise(mut num: u64, n_splits: u32) -> impl Iterator<Item = u64> {
    let length = num_length(num);

    // NOTE: return iterator to avoid heap allocations. Results in 5x speedup of part 2.
    (0..n_splits).map(move |_| {
        let exp = 10u64.pow(length / n_splits);
        let right = num % exp;
        num = num / exp;
        right
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_n_wise() {
        assert!(split_n_wise(123456, 1).eq([123456]));
        assert!(split_n_wise(123456, 2).eq([456, 123]));
        assert!(split_n_wise(123456, 3).eq([56, 34, 12]));
        assert!(split_n_wise(123456, 6).eq([6, 5, 4, 3, 2, 1]));
    }
}

#[allow(dead_code)]
pub fn main() {
    let contents = fs::read_to_string("assets/input02.txt").unwrap();

    let ranges = contents
        .trim()
        .split(",")
        .map(|l| {
            let (lower, upper) = l.split_once('-').unwrap();
            (lower.parse::<u64>().unwrap(), upper.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    let result = ranges
        .par_iter()
        // .iter()
        .flat_map(|&(lower, upper)| lower..=upper)
        .filter(|&id| {
            let length = num_length(id);
            // part 1
            // (2..=2)
            // part 2
            (2..=length).any(|n_splits| {
                if length % n_splits != 0 {
                    return false;
                }
                let mut splits = split_n_wise(id, n_splits);
                let first = splits.next().unwrap();
                splits.all(|e| e == first)
            })
        })
        .sum::<u64>();

    println!("{}", result);
}
