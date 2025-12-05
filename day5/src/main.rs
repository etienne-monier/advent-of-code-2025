use interval::{
    IntervalSet,
    prelude::{Cardinality, Contains, ToIntervalSet},
};
use regex::Regex;
use std::fs;
use std::time::Instant;

// The test file path
const FILE_PATH: &str = "input.txt";

/// Parse the input file
///
/// # Returns
///
/// - `IntervalSet<u64>` - The fresh ingredients interval set.
/// - `Vec<u64>` - The vector of available ingredients.
///
fn parse_file() -> (IntervalSet<u64>, Vec<u64>) {
    // Read file content
    let content = fs::read_to_string(&FILE_PATH).expect("Should have been able to read the file");

    // Regexes
    let re_id = Regex::new(r"(?m)^(\d+)$").unwrap();
    let re_range = Regex::new(r"(?m)^(\d+)-(\d+)$").unwrap();

    // Get fresh ranges
    let fresh_ranges: Vec<(u64, u64)> = re_range
        .captures_iter(&content)
        .map(|caps| {
            let (_, [start, end]) = caps.extract();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    // Get ingredient ids
    let ingredient_ids: Vec<u64> = re_id
        .captures_iter(&content)
        .map(|caps| {
            let (_, [id]) = caps.extract();
            id.parse().unwrap()
        })
        .collect();

    (fresh_ranges.to_interval_set(), ingredient_ids)
}

/// Return the fresh ids
///
/// # Arguments
///
/// - `fresh_ranges` (`&IntervalSet<u64>`) - The interval set of fresh ingredient ids.
/// - `ingredient_ids` (`&[u64]`) - The list of ingredient ids.
///
/// # Returns
///
/// - `Vec<u64>` - The vector of fresh ingredient ids.
///
fn get_fresh_ids(fresh_ranges: &IntervalSet<u64>, ingredient_ids: &[u64]) -> Vec<u64> {
    ingredient_ids
        .iter()
        .copied()
        .filter(|id| fresh_ranges.contains(id))
        .collect()
}

fn main() {
    let now = Instant::now();

    let (fresh_ranges, ingredient_ids) = parse_file();

    println!("Interval set: {}", fresh_ranges);
    println!("Ingredient ids: {:?}", ingredient_ids);

    let fresh_ids = get_fresh_ids(&fresh_ranges, &ingredient_ids);
    println!("Fresh ids: {:?}", fresh_ids);

    println!("Kitchen has {} fresh ids", fresh_ids.len());
    println!("Num of possible fresh ids: {}", fresh_ranges.size());

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
