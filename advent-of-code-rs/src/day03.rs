use std::fs;

/// Converts a slice of to base 10 number.
fn slice_to_base_10(bank: &[u8]) -> u64 {
    bank.iter()
        .rev()
        .enumerate()
        .map(|(e, &i)| u64::from(i) * 10u64.pow(e as u32))
        .sum()
}

/// Removes digit at position `idx` in base 10.
fn remove_digit(num: u64, idx: u32) -> u64 {
    let len = num.ilog10() + 1;
    let exp = 10u64.pow(len - 1 - idx);
    let left = num / (exp * 10);
    let right = num % exp;
    left * exp + right
}

fn find_max_joltage(bank: &[u8], battery_count: usize) -> u64 {
    let mut best = slice_to_base_10(&bank[bank.len() - battery_count..]);
    // iteratively prepend batteries from back to front
    for &battery in (&bank[0..(bank.len() - battery_count)]).iter().rev() {
        // find best battery to remove
        let best_with_removed = (0u32..battery_count as u32)
            .map(|idx_to_remove| remove_digit(best, idx_to_remove))
            .max()
            .unwrap();
        let proposed = (battery as u64) * 10u64.pow(battery_count as u32 - 1) + best_with_removed;
        best = best.max(proposed);
    }
    return best;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_digit() {
        assert_eq!(remove_digit(15, 1), 1);
        assert_eq!(remove_digit(15, 0), 5);
        assert_eq!(remove_digit(1235, 1), 135);
    }

    #[test]
    fn test_find_max_n() {
        assert_eq!(find_max_joltage(&[9, 8, 7, 6, 5, 4], 2), 98);
        assert_eq!(find_max_joltage(&[8, 1, 1, 1, 1, 9], 2), 89);
        assert_eq!(find_max_joltage(&[2, 3, 4, 2, 7, 8], 2), 78);
        assert_eq!(find_max_joltage(&[1, 9, 1, 1, 2, 1], 2), 92);
        assert_eq!(find_max_joltage(&[9, 9, 1, 1, 2, 1], 2), 99);

        assert_eq!(
            find_max_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12),
            987654321111
        );
        assert_eq!(
            find_max_joltage(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12),
            811111111119
        );
        assert_eq!(
            find_max_joltage(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12),
            434234234278
        );
        assert_eq!(
            find_max_joltage(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12),
            888911112111
        );
    }

    #[test]
    fn test_to_base_10() {
        assert_eq!(slice_to_base_10(&[9, 8, 7, 6, 5, 4]), 987654);
    }
}

#[allow(dead_code)]
pub fn main() {
    let contents = fs::read_to_string("assets/input03.txt").unwrap();

    // let banks = contents.lines().map(|l| l.as_bytes());
    let banks = contents.lines().map(|l| {
        l.char_indices()
            .map(|(i, c)| (&l[i..i + c.len_utf8()]).parse().unwrap())
            .collect::<Vec<_>>()
    });

    // part 1
    // let result: u64 = banks.map(|b| find_max_joltage(&b, 2)).sum();
    // part 2
    let result: u64 = banks.map(|b| find_max_joltage(&b, 12)).sum();
    println!("{}", result);
}
