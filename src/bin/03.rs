advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut batteries_total = 0;

    for batterie in input.lines() {
        let batterie_values = batterie
            .split("")
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().expect("invalid number"));

        let mut first_highest = 0;
        let mut second_highest = 0;

        for (k, value) in batterie_values.enumerate() {
            let parsed_value = value;

            if parsed_value > first_highest && k + 1 < batterie.len() {
                first_highest = parsed_value;
                second_highest = 0;
            } else if parsed_value > second_highest {
                second_highest = parsed_value;
            }
        }

        println!("{}{}", first_highest, second_highest);
        batteries_total = batteries_total
            + format!("{}{}", first_highest, second_highest)
                .parse::<u64>()
                .unwrap()
    }

    Some(batteries_total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut batteries_total: u64 = 0;
    let pack_size: usize = 12;

    for line in input.lines() {
        let digits: Vec<char> = line.chars().collect();
        let n = digits.len();

        let mut result = String::new();
        let mut start = 0;

        for i in 0..pack_size {
            let remaining_needed = pack_size - i - 1;
            let end = n - remaining_needed;

            let mut best_idx = start;
            let mut best_digit = digits[start];

            for j in start..end {
                if digits[j] > best_digit {
                    best_digit = digits[j];
                    best_idx = j;
                }
            }

            result.push(best_digit);
            start = best_idx + 1;
        }

        let joltage: u64 = result.parse().unwrap();
        batteries_total += joltage;
    }

    Some(batteries_total)
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
        assert_eq!(result, None);
    }
}
