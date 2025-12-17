use crate::machine::Machine;
use regex::Regex;
use std::{collections::HashSet, fs};

pub fn parse_line(line: &str) -> Machine {
    let re = Regex::new(r"\[([.#]+)\] ((?:\([\d,]+\) )+)\{([\d,]+)\}").unwrap();
    let caps = re.captures(line).unwrap();

    let light_pattrn: Vec<bool> = caps[1].chars().map(|c| c == '#').collect();
    let buttons: Vec<HashSet<usize>> = caps[2]
        .trim()
        .split(" ")
        .map(|s| {
            s[1..s.len() - 1]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    let joltage: Vec<i32> = caps[3].split(",").map(|s| s.parse().unwrap()).collect();

    Machine {
        light_pattrn: light_pattrn.clone(),
        buttons: buttons.clone(),
        joltage: joltage.clone(),
    }
}

pub fn parse_file(file_input: &str) -> Vec<Machine> {
    fs::read_to_string(file_input)
        .expect("Failed to read file")
        .lines()
        .map(|line| parse_line(&line))
        .collect()
}
