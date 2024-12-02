advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut count: u32 = 0;
    'line: for line in input.lines() {
        let report: Vec<&str> = line.split_whitespace().collect();
        let report: Vec<u32> = report
            .iter()
            .filter_map(|&s| s.parse::<u32>().ok())
            .collect();
        let is_increasing = report[1] > report[0];
        for window in report.windows(2) {
            if (is_increasing && window[1] < window[0])
                || (!is_increasing && window[1] > window[0])
                || (window[1].abs_diff(window[0]) < 1)
                || (window[1].abs_diff(window[0]) > 3)
            {
                continue 'line;
            }
        }
        count += 1;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count: u32 = 0;
    'line: for line in input.lines() {
        let report: Vec<&str> = line.split_whitespace().collect();
        let report: Vec<i32> = report
            .iter()
            .filter_map(|&s| s.parse::<i32>().ok())
            .collect();
        let is_increasing = report[1] > report[0];
        for window in report.windows(2) {
            if (is_increasing && window[1] < window[0])
                || (!is_increasing && window[1] > window[0])
                || (window[1].abs_diff(window[0]) < 1)
                || (window[1].abs_diff(window[0]) > 3)
            {
                let differences: Vec<i32> =
                    report.windows(2).map(|pair| pair[1] - pair[0]).collect();
                for window in differences.windows(differences.len() - 1) {
                    if (window.iter().all(|&x| x > 0) || window.iter().all(|&x| x < 0))
                        && window.iter().all(|&x| x.abs() > 0 && x.abs() < 4)
                    {
                        count += 1;
                        continue 'line;
                    }
                }
                for i in 0..differences.len() - 1 {
                    let mut dampener = differences.clone();
                    dampener[i] += dampener[i + 1];
                    dampener.remove(i + 1);
                    if (dampener.iter().all(|&x| x > 0) || dampener.iter().all(|&x| x < 0))
                        && dampener.iter().all(|&x| x.abs() > 0 && x.abs() < 4)
                    {
                        count += 1;
                        continue 'line;
                    }
                }
                continue 'line;
            }
        }
        count += 1;
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
