use regex::Regex;
use std::fs;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[allow(dead_code)]
pub fn main() {
    let contents = fs::read_to_string("assets/input06.txt").unwrap();

    // PART 1
    // let re = Regex::new(r"\S+").unwrap();
    //
    // let mut grid: Vec<Vec<String>> = contents
    //     .lines()
    //     .map(|l| re.find_iter(l).map(|w| w.as_str().into()).collect())
    //     .collect();
    //
    // let ops = grid.pop().unwrap();
    // let nums: Vec<Vec<i64>> = grid
    //     .into_iter()
    //     .map(|line| line.into_iter().map(|e| e.parse().unwrap()).collect())
    //     .collect();
    //
    // let mut result = 0;
    // for col in 0..nums[0].len() {
    //     let mut col_result = nums[0][col];
    //     for row in 1..nums.len() {
    //         let val = nums[row][col];
    //         match ops[col].as_str() {
    //             "+" => col_result += val,
    //             "-" => col_result -= val,
    //             "*" => col_result *= val,
    //             "/" => col_result /= val,
    //             _ => panic!("ouch"),
    //         }
    //     }
    //     result += col_result;
    // }

    // PART 2
    let mut grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    let ops: Vec<char> = grid
        .pop()
        .unwrap()
        .into_iter()
        .rev()
        .filter(|c| !c.is_whitespace())
        .collect();

    let mut grid = transpose(grid);
    grid.reverse();

    let groups: Vec<Vec<i64>> = grid
        // split groups on empty rows
        .split(|row| row.iter().all(|c| c.is_whitespace()))
        .map(|group| {
            group
                .iter()
                .map(|row| row.iter().collect::<String>().trim().parse().unwrap())
                .collect()
        })
        .collect();

    let result: i64 = groups
        .into_iter()
        .zip(ops)
        .map(|(group, op)| {
            group
                .into_iter()
                .reduce(|acc, num| match op {
                    '+' => acc + num,
                    '-' => acc - num,
                    '*' => acc * num,
                    '/' => acc / num,
                    _ => panic!("ouch"),
                })
                .unwrap()
        })
        .sum();

    println!("{result:?}");
}
