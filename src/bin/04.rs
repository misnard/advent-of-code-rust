advent_of_code::solution!(4);

fn check_neightboor(puzzle: &[Vec<&str>], line_c: i32, col_c: i32) -> i32 {
    let puzzle_height_border = puzzle.len();
    let puzzle_line_border = puzzle[0].len();

    let mut start_p_l: i32 = -1;
    let mut start_p_h: i32 = -1;

    let mut neightboor_count = 0;

    for _ in 0..9 {
        let mut is_inbound = true;
        if line_c + start_p_h < 0 || line_c + start_p_h >= puzzle_height_border as i32 {
            is_inbound = false; //skip when out of bounds height
        }

        if col_c + start_p_l < 0 || col_c + start_p_l >= puzzle_line_border as i32 {
            is_inbound = false; //skip when out of bounds lenght
        }

        if is_inbound == true
            && puzzle[(line_c + start_p_h) as usize][(col_c + start_p_l) as usize] == "@"
        {
            neightboor_count += 1
        }

        start_p_l += 1;

        if start_p_l == 2 {
            start_p_l = -1;
            start_p_h += 1;
        }
    }

    neightboor_count
}

pub fn part_one(input: &str) -> Option<u64> {
    let puzzle: Vec<Vec<&str>> = input.lines().map(|l| l.split("").collect()).collect();

    let mut accessible_rolls = 0;

    for line_c in 0..puzzle.len() {
        for col_c in 0..puzzle[line_c as usize].len() {
            let current_pos = puzzle[line_c][col_c];

            if current_pos == "@" {
                if check_neightboor(&puzzle, line_c as i32, col_c as i32) <= 4 {
                    accessible_rolls += 1
                }
            }
        }
    }

    Some(accessible_rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut puzzle: Vec<Vec<&str>> = input.lines().map(|l| l.split("").collect()).collect();

    let mut accessible_rolls = 0;

    loop {
        let mut removed_any = false;
        for line_c in 0..puzzle.len() {
            for col_c in 0..puzzle[line_c as usize].len() {
                let current_pos = puzzle[line_c][col_c];

                if current_pos == "@" {
                    if check_neightboor(&puzzle, line_c as i32, col_c as i32) <= 4 {
                        accessible_rolls += 1;
                        puzzle[line_c][col_c] = ".";
                        removed_any = true;
                    }
                }
            }
        }

        if !removed_any {
            break;
        }
    }

    Some(accessible_rolls)
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
