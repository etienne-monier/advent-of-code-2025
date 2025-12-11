use nalgebra::{Matrix3xX, Vector3};
use regex::Regex;
use std::fs;

use std::collections::HashSet;

const FILE_INPUT: &str = "input.txt";

const NUM_CONNECTIONS: i32 = 1000;

fn parse_file() -> Matrix3xX<f64> {
    let content = fs::read_to_string(FILE_INPUT).expect("Could not read the file");
    let data_regex = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();

    // Find all points in the file and return them as Vector3
    let points: Vec<Vector3<f64>> = data_regex
        .captures_iter(&content)
        .map(|c| {
            let (_, [x, y, z]) = c.extract();
            Vector3::new(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
        })
        .collect();

    // Return the points as a matrix.
    Matrix3xX::from_columns(&points)
}

fn sorted_pairwise_distances(x: &Matrix3xX<f64>) -> Vec<(f64, usize, usize)> {
    let n = x.ncols();
    let mut result = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            let d = (x.column(i) - x.column(j)).norm();
            result.push((d, i, j));
        }
    }

    result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    result
}

/// Connect a pair and return true in there was a connection indeed.
///
/// # Arguments
///
/// - `i` (`usize`) - The first junction box indice.
/// - `j` (`usize`) - The second junction box indice.
/// - `circuits` (`&mut Vec<HashSet<usize>>`) - The current circuit state.
///
fn connect_pair(i: usize, j: usize, circuits: &mut Vec<HashSet<usize>>) -> bool {
    let pos_i = circuits.iter().position(|set| set.contains(&i));
    let pos_j = circuits.iter().position(|set| set.contains(&j));

    if pos_i.is_some() && pos_j.is_none() {
        // i belongs to a circuit but j does not.
        // Add j to circuit of i
        circuits[pos_i.unwrap()].insert(j);

        true
    } else if pos_i.is_none() && pos_j.is_some() {
        // j belongs to a circuit but i does not.
        // Add i to circuit of j
        circuits[pos_j.unwrap()].insert(i);

        true
    } else if pos_i.is_none() && pos_j.is_none() {
        // None of i or j belongs to a circuit
        // Connect {i, j} to create a new circuit.
        circuits.push(HashSet::from([i, j]));

        true
    } else {
        // Both i and j belong to a circuit.
        if pos_i.unwrap() != pos_j.unwrap() {
            // Merge both circuits
            let new_circuit = circuits[pos_i.unwrap()]
                .union(&circuits[pos_j.unwrap()])
                .cloned()
                .collect();

            let min_pos = pos_i.min(pos_j).unwrap();
            let max_pos = pos_i.max(pos_j).unwrap();

            circuits.remove(max_pos);
            circuits.remove(min_pos);
            circuits.push(new_circuit);

            true
        } else {
            // They both belong to the same circut. Skip.
            false
        }
    }
}

fn perform_connections(data: &Vec<(f64, usize, usize)>) -> usize {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    for current_distance_index in 0..NUM_CONNECTIONS as usize {
        // Get next least distance
        let &(_, i, j) = &data[current_distance_index];
        // Connect pairs.
        connect_pair(i, j, &mut circuits);
    }

    // Sort circuits by size
    circuits.sort_by_key(|v| v.len());

    // Print circuits
    for circuit in circuits.iter() {
        println!("{:?}", circuit);
    }

    // Compute result
    let &res = &circuits[circuits.len() - 3..circuits.len()]
        .iter()
        .fold(1, |acc, v| acc * v.len());

    println!("Result: {}", res);

    res
}

fn get_longest_connection(data: &Vec<(f64, usize, usize)>, positions: &Matrix3xX<f64>) -> f64 {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    let mut last_index: usize = 0;

    for current_distance_index in 0..data.len() as usize {
        // Get next least distance
        let &(_, i, j) = &data[current_distance_index];

        if connect_pair(i, j, &mut circuits) {
            last_index = current_distance_index;
        };
    }

    let &(_, i, j) = &data[last_index];

    let res = positions[(0, i)] * positions[(0, j)];

    println!("Result: {}", res);

    res
}

fn main() {
    let m = parse_file();
    let data = sorted_pairwise_distances(&m);
    perform_connections(&data);
    get_longest_connection(&data, &m);
}
