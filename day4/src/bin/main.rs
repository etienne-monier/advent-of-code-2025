extern crate nalgebra as na;
use day4::{DisplayMatrix, convolve2d, pad_matrix};
use na::DMatrix;

use std::fs;

const FILE_PATH: &str = "input.txt";

const THRESHOLD: i32 = 4;

fn get_accessible_rolls(matrix: &DMatrix<i32>, verbose: bool) -> DMatrix<i32> {
    let kernel = DMatrix::from_vec(3, 3, vec![1, 1, 1, 1, 0, 1, 1, 1, 1]);

    // Conpute the padded mattrix and its correlation
    let padded_matrix = pad_matrix(&matrix, 1);
    let result = convolve2d(&padded_matrix, &kernel);

    let accessible_rolls =
        result.zip_map(&matrix, |x, y| if x < THRESHOLD && y == 1 { 1 } else { 0 });

    if verbose {
        println!("Accessible rolls");
        println!("{}", DisplayMatrix(&accessible_rolls));
    }

    accessible_rolls
}

fn main() {
    // Open the file and read it.
    let content = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    // Get lines
    let lines = content.split("\n").collect::<Vec<&str>>();
    // Get data as a Vec
    let data = lines
        .iter()
        .flat_map(|s| s.chars().map(|x| if x == '@' { 1 } else { 0 }))
        .collect();

    // Get matrix size
    let (n_rows, n_cols) = (lines.len(), lines[0].len());

    // Create base matrix and kernel
    let mut matrix = DMatrix::from_vec(n_rows, n_cols, data).transpose(); // Transpose as column-by-column fill
    let mut accessible_rolls;
    let mut sum;
    let mut accu = 0;

    loop {
        accessible_rolls = get_accessible_rolls(&matrix, false);

        sum = accessible_rolls.sum();
        accu += sum;

        if sum == 0 {
            println!("No more accessible roll");
            break;
        } else {
            println!("Accessible rolls: {}", sum);

            matrix -= accessible_rolls;
        }
    }

    println!("Finished");

    println!("Used {} rolls", accu)
}
