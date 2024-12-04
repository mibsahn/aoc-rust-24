use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i32> {
    let mut sum = 0;
    let re = Regex::new(r"mul[(]\d{1,3}[,]\d{1,3}[)]").unwrap();
    for mat in re.find_iter(input) {
        let num_re = Regex::new(r"\d{1,3}").unwrap();
        let mut mul = 1;
        for num_mat in num_re.find_iter(mat.as_str()) {
            mul *= num_mat.as_str().parse::<i32>().unwrap();
        }
        sum += mul;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut sum = 0;

    let re_mul = Regex::new(r"mul[(]\d{1,3}[,]\d{1,3}[)]").unwrap();
    let re_do = Regex::new(r"do[()][)]").unwrap();
    let re_dont = Regex::new(r"don[']t[()][)]").unwrap();

    let mut start_pos = 0;

    loop {
        let mut mul_start = start_pos;
        while let Some(mul_mat) = re_mul.find_at(input, mul_start) {
            if let Some(dont_mat) = re_dont.find_at(input, mul_start) {
                if dont_mat.start() < mul_mat.start() {
                    start_pos = dont_mat.end();
                    break;
                }
            }
            let mut mul = 1;
            let num_re = Regex::new(r"\d{1,3}").unwrap();
            for num_mat in num_re.find_iter(mul_mat.as_str()) {
                mul *= num_mat.as_str().parse::<i32>().unwrap();
            }
            sum += mul;
            mul_start = mul_mat.end();
        }
        if let Some(do_mat) = re_do.find_at(input, start_pos) {
            start_pos = do_mat.end();
        } else {
            break;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
