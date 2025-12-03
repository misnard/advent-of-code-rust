advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let number_ranges = input.split(",");

    let mut total_result = 0;

    for number_range in number_ranges {
        let mut range_min_max = number_range.split("-").into_iter();
        let min = range_min_max.next().unwrap().parse::<u64>().unwrap();
        let max = range_min_max.next().unwrap().parse::<u64>().unwrap();

        for i in min..max {
            let num_str = i.to_string();
            let num_len = num_str.len();

            if num_len % 2 == 0 {
                let first_pattern = num_str[0..(num_len / 2)].to_string();
                let second_pattern = num_str[(num_len / 2)..num_len].to_string();

                if first_pattern == second_pattern {
                    total_result = total_result + i;
                }
            }
        }
    }

    Some(total_result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let cleaned = input.replace("\n", "");
    let number_ranges = cleaned.split(",");

    let mut total_result = 0;

    for number_range in number_ranges {
        if number_range.is_empty() {
            continue;
        }
        let mut range_min_max = number_range.split("-").into_iter();
        let min = range_min_max.next().unwrap().parse::<u64>().unwrap();
        let max = range_min_max.next().unwrap().parse::<u64>().unwrap();

        for i in min..=max {
            let num_str = i.to_string();
            let num_len = num_str.len();

            for p in 1..num_len {
                if num_len % p != 0 {
                    continue;
                }

                let repeats = num_len / p;
                let block = &num_str[..p];
                if block.repeat(repeats) == num_str {
                    total_result = total_result + i;
                    break;
                }
            }
        }
    }

    Some(total_result)
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
        assert_eq!(result, Some(4174379265));
    }
}
