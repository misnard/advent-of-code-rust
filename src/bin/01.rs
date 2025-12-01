advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();

    let mut hitted_zero_times = 0;
    let mut current_state: i32 = 50;

    for line in lines {
        let movement_direction = &line[0..1];
        let movement_count: i32 = line[1..].parse().unwrap();

        match movement_direction {
            "L" => {
                current_state = (current_state - movement_count) % 100;
            }
            "R" => {
                current_state = (movement_count + current_state) % 100;
            }
            _ => {
                panic!("Nein")
            }
        }

        if current_state == 0 {
            hitted_zero_times += 1
        }
    }

    Some(hitted_zero_times)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();

    let mut hitted_zero_times: u64 = 0;
    let mut current_state: i64 = 50;

    for line in lines {
        let movement_direction = &line[0..1];
        let movement_count: i64 = line[1..].parse().unwrap();

        match movement_direction {
            "L" => {
                let first_hit = if current_state == 0 {
                    100
                } else {
                    current_state
                };
                let zeros = if movement_count >= first_hit {
                    1 + (movement_count - first_hit) / 100
                } else {
                    0
                };
                hitted_zero_times += zeros as u64;

                current_state = ((current_state - movement_count) % 100 + 100) % 100;
            }
            "R" => {
                let first_hit = if current_state == 0 {
                    100
                } else {
                    100 - current_state
                };
                let zeros = if movement_count >= first_hit {
                    1 + (movement_count - first_hit) / 100
                } else {
                    0
                };
                hitted_zero_times += zeros as u64;

                current_state = (current_state + movement_count) % 100;
            }
            _ => {
                panic!("Nein")
            }
        }
    }

    Some(hitted_zero_times)
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
