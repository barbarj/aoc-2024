use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(dead_code)]
fn parse_input(filename: &str) -> Vec<Vec<i32>> {
    let file = File::open("input/2/".to_owned() + filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_valid_window(window: &[i32], ascending: bool) -> bool {
    assert_eq!(window.len(), 3);
    let direction = (ascending && window[0] < window[1] && window[1] < window[2])
        || (!ascending && window[0] > window[1] && window[1] > window[2]);
    let magnitude = (1..=3).contains(&window[0].abs_diff(window[1]))
        && (1..=3).contains(&window[1].abs_diff(window[2]));
    direction && magnitude
}

fn is_report_safe_when_removing_i(report: &[i32], mut dampened: bool, to_remove: usize) -> bool {
    assert!(report.len() > 2);
    assert!(to_remove < 3);
    let mut window: Vec<i32> = report.iter().take(3).copied().collect();
    let mut ascending = window[0] < window[1];
    if !is_valid_window(&window, ascending) {
        if dampened {
            dampened = false;
            window.remove(to_remove);
            ascending = window[0] < window[1];
        } else {
            return false;
        }
    } else {
        window.remove(0);
    }
    for level in report.iter().skip(3) {
        window.push(*level);
        if !is_valid_window(&window, ascending) {
            if dampened {
                dampened = false;
                window.remove(to_remove);
            } else {
                return false;
            }
        } else {
            window.remove(0);
        }
    }
    (1..=3).contains(&window[0].abs_diff(window[1]))
}

#[allow(dead_code)]
fn is_report_safe(report: &[i32], dampened: bool) -> bool {
    is_report_safe_when_removing_i(report, dampened, 0)
        || is_report_safe_when_removing_i(report, dampened, 1)
        || is_report_safe_when_removing_i(report, dampened, 2)
}

#[allow(dead_code)]
fn count_safe_reports(filename: &str, dampened: bool) -> usize {
    let input = parse_input(filename);
    input
        .iter()
        .filter(|report| is_report_safe(report, dampened))
        .count()
}

#[cfg(test)]
mod tests {
    use super::{count_safe_reports, is_report_safe, parse_input};

    #[test]
    fn part1_example() {
        let result = count_safe_reports("example.txt", false);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1() {
        let result = count_safe_reports("input.txt", false);
        assert_eq!(result, 230);
    }

    #[test]
    fn part2_example() {
        let result = count_safe_reports("example.txt", true);
        assert_eq!(result, 4);
    }

    #[test]
    fn print_unsafe() {
        let input = parse_input("input.txt");
        let is_unsafe = input.iter().filter(|report| !is_report_safe(report, true));
        for x in is_unsafe.skip(200).take(5) {
            println!("{x:?}");
        }
        //panic!("just to test");
    }

    #[test]
    fn print_safe() {
        let input = parse_input("input.txt");
        let safe = input
            .iter()
            .filter(|report| is_report_safe(report, true))
            .filter(|report| !is_report_safe(report, false));
        for x in safe.skip(5).take(5) {
            println!("{x:?}");
        }
        //panic!("just to test");
    }

    #[test]
    fn unsafe_edge_cases() {
        let report = vec![69, 67, 66, 67, 70, 71, 74, 75];
        assert!(!is_report_safe(&report, true));
    }

    #[test]
    fn part2() {
        let result = count_safe_reports("input.txt", true);
        assert_eq!(result, 301);
    }

    #[test]
    fn part2_more_valid() {
        let result = count_safe_reports("more_ex.txt", true);
        assert_eq!(result, 12);
    }
}
