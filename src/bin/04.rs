advent_of_code::solution!(4);

const SIZE: usize = 140;

pub fn part_one(input: &str) -> Option<i32> {
    let mut count: i32 = 0;
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for (r, row) in matrix.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            if *char == 'X' {
                if ((c + 3) < SIZE
                    && matrix[r][c + 1] == 'M'
                    && matrix[r][c + 2] == 'A'
                    && matrix[r][c + 3] == 'S')
                {
                    count += 1;
                }
                if ((r + 3) < SIZE
                    && (c + 3) < SIZE
                    && matrix[r + 1][c + 1] == 'M'
                    && matrix[r + 2][c + 2] == 'A'
                    && matrix[r + 3][c + 3] == 'S')
                {
                    count += 1;
                }
                if ((r + 3) < SIZE
                    && matrix[r + 1][c] == 'M'
                    && matrix[r + 2][c] == 'A'
                    && matrix[r + 3][c] == 'S')
                {
                    count += 1;
                }
                if ((r + 3) < SIZE
                    && c > 2
                    && matrix[r + 1][c - 1] == 'M'
                    && matrix[r + 2][c - 2] == 'A'
                    && matrix[r + 3][c - 3] == 'S')
                {
                    count += 1;
                }
                if (c > 2
                    && matrix[r][c - 1] == 'M'
                    && matrix[r][c - 2] == 'A'
                    && matrix[r][c - 3] == 'S')
                {
                    count += 1;
                }
                if (r > 2
                    && c > 2
                    && matrix[r - 1][c - 1] == 'M'
                    && matrix[r - 2][c - 2] == 'A'
                    && matrix[r - 3][c - 3] == 'S')
                {
                    count += 1;
                }
                if (r > 2
                    && matrix[r - 1][c] == 'M'
                    && matrix[r - 2][c] == 'A'
                    && matrix[r - 3][c] == 'S')
                {
                    count += 1;
                }
                if (r > 2
                    && (c + 3) < SIZE
                    && matrix[r - 1][c + 1] == 'M'
                    && matrix[r - 2][c + 2] == 'A'
                    && matrix[r - 3][c + 3] == 'S')
                {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut count: i32 = 0;
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for (r, row) in matrix.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            if *char == 'A'
                && r > 0
                && c > 0
                && r < SIZE - 1
                && c < SIZE - 1
                && ((matrix[r - 1][c - 1] == 'M'
                    && matrix[r - 1][c + 1] == 'M'
                    && matrix[r + 1][c - 1] == 'S'
                    && matrix[r + 1][c + 1] == 'S')
                    || (matrix[r - 1][c - 1] == 'S'
                        && matrix[r - 1][c + 1] == 'S'
                        && matrix[r + 1][c - 1] == 'M'
                        && matrix[r + 1][c + 1] == 'M')
                    || (matrix[r - 1][c - 1] == 'S'
                        && matrix[r + 1][c - 1] == 'S'
                        && matrix[r - 1][c + 1] == 'M'
                        && matrix[r + 1][c + 1] == 'M')
                    || (matrix[r - 1][c - 1] == 'M'
                        && matrix[r + 1][c - 1] == 'M'
                        && matrix[r - 1][c + 1] == 'S'
                        && matrix[r + 1][c + 1] == 'S'))
            {
                count += 1;
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
