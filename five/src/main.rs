use std::fs;

fn get_span(lower: i32, upper: i32, letter: u8) -> Result<(i32, i32), &'static str> {
    if letter == b'B' || letter == b'R'  {
        return Ok(find_next_upper(lower, upper))
    } else if letter == b'F' || letter == b'L' {
        return Ok(find_next_lower(lower, upper))
    }
    Err("Invalid letter")
}

fn find_next_lower(lower: i32, upper: i32) -> (i32, i32) {
    let half = (upper - lower) / 2 as i32;
    (lower, upper - half - 1)
}

fn find_next_upper(lower: i32, upper: i32) -> (i32, i32) {
    let half = (upper - lower) / 2 as i32;
    (lower + half + 1, upper)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error occurerd reading input");

    let boarding_passes: Vec<&str> = input.split("\n")
                                            .collect();

    let mut row = 0;
    let mut col = 0;
    let mut seat_ids: Vec<i32> = Vec::new();
    for (i, &b_pass) in boarding_passes.iter().enumerate() {
        let mut upper_row: i32 = 127;
        let mut lower_row: i32 = 0;
        let mut lower_col: i32 = 0;
        let mut upper_col: i32 = 7;
        for (j, &letter) in b_pass.as_bytes().iter().enumerate() {
            if j <= 6 {
                match get_span(lower_row, upper_row, letter) {
                    Ok(s) => {
                        upper_row = s.1;
                        lower_row = s.0;
                    },
                    Err(e) => println!("Error {}", e)
                }
                if lower_row == upper_row {
                    row = lower_row;
                }
            } else if j <= 9 {
                match get_span(lower_col, upper_col, letter) {
                    Ok(s) => {
                        upper_col = s.1;
                        lower_col= s.0;
                    },
                    Err(e) => println!("Error {}", e)
                }
                if lower_col == upper_col {
                    col = lower_col;
                }
            }
        }
        seat_ids.push(8*row + col);
    }

    seat_ids.sort();

    let mut my_seat = 0;
    for (i, s_id) in seat_ids.iter().enumerate() {
        if (i < seat_ids.len() - 1) {
            let mut before = seat_ids.get(i);
            let mut after = seat_ids.get(i + 1);
            if before.is_some() && after.is_some() {
                match before {
                    Some(bef) => match after {
                        Some(af) => {
                            if af - bef > 1 {
                                my_seat = *s_id + 1;
                            }
                        }
                        _ => {}
                    }
                    _ => {}
                }
            }
        }
    }

    println!("part1: {}, part2: {}",seat_ids.iter().max().unwrap(), my_seat);

}
