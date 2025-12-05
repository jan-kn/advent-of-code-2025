use std::fs;

#[allow(dead_code)]
pub fn main() {
    let contents = fs::read_to_string("assets/input04.txt").unwrap();

    let mut grid: Vec<Vec<bool>> = contents
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();

    let nx = grid.len();
    let ny = grid[0].len();

    let mut result = 0;
    loop {
        // keep track of removed rolls
        let mut removed = vec![vec![false; ny]; nx];
        let mut n_removed = 0;
        for x in 0..nx {
            for y in 0..ny {
                let mut n_adjacent = 0;
                if !grid[x][y] {
                    continue;
                }
                for dx in -1isize..=1 {
                    for dy in -1isize..=1 {
                        if (dx == 0 && dy == 0)
                            || !(0..nx).contains(&((x as isize + dx) as usize))
                            || !(0..ny).contains(&((y as isize + dy) as usize))
                        {
                            continue;
                        }
                        if grid[(x as isize + dx) as usize][(y as isize + dy) as usize] {
                            n_adjacent += 1;
                        }
                    }
                }
                if n_adjacent < 4 {
                    n_removed += 1;
                    removed[x][y] = true;
                }
            }
        }

        if n_removed == 0 {
            break;
        }

        result += n_removed;

        // remove from grid
        for x in 0..nx {
            for y in 0..ny {
                if removed[x][y] {
                    grid[x][y] = false;
                }
            }
        }

        // part 1/2
        // break;
    }

    println!("{}", result);
}
