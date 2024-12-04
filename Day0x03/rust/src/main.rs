use regex::{Captures, Regex};
use std::fs;

struct Multiplier {
    start: usize,
    a: u32,
    b: u32,
}

struct Flipper {
    start: usize,
    enable: bool,
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Should have been able to read file");
}

fn compute_mul(memory: String) -> u32 {
    let re_mul: Regex = Regex::new(r"mul\(\d+,\d+\)").expect("Regex pattern should have compiled");
    let re_mul_digits: Regex =
        Regex::new(r"mul\((\d+),(\d+)\)").expect("Regex pattern should have compiled");
    let re_flipper: Regex =
        Regex::new(r"do(?:n't)?\(\)").expect("Regex pattern should have compiled");

    let mut sum: u32 = 0;

    let mut multipliers: Vec<Multiplier> = Vec::new();
    let mut flippers: Vec<Flipper> = Vec::new();

    for mul in re_mul.find_iter(memory.as_str()) {
        let cap: Captures = re_mul_digits
            .captures(mul.as_str())
            .expect("Regex should have captured multiplier digits");
        let a: u32 = cap
            .get(1)
            .expect("Should have captured first digit")
            .as_str()
            .parse::<u32>()
            .expect("Should have parsed first digit");
        let b: u32 = cap
            .get(2)
            .expect("Should have captured second digit")
            .as_str()
            .parse::<u32>()
            .expect("Should have parsed second digit");
        multipliers.push(Multiplier {
            start: mul.start(),
            a,
            b,
        });
    }

    for fl in re_flipper.find_iter(memory.as_str()) {
        let enable: bool;
        if fl.as_str() == "do()" {
            enable = true;
        } else {
            enable = false;
        }

        flippers.push(Flipper {
            start: fl.start(),
            enable,
        });
    }

    let mut enabled: bool = true;
    let mut multipliers_idx: usize = 0;
    let mut flippers_idx: usize = 0;
    while multipliers_idx < multipliers.len() && flippers_idx < flippers.len() {
        let multiplier: &Multiplier = multipliers
            .get(multipliers_idx)
            .expect("Should have been able to index multipliers");
        let flipper: &Flipper = flippers
            .get(flippers_idx)
            .expect("Should have been able to index flippers");

        if flipper.start < multiplier.start {
            flippers_idx += 1;
            enabled = flipper.enable;
            continue;
        }

        multipliers_idx += 1;
        if enabled {
            sum += multiplier.a * multiplier.b;
        }
    }

    while enabled && multipliers_idx < multipliers.len() {
        let multiplier: &Multiplier = multipliers
            .get(multipliers_idx)
            .expect("Should have been able to index multipliers");
        multipliers_idx += 1;
        sum += multiplier.a * multiplier.b;
    }

    return sum;
}

fn main() {
    let file_path: &str = "../memory.txt";
    let file_contents: String = read_file(file_path);
    let sum: u32 = compute_mul(file_contents);

    println!("Part 1: {sum}");
}
