use std::fs;

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn contains(&self, x: i64) -> bool {
        self.start <= x && x <= self.end
    }

    fn len(&self) -> i64 {
        self.end - self.start + 1
    }

    fn merge(self, other: Range) -> (Range, Option<Range>) {
        // check if disjoint
        if self.end + 1 < other.start || other.end + 1 < self.start {
            return (self, Some(other));
        }

        (
            Range {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            },
            None,
        )
    }
}

#[allow(dead_code)]
pub fn main() {
    let contents = fs::read_to_string("assets/input05.txt").unwrap();

    let mut ranges = contents
        .clone()
        .lines()
        .take_while(|&l| l != "")
        .map(|l| {
            let (left, right) = l.split_once("-").unwrap();
            Range {
                start: left.parse().unwrap(),
                end: right.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let items = contents
        .lines()
        .skip_while(|&l| l != "")
        .skip(1)
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // PART 1
    // let result = items
    //     .iter()
    //     .filter(|item| ranges.iter().any(|r| r.contains(**item)))
    //     .count();

    // PART 2
    ranges.sort_by_key(|r| -r.start);
    let mut merged_ranges = vec![];
    let mut current = ranges.pop().unwrap();
    while let Some(range) = ranges.pop() {
        match current.merge(range) {
            (merged, None) => {
                current = merged;
            }
            (disjoint_1, Some(disjoint_2)) => {
                merged_ranges.push(disjoint_1);
                current = disjoint_2;
            }
        }
    }
    merged_ranges.push(current);

    let result: i64 = merged_ranges.iter().map(Range::len).sum();
    println!("{}", result);
}
