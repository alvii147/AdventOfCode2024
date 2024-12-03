use std::fs;

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Should have been able to read file");
}

struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut prev_inc_or_dec: Option<bool> = None;
        let mut prev_level: Option<u32> = None;
        for &level in self.levels.iter() {
            if let Some(pl) = prev_level {
                let inc_or_dec: bool = pl < level;
                let diff: u32;
                if inc_or_dec {
                    diff = level - pl;
                } else {
                    diff = pl - level;
                }

                if (inc_or_dec && diff > 3) || (!inc_or_dec && diff > 3) {
                    return false;
                }

                if (inc_or_dec && diff < 1) || (!inc_or_dec && diff < 1) {
                    return false;
                }

                if let Some(pi) = prev_inc_or_dec {
                    if (inc_or_dec && !pi) || (!inc_or_dec && pi) {
                        return false;
                    }
                }

                prev_inc_or_dec = Some(inc_or_dec);
            }

            prev_level = Some(level);
        }

        return true;
    }
}

fn main() {
    let file_path: &str = "../reports.txt";
    let file_contents: String = read_file(file_path);

    let mut n_safe_reports: u32 = 0;
    let mut n_damped_safe_reports: u32 = 0;

    for line in file_contents.lines() {
        let report: Report = Report {
            levels: line
                .split_whitespace()
                .map(|l| l.parse::<u32>().unwrap())
                .collect(),
        };

        if report.is_safe() {
            n_safe_reports += 1;
        }

        for damped_level_index in 0..report.levels.len() {
            let damped_report: Report = Report {
                levels: report
                    .levels
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != damped_level_index)
                    .map(|(_, l)| *l)
                    .collect(),
            };

            if damped_report.is_safe() {
                n_damped_safe_reports += 1;
                break;
            }
        }
    }

    println!("Part 1: {n_safe_reports}");
    println!("Part 2: {n_damped_safe_reports}");
}
