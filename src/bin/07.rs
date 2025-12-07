advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let puzzle: Vec<Vec<&str>> = input.lines().map(|l| l.split("").collect()).collect();
    let mut split_times = 0;

    let beam_start_pos = puzzle[0].iter().position(|c| *c == "S").unwrap();

    println!("{}", beam_start_pos);

    let mut beams_pos: Vec<usize> = Vec::new();

    beams_pos.push(beam_start_pos);

    for line in puzzle {
        let splitter_pos: Vec<usize> = line
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == "^")
            .map(|(i, _)| i)
            .collect();

        //println!("splitters {:?}, beams: {:?}", splitter_pos, beams_pos);

        if splitter_pos.len() == 0 {
            continue;
        }

        let mut new_beams = Vec::new();

        for &beam_pos in &beams_pos {
            if splitter_pos.contains(&beam_pos) {
                let mut beam_has_hit = false;
                // Split into two new beams

                let potential_right_beam = beam_pos.checked_add(1).unwrap();
                let potenttial_left_beam = beam_pos.checked_sub(1).unwrap();

                if new_beams.contains(&potential_right_beam) == false {
                    new_beams.push(potential_right_beam);
                    beam_has_hit = true;
                }

                if new_beams.contains(&potenttial_left_beam) == false {
                    new_beams.push(potenttial_left_beam);
                    beam_has_hit = true;
                }

                if beam_has_hit {
                    split_times += 1;
                }
            } else {
                new_beams.push(beam_pos);
            }
        }

        beams_pos = new_beams;
    }

    beams_pos.sort();
    beams_pos.dedup();

    //println!("splitters {:?}, beams: {:?}", "", beams_pos);
    Some(split_times)
}

pub fn part_two(input: &str) -> Option<u64> {
    let puzzle: Vec<Vec<&str>> = input.lines().map(|l| l.split("").collect()).collect();

    let height = puzzle.len();
    if height == 0 {
        return Some(0);
    }
    let width = puzzle[0].len();

    let start_col = puzzle[0].iter().position(|c| *c == "S")?;

    let mut dp = vec![vec![0u64; width]; height + 1];

    for col in 0..width {
        dp[height][col] = 1;
    }

    for row in (0..height).rev() {
        for col in 0..width {
            let cell = puzzle[row][col];

            if cell == "S" || cell == "." || cell == "" {
                dp[row][col] = dp[row + 1][col];
            } else if cell == "^" {
                let mut total = 0u64;

                if col == 0 {
                    total += 1;
                } else {
                    total += dp[row + 1][col - 1];
                }

                if col + 1 >= width {
                    total += 1;
                } else {
                    total += dp[row + 1][col + 1];
                }

                dp[row][col] = total;
            } else {
                dp[row][col] = dp[row + 1][col];
            }
        }
    }

    Some(dp[0][start_col])
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
