use nalgebra::DMatrix;
use std::fs;

const FILE_PATH: &str = "input.txt";

fn update_line(
    row: usize,
    current_line: &str,
    previous_line: &str,
    matrix: &mut DMatrix<i64>,
) -> (String, i32) {
    let mut res: Vec<char> = current_line.chars().collect();
    let prev: Vec<char> = previous_line.chars().collect();

    let mut cnt = 0;

    for (i, &pc) in prev.iter().enumerate() {
        match pc {
            'S' => {
                res[i] = '|';
                matrix[(row, i)] += 1;
            }
            '|' => {
                if res[i] != '^' {
                    // propagate straight
                    res[i] = '|';
                    matrix[(row, i)] += matrix[(row - 1, i)];
                } else {
                    // splitter
                    cnt += 1;

                    if i > 0 {
                        res[i - 1] = '|';
                        matrix[(row, i - 1)] += matrix[(row - 1, i)];
                    }
                    if i + 1 < res.len() {
                        res[i + 1] = '|';
                        matrix[(row, i + 1)] += matrix[(row - 1, i)];
                    }
                }
            }
            _ => {}
        }
    }

    (res.iter().collect(), cnt)
}

fn process() -> (String, i32, i64) {
    let mut lines: Vec<String> = fs::read_to_string(FILE_PATH)
        .expect("Failed to open file")
        .lines() // handles \n and \r\n, no empty last line
        .map(|s| s.to_string())
        .collect();

    let mut result = 0;
    let mut matrix = DMatrix::from_element(lines.len(), lines[0].len(), 0);

    for i in 1..lines.len() {
        let (new_line, cnt) = update_line(i, &lines[i], &lines[i - 1], &mut matrix);
        lines[i] = new_line;
        result += cnt;
    }

    (
        lines.join("\n"),
        result,
        matrix.row(matrix.nrows() - 1).sum(),
    )
}

fn main() {
    let (lines, cnt, cnt_bis) = process();

    println!("{}", lines);
    println!("The beam is splitted {} times", cnt);
    println!("The number of paths {}", cnt_bis);
}
