use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ls_1: Vec<u32> = Vec::new();
    let mut ls_2: Vec<u32> = Vec::new();
    for line in input.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();
        ls_1.push(v[0].parse().expect("Non number detected"));
        ls_2.push(v[1].parse().expect("Non number detected"));
    }
    ls_1.sort();
    ls_2.sort();
    let mut sum: u32 = 0;
    for (a, b) in ls_1.iter().zip(ls_2.iter()) {
        sum += a.abs_diff(*b);
    }
    dbg!(sum);
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ls_1: Vec<u32> = Vec::new();
    let mut ls_2: Vec<u32> = Vec::new();
    for line in input.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();
        ls_1.push(v[0].parse().expect("Non number detected"));
        ls_2.push(v[1].parse().expect("Non number detected"));
    }
    let mut ls_1_counts: HashMap<u32, u32> = HashMap::new();
    for &item in &ls_1 {
        *ls_1_counts.entry(item).or_insert(0) += 1;
    }
    let mut sum: u32 = 0;
    for (key, value) in ls_1_counts {
        sum += key * value * ls_2.iter().filter(|&&x| x == key).count() as u32;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 11);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 31);
    }
}
