use std::fs;

pub fn main() {
    let contents = fs::read_to_string("assets/input01.txt").unwrap();

    let rotations = contents.lines().map(|l| match l.split_at(1) {
        ("L", num) => -num.parse::<i64>().unwrap(),
        ("R", num) => num.parse::<i64>().unwrap(),
        _ => panic!("invalid line {}", l),
    });

    let start_pos = 50;
    let size = 100;

    // PART 1
    // let result = rotations
    //     .fold((start_pos, 0), |(pos, count), num| {
    //         match (pos + num).rem_euclid(size) {
    //             0 => (0, count + 1),
    //             i => (i, count),
    //         }
    //     })
    //     .1;

    // PART 2
    let result = rotations
        .fold::<(i64, i64), _>((start_pos, 0), |(pos, n), rotation| {
            (
                // new position
                (pos + rotation).rem_euclid(size),
                // old + num full rotations (div) + possible partial rotation (mod) over zero
                n + (rotation / size).abs()
                    + i64::from(pos != 0 && !(1..=99).contains(&(pos + (rotation % size)))),
            )
        })
        .1;

    println!("{}", result);
}
