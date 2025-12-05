use std::fs;

/// Converts a slice of to base 10 number.
fn slice_to_base_10(bank: &[u64]) -> u64 {
    bank.iter()
        .rev()
        .enumerate()
        .map(|(e, &i)| i * 10u64.pow(e as u32))
        .sum()
}

fn find_max_joltage(bank: &[u64], battery_count: usize) -> u64 {
    let mut best = (&bank[bank.len() - battery_count..]).to_vec();
    // iteratively prepend batteries from back to front
    for &battery in (&bank[0..(bank.len() - battery_count)]).iter().rev() {
        // find best battery to remove
        let mut best_proposed = (0..battery_count)
            .map(|idx_to_remove| {
                best.iter()
                    .enumerate()
                    .filter_map(|(j, n)| if j != idx_to_remove { Some(*n) } else { None })
                    .collect::<Vec<_>>()
            })
            .max_by_key(|bank| slice_to_base_10(bank))
            .unwrap();
        best_proposed.insert(0, battery);
        if slice_to_base_10(&best_proposed) >= slice_to_base_10(&best) {
            best = best_proposed
        }
    }
    slice_to_base_10(&best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_n() {
        assert_eq!(find_max_joltage(&[9, 8, 7, 6, 5, 4], 2), 98);
        assert_eq!(find_max_joltage(&[8, 0, 0, 0, 0, 9], 2), 89);
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

    let banks: Vec<Vec<u64>> = contents
        .lines()
        .map(|l| {
            l.char_indices()
                .map(|(i, c)| (&l[i..i + c.len_utf8()]).parse().unwrap())
                .collect()
        })
        .collect();

    // part 1
    // let n = 2;
    // part 2
    let n = 12;
    let result: u64 = banks.iter().map(|b| find_max_joltage(b, n)).sum();
    println!("{}", result);
}
