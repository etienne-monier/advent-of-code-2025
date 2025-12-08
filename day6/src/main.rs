use regex::Regex;
use std::fs;

// The test file path
const FILE_PATH: &str = "input.txt";

/// An operator
///
/// # Variants
///
/// - `Add` - An addition.
/// - `Sub` - A subtraction.
/// - `Mult` - A mutiplication.
/// - `Div` - A division.
///
#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mult,
    Div,
}

/// Implement From trait to parse str into an operation.
///
impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mult,
            "/" => Operator::Div,
            val => panic!("Invalid operator {}", val),
        }
    }
}

/// An operation comming from the input.
#[derive(Debug)]
struct Operation {
    nums: Vec<i64>,
    operator: Option<Operator>,
}

impl Operation {
    fn init(first_num: i64) -> Self {
        Self {
            nums: vec![first_num],
            operator: None,
        }
    }

    fn add_num(&mut self, num: i64) {
        self.nums.push(num);
    }

    fn add_operator(&mut self, value: &str) {
        self.operator = Some(Operator::from(value));
    }

    fn compute(&self) -> i64 {
        let mut iter = self.nums.iter();
        let first_num = *iter.next().unwrap();

        iter.fold(first_num, |acc, x| match self.operator {
            Some(Operator::Add) => acc + x,
            Some(Operator::Sub) => acc - x,
            Some(Operator::Mult) => acc * x,
            Some(Operator::Div) => acc / x,
            None => panic!("Operator not defined. Cannot perform operation."),
        })
    }
}

/// Parse the problem input and generate all operation structs (step 1).
///
/// # Returns
///
/// - `Vec<Operation>` - The vector of all operations comming from the file.
///
fn parse_content() -> Vec<Operation> {
    // Read content
    let content = fs::read_to_string(&FILE_PATH).expect("Error while reading the file");

    // REGEXES
    // match a number
    let re_digit = Regex::new(r"\d+").unwrap();
    // match an operator
    let re_operator = Regex::new(r"\+|-|\*|/").unwrap();

    let mut operations: Vec<Operation> = Vec::new();

    for (num, line) in content.split("\n").enumerate() {
        // Match all numbers in the line
        let numbers: Vec<i64> = re_digit
            .find_iter(&line)
            .map(|d| d.as_str().parse().unwrap())
            .collect();

        // Match all operators in the line
        let operators: Vec<&str> = re_operator.find_iter(&line).map(|d| d.as_str()).collect();

        if num == 0 {
            // First line
            // The vector must be initialized with initial operations
            for number in numbers {
                operations.push(Operation::init(number));
            }
        } else {
            // Other line
            // Simply fill-in operation data
            for (cnt, number) in numbers.iter().enumerate() {
                operations[cnt].add_num(*number);
            }
            for (cnt, operator) in operators.iter().enumerate() {
                operations[cnt].add_operator(*operator);
            }
        }
    }

    operations
}

/// Parse the digit part of the file and extract the numbers as columns.
///
/// # Arguments
///
/// - `lines` (`&[&str]`) - The digit lines.
///
/// # Returns
///
/// - `Vec<Vec<i64>>` - A vector containig vectors of numbers (one per operation).
///
fn get_column_numbers(lines: &[&str]) -> Vec<Vec<i64>> {
    if lines.is_empty() {
        return vec![];
    }

    let rows: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let height = rows.len();
    let width = rows[0].len();

    let mut result = Vec::with_capacity(width);
    let mut operation_numbers: Vec<i64> = Vec::new();

    for x in 0..width {
        let mut s = String::with_capacity(height);
        for y in 0..height {
            if rows[y][x] != ' ' {
                s.push(rows[y][x]);
            }
        }

        if s != "" {
            // This column is a number
            operation_numbers.push(s.parse().unwrap());
        } else {
            // This column is not a number
            result.push(operation_numbers.clone());
            operation_numbers = Vec::new()
        }
    }

    if operation_numbers.len() > 0 {
        result.push(operation_numbers.clone());
    }

    result
}

/// Parse the problem input and generate all operation structs (step 1).
///
/// # Returns
///
/// - `Vec<Operation>` - The vector of all operations comming from the file.
///
fn parse_content_fixed() -> Vec<Operation> {
    // Read content
    let content = fs::read_to_string(&FILE_PATH).expect("Error while reading the file");

    // match an operator
    let re_operator = Regex::new(r"\+|-|\*|/").unwrap();

    let mut operations: Vec<Operation> = Vec::new();

    // First split the two parts
    let lines: Vec<&str> = content.split("\n").collect();
    let operator_line = lines[lines.len() - 1];
    let digit_lines = &lines[0..lines.len() - 1];

    // Get operators
    let operators: Vec<&str> = re_operator
        .find_iter(&operator_line)
        .map(|d| d.as_str())
        .collect();

    // Get numbers
    let numbers = get_column_numbers(&digit_lines);

    // Create operation structs
    for i in 0..operators.len() {
        operations.push(Operation {
            nums: numbers[i].clone(),
            operator: Some(Operator::from(operators[i])),
        });
    }

    operations
}

/// Perform the operations and return their sum.
///
/// # Arguments
///
/// - `operations` (`&Vec<Operation>`) - The operations comming from the problem input.
///
/// # Returns
///
/// - `i64` - The sum of all operations
///
fn perform_operations(operations: &Vec<Operation>) -> i64 {
    let mut res = 0;

    for operation in operations {
        res += operation.compute();
    }

    res
}

/// The main program.
///
fn main() {
    let operations = parse_content();
    let res = perform_operations(&operations);
    println!("Result: {}", res);

    let operations = parse_content_fixed();
    let res = perform_operations(&operations);
    println!("Result: {}", res);
}
