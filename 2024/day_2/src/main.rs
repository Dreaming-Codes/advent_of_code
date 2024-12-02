use std::io::Read;

fn check_diffs(line: &[i32]) -> Vec<i8> {
    let mut results = vec![];

    if line.len() < 2 {
        return results;
    }

    for window in line.windows(2) {
        let diff = window[1] - window[0];
        results.push(diff as i8);
    }

    results
}

fn is_safe(line: &[i8]) -> bool {
    let (increasing, decreasing, other) = line.iter().fold((0, 0, 0), |(inc, dec, oth), &diff| {
        if (1..=3).contains(&diff) {
            (inc + 1, dec, oth)
        } else if (-3..=-1).contains(&diff) {
            (inc, dec + 1, oth)
        } else {
            (inc, dec, oth + 1)
        }
    });

    other == 0 && (increasing == 0 || decreasing == 0)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("Invalid integer"))
                .collect()
        })
        .collect();

    let results: Vec<Vec<i8>> = reports.iter().map(|line| check_diffs(line)).collect();

    let safe_part1 = results.iter().filter(|line| is_safe(line)).count();
    println!("Safe reports (Part 1): {:?}", safe_part1);

    let safe_part2 = reports
        .iter()
        .filter(|line| {
            let diffs = check_diffs(line);
            if is_safe(&diffs) {
                return true;
            }

            for i in 0..line.len() {
                if i > 0 && i < line.len() - 1 {
                    let mut modified_diffs = diffs.clone();
                    modified_diffs[i - 1] = (line[i + 1] - line[i - 1]) as i8;
                    modified_diffs.remove(i);
                    if is_safe(&modified_diffs) {
                        return true;
                    }
                } else if i == 0 || i == line.len() - 1 {
                    let mut modified_diffs = diffs.clone();
                    modified_diffs.remove(i.min(diffs.len() - 1));
                    if is_safe(&modified_diffs) {
                        return true;
                    }
                }
            }
            false
        })
        .count();

    println!("Safe reports with dampener (Part 2): {:?}", safe_part2);
}
