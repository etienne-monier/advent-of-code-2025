use regex::Regex;
use std::fs;

const FILE_PATH: &str = "input.txt";
const NUM_POS: i32 = 100;

#[derive(Debug)]
enum Movement {
    Left(i32),
    Right(i32),
}

impl From<&str> for Movement {
    fn from(value: &str) -> Self {
        // The input value regex
        let re = Regex::new(r"(?<direction>L|R)(?<num>\d+)$").unwrap();

        // Parse the input value
        let Some(caps) = re.captures(value) else {
            panic!("Value {value} is not a correct movement.");
        };

        match &caps["direction"] {
            "L" => Movement::Left((&caps["num"]).parse::<i32>().unwrap()),
            "R" => Movement::Right((&caps["num"]).parse::<i32>().unwrap()),
            _ => panic!("Invalid direction"),
        }
    }
}

fn apply_movement(position: &mut i32, movement: Movement) -> i32 {
    let previous_pos = *position;

    let next_pos = match movement {
        Movement::Left(num) => *position - num,
        Movement::Right(num) => *position + num,
    };
    *position = next_pos.rem_euclid(NUM_POS);

    let num_zero_skip = next_pos.div_euclid(NUM_POS).abs();

    if *position == 0 && previous_pos == 0 {
        num_zero_skip - 1
    } else {
        num_zero_skip
    }
}

fn main() {
    // Open the file and read it.
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    // Get the movements to execute
    let movements = contents.split("\n").map(|mov| Movement::from(mov));

    // Perform the movements and update the position.
    let mut position = 50;
    let mut num_of_zeros_stops = 0;
    let mut num_of_zeros_skip = 0;

    for mov in movements {
        num_of_zeros_skip += apply_movement(&mut position, mov);

        if position == 0 {
            num_of_zeros_stops += 1
        }
    }

    let total = num_of_zeros_stops + num_of_zeros_skip;

    // Print result
    println!("Num of zeros positions: {num_of_zeros_stops}");
    println!("Num of zeros skip: {num_of_zeros_skip}");
    println!("Num of zeros totol: {total}");
}
