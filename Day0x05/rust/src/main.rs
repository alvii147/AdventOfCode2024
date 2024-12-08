use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Should have been able to read file");
}

fn split_file_contents(file_contents: &String) -> (&str, &str) {
    let split_contents: Vec<&str> = file_contents.split("\n\n").collect();
    let rules: &str = split_contents
        .get(0)
        .expect("Should have been able to parse rules section");
    let updates: &str = split_contents
        .get(1)
        .expect("Should have been able to parse updates section");

    return (rules, updates);
}

fn parse_rules(rules: &str) -> HashMap<&str, HashSet<&str>> {
    let mut rules_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for rule in rules.split_whitespace() {
        let split_rule: Vec<&str> = rule.split('|').collect();
        let left_page_num: &str = split_rule
            .get(0)
            .expect("Should have been able to parse left page number");
        let right_page_num: &str = split_rule
            .get(1)
            .expect("Should have been able to parse right page number");
        rules_map
            .entry(left_page_num)
            .and_modify(|s| _ = s.insert(right_page_num))
            .or_insert(HashSet::from([right_page_num]));
    }

    return rules_map;
}

fn compare_page_numbers(p1: &str, p2: &str, rules: &HashMap<&str, HashSet<&str>>) -> Ordering {
    if let Some(pages_after_p1) = rules.get(p1) {
        if pages_after_p1.contains(p2) {
            return Ordering::Less;
        }
    }

    if let Some(pages_after_p2) = rules.get(p2) {
        if pages_after_p2.contains(p1) {
            return Ordering::Greater;
        }
    }

    return Ordering::Equal;
}

fn is_update_ordered(update: &Vec<&str>, rules: &HashMap<&str, HashSet<&str>>) -> bool {
    let mut prev: Option<&str> = None;
    for curr_page in update {
        if let Some(prev_page) = prev {
            if compare_page_numbers(prev_page, curr_page, rules) != Ordering::Less {
                return false;
            }
        }

        prev = Some(curr_page);
    }

    return true;
}

fn get_middle_value<T>(v: &Vec<T>) -> &T {
    return v
        .get(v.len() / 2)
        .expect("Should have been able to get middle value");
}

fn main() {
    let file_path: &str = "../pages.txt";
    let file_contents: String = read_file(file_path);
    let (rules, updates): (&str, &str) = split_file_contents(&file_contents);

    let rules_map: HashMap<&str, HashSet<&str>> = parse_rules(rules);
    let mut sum_ordered: u32 = 0;
    let mut sum_unordered: u32 = 0;

    for update_str in updates.split_whitespace() {
        let mut update: Vec<&str> = update_str.split(',').collect();
        if is_update_ordered(&update, &rules_map) {
            let middle_value: &str = *get_middle_value(&update);
            sum_ordered += middle_value
                .parse::<u32>()
                .expect("Should have been able to parse middle value");
        } else {
            update.sort_by(|p1, p2| compare_page_numbers(p1, p2, &rules_map));
            let middle_value: &str = *get_middle_value(&update);
            sum_unordered += middle_value
                .parse::<u32>()
                .expect("Should have been able to parse middle value");
        }
    }

    println!("Part 1: {sum_ordered}");
    println!("Part 2: {sum_unordered}");
}
