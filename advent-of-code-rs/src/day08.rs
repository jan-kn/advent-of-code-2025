use rayon::prelude::*;
use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Point(i64, i64, i64);

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        (((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)) as f64)
            .sqrt()
    }
}

#[allow(dead_code)]
pub fn main() {
    let contents = fs::read_to_string("assets/input08.txt").unwrap();

    let boxes: Vec<Point> = contents
        .lines()
        .map(|l| {
            let mut iter = l.split(',');
            Point(
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut distances: Vec<((usize, usize), f64)> = (0..boxes.len())
        .flat_map(|i| ((i + 1)..boxes.len()).map(move |j| (i, j)))
        .map(|(i, j)| ((i, j), boxes[i].distance(&boxes[j])))
        .collect();
    distances.par_sort_by(|(_, d_1), (_, d_2)| d_1.total_cmp(d_2));

    let mut box_to_cluster: HashMap<usize, usize> = (0..boxes.len()).map(|i| (i, i)).collect();
    let mut last_connection = None;

    // PART 1
    // for (connection, _) in distances.iter().take(1000) {
    for (connection, _) in distances.iter() {
        let cluster_a = box_to_cluster[&connection.0];
        let cluster_b = box_to_cluster[&connection.1];

        // already in the same cluster
        if cluster_a == cluster_b {
            continue;
        }

        // connect all to cluster a
        for (_, b) in box_to_cluster.iter_mut() {
            if *b == cluster_b {
                *b = cluster_a;
            }
        }

        last_connection = Some(connection);
    }

    let mut box_to_clusters = box_to_cluster.iter().collect::<Vec<_>>();
    box_to_clusters.sort_by_key(|e| e.1);

    let mut cluster_sizes = box_to_clusters
        .chunk_by(|a, b| a.1 == b.1)
        .map(|g| g.len())
        .collect::<Vec<_>>();
    cluster_sizes.sort();

    // PART 1
    // let result = &cluster_sizes[(cluster_sizes.len() - 3)..]
    //     .iter()
    //     .copied()
    //     .reduce(|acc, e| acc * e);

    let result = boxes[last_connection.unwrap().0].0 * boxes[last_connection.unwrap().1].0;
    println!("{:?}", result);
}
