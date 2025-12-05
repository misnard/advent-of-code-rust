advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let puzzle_parts: Vec<&str> = input.split("\n\n").collect();

    let ranges: Vec<(i64, i64)> = puzzle_parts[0]
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|r| {
            let r_parts: Vec<&str> = r.split("-").collect();
            let start_r = r_parts[0].parse::<i64>().expect("Not a valid number");
            let end_r = r_parts[1].parse::<i64>().expect("Not a valid number");

            (start_r, end_r)
        })
        .collect();

    let food_ids = puzzle_parts[1].lines().collect::<Vec<&str>>();

    let mut good_food_count = 0;

    for food_id in food_ids {
        for (start_range, end_range) in &ranges {
            let food_id_comp = food_id.parse::<i64>().expect("Not a valid number");

            if food_id_comp >= *start_range && food_id_comp <= *end_range {
                good_food_count += 1;
                break;
            }
        }
    }

    Some(good_food_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let puzzle_parts: Vec<&str> = input.split("\n\n").collect();

    let mut ranges: Vec<(i64, i64)> = puzzle_parts[0]
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|r| {
            let r_parts: Vec<&str> = r.split("-").collect();
            let start_r = r_parts[0].parse::<i64>().expect("Not a valid number");
            let end_r = r_parts[1].parse::<i64>().expect("Not a valid number");

            (start_r, end_r)
        })
        .collect();

    // Sort ranges by start value
    ranges.sort_by_key(|r| r.0);

    // Merge overlapping ranges and count unique values
    let mut merged: Vec<(i64, i64)> = Vec::new();

    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            // If current range overlaps or is adjacent to the last merged range
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    // Count total unique values across all merged ranges
    let total: i64 = merged.iter().map(|(s, e)| e - s + 1).sum();

    Some(total as u64)
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
