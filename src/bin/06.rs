advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut puzzle: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.trim().split(" ").filter(|l| *l != "").collect())
        .collect();

    let operators = puzzle.pop().unwrap();
    let mut grand_total = 0;

    for len_line in 0..puzzle[0].len() {
        let mut line_value = 0;
        for line in &puzzle {
            let operator = operators[len_line];

            if operator == "*" {
                if line_value == 0 {
                    line_value = line[len_line].parse::<u64>().unwrap();
                } else {
                    line_value *= line[len_line].parse::<u64>().unwrap();
                }
            }

            if operator == "+" {
                line_value += line[len_line].parse::<u64>().unwrap();
            }
        }

        grand_total += line_value;
    }

    Some(grand_total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let width = lines.iter().map(|l| l.len()).max().unwrap();
    let get = |row: usize, col: usize| *lines[row].get(col).unwrap_or(&' ');

    let mut grand_total = 0u64;
    let mut col = width;

    while col > 0 {
        col -= 1;
        // Skip separator columns
        if (0..lines.len() - 1).all(|r| !get(r, col).is_ascii_digit()) {
            continue;
        }

        // Find operator and collect numbers for this problem
        let mut op = '+';
        let mut nums = vec![];

        while col > 0 || (0..lines.len() - 1).any(|r| get(r, col).is_ascii_digit()) {
            let num: String = (0..lines.len() - 1).filter_map(|r| {
                let c = get(r, col);
                c.is_ascii_digit().then_some(c)
            }).collect();

            if !num.is_empty() {
                nums.push(num.parse::<u64>().unwrap());
            }
            if get(lines.len() - 1, col) == '*' { op = '*'; }

            if col == 0 { break; }
            col -= 1;
            if (0..lines.len() - 1).all(|r| !get(r, col).is_ascii_digit()) { break; }
        }

        grand_total += if op == '+' { nums.iter().sum::<u64>() } else { nums.iter().product() };
    }

    Some(grand_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
