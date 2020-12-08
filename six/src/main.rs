use std::fs;
use std::collections::HashMap;

fn count_group(group: &str) -> i32{
    let mut occurences: HashMap<char, i32> = HashMap::new();
    for g in group.split("\n") {
        for c in g.chars() {
            match occurences.get(&c) {
                None => {
                    occurences.insert(c, 1);
                },
                Some(count) => {
                    occurences.insert(c, count + 1);
                },
            }
        }
    }
    occurences.len() as i32
}

fn count_group_p2(group: &str) -> i32 {
    let mut occurences: HashMap<char, i32> = HashMap::new();
    let mut no_all_answers = 0;
    let mut no_individuals = 0;
    for i in group.split("\n") {
        if (!i.is_empty() && i != "\n") {
            no_individuals += 1;
            for c in i.chars() {
                match occurences.get(&c) {
                    None => {
                        occurences.insert(c, 1);
                    },
                    Some(count) => {
                        occurences.insert(c, count + 1);
                    },
                }
            }
        }
    }

    for ans in occurences {
        if ans.1 == no_individuals {
            no_all_answers += 1;
        }
    }
    no_all_answers
}

fn main() {

    let input = fs::read_to_string("input.txt")
        .expect("Error reading input.");

    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut total_part1 = 0;
    let mut total_part2 = 0;
    for (i, g) in groups.iter().enumerate() {
        total_part1 += count_group(g);
        total_part2 += count_group_p2(g);
    }

    println!("part1 {}", total_part1);
    println!("part2 {}", total_part2);
}
