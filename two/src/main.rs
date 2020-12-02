use std::fs;

struct Policy {

    atleast: i32,
    at_most: i32,
    letter: char
}

impl Policy {
    fn validate_part_one(&self, password: String) -> bool {
        let mut contains_count = 0;
        for c in password.chars() {
            if self.letter.eq(&c) {
                contains_count += 1;
            }
        }
        contains_count <= self.at_most && contains_count >= self.atleast
    }

    fn validate_part_two(&self, password: String) -> bool {
        let char_vec: Vec<char> = password.chars().collect();
        let first_validation_char: &char = char_vec.get((self.atleast - 1) as usize)
            .expect("");
        let second_validation_char: &char = char_vec.get((self.at_most - 1)  as usize)
            .expect("");
        let contains_first = self.letter.eq(first_validation_char);
        let contains_second = self.letter.eq(second_validation_char);
        (contains_first && !contains_second) || (!contains_first && contains_second)
    }
}

fn main() {

    let input = fs::read_to_string("input.txt")
        .expect("invalid input");

    let splitted_inputs = input.split("\n");
    let mut no_valid_pass_one = 0;
    let mut no_valid_pass_two = 0;
    for policy in splitted_inputs {
        if (policy.is_empty()) {
            continue;
        }
        let policy_parts: Vec<&str> = policy
            .split(" ")
            .collect();

        let alteast_atmost: Vec<&str> = policy_parts.get(0)
            .expect("")
            .split("-")
            .collect();

        let atleast: i32 = alteast_atmost.get(0)
            .unwrap()
            .parse()
            .expect("");

        let at_most: i32 = alteast_atmost.get(1)
            .unwrap()
            .parse()
            .unwrap();

        let letter_as_str= policy_parts.get(1)
            .expect("")
            .get(0..1)
            .expect("");

        let letters: Vec<char> = letter_as_str.chars()
            .collect();

        let password: &str = policy_parts.get(2)
            .expect("");

        let pass_pol = Policy {
            atleast,
            at_most,
            letter: letters[0]
        };

        if pass_pol.validate_part_one(password.parse().unwrap()) {
            no_valid_pass_one += 1;
        }

        if pass_pol.validate_part_two(password.parse().unwrap()) {
            no_valid_pass_two += 1;
        }
    }
    println!("First part: {}", no_valid_pass_one);
    println!("Second part: {}", no_valid_pass_two);
}
