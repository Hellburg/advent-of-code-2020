use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Error loading input");

    let rows: Vec<&str> = input.split("\n")
        .collect();

    let mut no_trees= 0;
    for (i, &row) in rows.iter().enumerate() {
        if !row.is_empty() {
            if (i*3 > 32) {
                println!("was: {}, became: {}, on row: {}", i*3, (i*3) % 32, i);
            }
            println!("{:?}, {}", row, i);
            match row.as_bytes().get((i*3) % 32) {
                Some(res) => if res == &b'#' {
                    no_trees += 1;
                    println!{"hit! row: {}, col: {}", i, (i*3) % 32}
                },
                None => {}
            }
        }
    }
    println!("{}", no_trees);
}
