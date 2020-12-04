use std::fs;
use std::collections::HashMap;

struct Passport {
    fields: HashMap<String, String>,
    required: Vec<String>
}

enum Height {
    CM(i32),
    IN(i32)
}

impl Passport {

    fn print(&self) {
        println!("{:?}", self.fields);
    }

    fn create(passport_string: &str) -> Passport {
        let mut fields: HashMap<String, String> = HashMap::new();
        let pass_split_on_newline: Vec<&str> = passport_string.split("\n").collect();
        for split_on_newline in pass_split_on_newline {
            let split_again: Vec<&str> = split_on_newline.split(" ").collect();
            for split_on_mellanslag in split_again {
                let part: Vec<&str> = split_on_mellanslag.split(":")
                    .filter(|x| !x.is_empty()).collect();
                println!("{:?}", part);
                if !part.is_empty() {
                    fields.insert(String::from(*part.get(0).unwrap()), String::from(*part.get(1).unwrap()));
                }
            }
        }
        Passport {
            fields,
            required: vec![String::from("byr"),
                           String::from("iyr"),
                           String::from("eyr"),
                           String::from("hgt"),
                           String::from("hcl"),
                           String::from("ecl"),
                           String::from("pid")]
        }
    }

    fn is_valid(&self) -> bool {
        let mut i = 0;
        let mut valid = true;
        for req in &self.required {
            let opt = self.fields.get(req);
            valid = valid && match opt {
                Some(r) => match req.as_str() {
                    "byr" => Passport::atleast_at_most(r.parse::<i32>().unwrap(), 1920, 2002),
                    "iyr" => Passport::atleast_at_most(r.parse::<i32>().unwrap(), 2010, 2020),
                    "eyr" => Passport::atleast_at_most(r.parse::<i32>().unwrap(), 2020, 2030),
                    "hgt" => Passport::valid_height(r),
                    "hcl" => Passport::valid_hair_color(r),
                    "ecl" => Passport::valid_eye_color(r),
                    "pid" => match r.parse::<i32>() {
                        Ok(res) => {
                            let ress = (res / 1000000000 < 10);
                            println!("pid valid? {}, {}", ress, res);
                            ress
                        },
                        Err(e) => false
                    },
                    _ => { false }
                },
                None => { false }
            };
            i += 1;
        }
        valid
    }

    fn atleast_at_most(number: i32, atleast: i32, most: i32) -> bool {
        if number < atleast {
            return false
        }
        if number > most {
            return false
        }
        println!("{} is atleast {} and at most {}", number, atleast, most);
        true
    }

    fn valid_height(height_string: &String) -> bool {
        let mut valid = true;

        let height_as_chars = height_string
            .chars();

        let last_two: Vec<char> = height_as_chars
            .rev()
            .take(2)
            .collect();

        let height_as_chars = height_string
            .chars();

        let count = height_as_chars.count() - 2;

        let height_as_chars = height_string
            .chars();


        let height_digit_chars: Vec<char> = height_as_chars
            .take(count)
            .collect();
        println!("{:?}", height_digit_chars);
        let mut height_sum = 0;
        let mut factor = 1;
        for q in &height_digit_chars {
            factor *= 10;
        }
        factor /= 10;

        height_digit_chars.iter()
            .map(|h| h.to_digit(10))
            .for_each(|d| match d {
                Some(dig) => {
                    height_sum += dig * factor;
                    factor /= 10;
                },
                None => {
                    valid = false;
                }
            });

        println!("{:?}, height: {}", last_two, height_sum);

        if last_two.contains(&'c') && last_two.contains(&'m') {
            valid = (height_sum >= 150) && (height_sum <= 193);
            println!("cm, {}, {}", height_sum, valid);
        }
        else if last_two.contains(&'i') && last_two.contains(&'n') {
            valid = (height_sum >= 59) && (height_sum <= 76);
            println!("in, {}, {}", height_sum, height_sum >= 150 as u32);
        }
        if valid {
            println!("correct height! {}", height_string)
        } else {
            println!("invalid height! {}", height_string)
        }
        valid
    }

    fn valid_hair_color(hair_color: &String) -> bool{
        let allowed_chars: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
        let chars = hair_color.chars();
        let first_char: Vec<char> = chars.take(1).collect();
        let mut valid = first_char.get(0).unwrap() == &'#';
        let chars = hair_color.chars();
        let coll_chars: Vec<char> = chars.collect();
        if valid {
            let chars = hair_color.chars();
            for c in chars.rev().take(coll_chars.len() - 1) {
                if !allowed_chars.contains(&c) {
                    valid = false;
                    break;
                }
            }
        }


        valid = valid && coll_chars.len() == 7;

        if valid {
            println!("valid hair color! {}", hair_color);
        }

        valid
    }

    fn valid_eye_color(eye_color: &String) -> bool {
        let allowed_strings: [String; 6] =  [String::from("amb"),
            String::from("blu"),
            String::from("brn"),
            String::from("gry"),
            String::from("grn"),
            String::from("hzl")];

        let valid = allowed_strings.contains(eye_color);

        println!("valid? {}, {}", valid, eye_color);
        valid
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let passports_string: Vec<&str> = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .collect();

    let mut passports: Vec<Passport> = Vec::new();

    for pass in passports_string {
        passports.push(Passport::create(pass))
    }

    let mut no_valid_passports = 0;
    println!("no of pps {}", passports.len());
    for passport in passports {
        if passport.is_valid() {
            println!("is valid!");
            passport.print();
            no_valid_passports += 1;
        }
    }

    /* for pp in passports {
         let mut valid = true;
         for req in required.iter() {
             println!("checking for required: {}", req);
             match pp.get(req) {
                 Some(val) => match &req[..] {
                     "byr" => valid = atleast_and_most(val.parse::<i32>().unwrap(), 1920, 2002),
                     "iyr" => valid = atleast_and_most(val.parse::<i32>().unwrap(), 2010, 2020),
                     "eyr" => valid = atleast_and_most(val.parse::<i32>().unwrap(), 2020, 2030),
                     "hgt" => {
                         let last_two: Vec<char> = String::from(*val).chars().rev().take(2).collect();
                         let mut i = 0;
                         let mut cool = false;
                         for c in &last_two {
                             let nbr = *last_two.get(i).unwrap();
                             let mut cm_true = 0;
                             let mut in_true = 0;
                             if nbr == 'c' {
                                 cm_true += 1;
                             }
                             else if (nbr == 'm') {
                                 cm_true += 1;
                             }
                             else if (nbr == 'i') {
                                 in_true += 1;
                             }
                             else if (nbr == 'n') {
                                 in_true += 1;
                             }
                             if (cm_true == 2) {
                                 cool = atleast_and_most(val.parse::<i32>().unwrap(), 93, 150)
                             }
                             else if (in_true == 2) {
                                 cool = atleast_and_most(val.parse::<i32>().unwrap(), 59, 76)
                             }
                         }
                         valid = cool;
                     },
                     "hcl" => {
                         let allowed_chars: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
                         let chars = val.chars();
                         let first_char: Vec<char> = chars.take(1).collect();
                         valid = first_char.get(0).unwrap() == &'#';
                         let coll_chars = chars.collect();
                         if valid {
                             for c in chars.rev().take(coll_chars.len() - 1) {
                                 if !allowed_chars.contains(&c) {
                                     valid = false;
                                     break;
                                 }
                             }
                         }
                         valid = valid && coll_chars.len() == 7;
                     },
                     "ecl" => valid = true,
                     "pid" => valid = val.parse::<i32>().unwrap() == 9,
                     _ => {}
                 }
                 None => valid = false,
             }
             if !valid {
                 break;
             }
         }
         if valid {
             no_valid_passports += 1;
             println!("valid!");
         }
         println!("------------------------------------------");
     }

 */
    println!("{}", no_valid_passports);

}
