use std::fs;

const FILE_PATH: &str = "input.txt";

// 2 for first step, 12 for second.
const SIZE: usize = 12;

fn get_max_joltage(line: &str, size: usize) -> i64 {
    // Get the line length
    let length = line.len();

    // Convert the string into a vec of chars
    let chars: Vec<char> = line.chars().collect();

    let mut min_idx = 0;
    let mut max_idx;
    let mut res_vec: Vec<char> = Vec::new();

    for digit_num in (0..size).rev() {
        // The max index to search for max digit.
        max_idx = length - digit_num;

        // The vector to search for max digit
        // v[min_pos..max_pos]
        let (max_pos, max_digit) = chars[min_idx..max_idx]
            .iter()
            .enumerate()
            .rev() // WARN ! REVERSED as the max_by_key return last element in case equal value
            .max_by_key(|&(_i, val)| val)
            .unwrap();

        // Update min index
        min_idx = min_idx + max_pos + 1;

        // Add current char
        res_vec.push(*max_digit);
    }

    String::from_iter(res_vec).parse().unwrap()
}

fn main() {
    // Open the file and read it.
    let content = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let mut res: i64 = 0;

    // Iterate over lines
    for line in content.split("\n") {
        let tmp_joltage_value = get_max_joltage(&line, SIZE);

        println!("{}", tmp_joltage_value);
        res += tmp_joltage_value;
    }

    println!("The total joltage is: {}", res)
}
