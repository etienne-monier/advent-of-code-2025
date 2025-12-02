use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";

fn get_ranges() -> Vec<(i64, i64)> {
    // Read input file
    let content = fs::read_to_string(INPUT_FILE_PATH).unwrap();

    // Parse content
    content
        .split(",")
        .map(|range| {
            let t = range
                .split_once("-")
                .expect("Expected format like 123-1234");
            (t.0.parse::<i64>().unwrap(), t.1.parse::<i64>().unwrap())
        })
        .collect()
}

fn first_part() {
    // The final value
    let mut accu: i64 = 0;

    for (a, b) in get_ranges() {
        for num in a..=b {
            let num_str = num.to_string();
            let lenght = num_str.len();

            if lenght % 2 == 0 {
                // We can split this id
                let (start, end) = num_str.split_at(lenght / 2);

                if start == end {
                    // This is a wrong id
                    accu += num;
                }
            }
        }
    }

    println!("First part: The sum of all invalid ids is {}", accu);
}

fn second_part() {
    // The final value
    let mut accu: i64 = 0;

    for (a, b) in get_ranges() {
        for num in a..=b {
            let num_str = num.to_string();
            let lenght = num_str.len();

            for factor in 2..10 {
                if lenght % factor == 0 {
                    // We can split this id
                    let (start, _) = num_str.split_at(lenght / factor);

                    if num_str == start.repeat(factor) {
                        // This is a wrong id
                        accu += num;
                        // Break to avoid counting it several times.
                        break;
                    }
                }
            }
        }
    }

    println!("Second part: The sum of all invalid ids is {}", accu);
}

fn main() {
    first_part();
    second_part();
}
