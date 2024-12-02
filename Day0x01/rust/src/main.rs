use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;
use std::str::SplitWhitespace;

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Should have been able to read file");
}

fn parse_line(line: &str) -> (u32, u32) {
    let mut line_split: SplitWhitespace = line.split_whitespace();
    let left_id: u32 = line_split
        .next()
        .expect("Should have been able to read left id")
        .parse::<u32>()
        .expect("Should have been able to parse left id to u32");
    let right_id: u32 = line_split
        .next()
        .expect("Should have been able to read right id")
        .parse::<u32>()
        .expect("Should have been able to parse right id to u32");

    return (left_id, right_id);
}

fn abs_diff(n1: u32, n2: u32) -> u32 {
    if n1 > n2 {
        return n1 - n2;
    }

    return n2 - n1;
}

fn main() {
    let file_path: &str = "../lists.txt";
    let file_contents: String = read_file(file_path);

    let mut left_heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    let mut right_heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    let mut left_counts: HashMap<u32, u32> = HashMap::new();
    let mut right_counts: HashMap<u32, u32> = HashMap::new();

    for line in file_contents.lines() {
        let (left_id, right_id) = parse_line(line);
        left_heap.push(Reverse(left_id));
        right_heap.push(Reverse(right_id));
    }

    let mut total_distance: u32 = 0;
    while let (Some(Reverse(left_id)), Some(Reverse(right_id))) =
        (left_heap.pop(), right_heap.pop())
    {
        total_distance += abs_diff(left_id, right_id);

        left_counts
            .entry(left_id)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        right_counts
            .entry(right_id)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut similarity_score: u32 = 0;
    for (left_id, left_count) in &left_counts {
        let right_count = right_counts.get(left_id).unwrap_or(&0);
        similarity_score += left_id * left_count * right_count;
    }

    println!("Part 1: {total_distance}");
    println!("Part 2: {similarity_score}");
}
