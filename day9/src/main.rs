use geo::{Coord, Covers};
use geo_types::{LineString, Polygon, Rect, coord};
use num_traits::{Num, Signed};
use regex::Regex;
use std::{fmt::Debug, fs, str::FromStr};

// The input file to be read.
const FILE_INPUT: &str = "input.txt";

/// Parse the file and return the points it contains.
///
/// # Returns
///
/// - `Vec<(T, T)> where T: FromStr, T::Err: Debug,` - The points the file contains.
///
fn parse_file<T>() -> Vec<(T, T)>
where
    T: FromStr,
    T::Err: Debug,
{
    let content = fs::read_to_string(FILE_INPUT).expect("Could not read the file");
    let data_regex = Regex::new(r"(\d+),(\d+)").unwrap();

    // Find all points in the file
    data_regex
        .captures_iter(&content)
        .map(|c| {
            let (_, [x, y]) = c.extract();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

/// Compute the areas of all possible rectangles and return them sorted in descending order.
///
/// # Arguments
///
/// - `x` (`&[(T, T`) - The points.
///
fn compute_areas<T>(x: &[(T, T)]) -> Vec<(usize, usize, T)>
where
    T: Num + Signed + Copy + PartialOrd,
{
    let n = x.len();
    let mut result = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            let area = ((x[j].0 - x[i].0).abs() + T::one()) * ((x[j].1 - x[i].1).abs() + T::one());
            result.push((i, j, area));
        }
    }

    // Reverse sort: largest areas first
    result.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    result
}

/// Find the largest rectangle included in the polygon.
///
/// # Arguments
///
/// - `poly` (`&[(f64, f64`) - The polygon points.
///
fn find_largest_interior_rectangle(poly: &[(f64, f64)], areas: &[(usize, usize, f64)]) {
    let mut poly_coords: Vec<Coord<f64>> = poly
        .iter()
        .cloned()
        .map(|(x, y)| coord! {x:x,y:y})
        .collect();
    poly_coords.push(coord! {x: poly[0].0 , y: poly[0].1});

    let polygon = Polygon::new(LineString::from(poly_coords), vec![]);

    let mut cnt = 0;

    loop {
        let &(i, j, area) = &areas[cnt];
        let (point_i, point_j) = (poly[i], poly[j]);
        let rectangle = Rect::new(
            coord! {x: point_i.0, y:point_i.1},
            coord! {x: point_j.0, y:point_j.1},
        )
        .to_polygon();

        if polygon.covers(&rectangle) {
            println!("Largest included area: {} (index #{})", area, cnt);
            break;
        } else {
            cnt += 1;
        }
    }
}

fn main() {
    let points = parse_file::<f64>();
    let areas = compute_areas(&points);
    println!("Largest area: {}", areas[0].2);

    find_largest_interior_rectangle(&points, &areas)
}
