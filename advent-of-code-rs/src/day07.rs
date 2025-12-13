use std::{collections::HashMap, fs};

#[allow(dead_code)]
pub fn main() {
    let contents = fs::read_to_string("assets/input07.txt").unwrap();
    let grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    let start_loc = grid[0].iter().position(|&c| c == 'S').unwrap();

    // PART 1
    // let mut beams = vec![start_loc];
    // let mut result = 0;
    // for row in &grid[1..] {
    //     let mut new_beams = Vec::new();
    //     for beam in beams {
    //         if row[beam] == '^' {
    //             new_beams.push(beam - 1);
    //             new_beams.push(beam + 1);
    //             result += 1;
    //         } else {
    //             new_beams.push(beam);
    //         }
    //     }
    //     beams = new_beams;
    //     beams.sort();
    //     beams.dedup();
    // }
    //
    // PART 2
    let result = count_paths(0, start_loc, &grid, &mut HashMap::new());

    println!("{}", result);
}

fn count_paths(
    mut row: usize,
    col: usize,
    grid: &Vec<Vec<char>>,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // move down until split
    while row < grid.len() && grid[row][col] != '^' {
        row += 1;
    }

    if let Some(cached_val) = cache.get(&(row, col)) {
        return *cached_val;
    }

    // base case
    if row == grid.len() {
        return 1;
    }

    let result =
        count_paths(row + 1, col - 1, grid, cache) + count_paths(row + 1, col + 1, grid, cache);

    cache.insert((row, col), result);
    result
}
