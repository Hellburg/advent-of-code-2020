use std::fs;
use std::io;
use std::env;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the inputs.");

    let mut split = input.split("\n");
    let mut numbers : Vec<i32> = Vec::new();
    for number_as_string  in split {
        if (!number_as_string.is_empty()) {
            numbers.push(number_as_string.parse::<i32>().unwrap());
        }
    }

    let mut i = 1;
    let mut currentComparer = numbers.get(0);
    let mut done = false;
    let mut result: (i32, i32) = (0, 0);
    while i <= numbers.len() {
        let mut j = 0;
        while j < numbers.len() - i  {
            let a = numbers.get(j + i).unwrap();
            let b = currentComparer.unwrap();
            let sum = a + b;
            if sum == 2020 {
                result = (*b, *a);
                done = true;
                break;
            }
            j += 1;
        }
        if done {
            break;
        }
        currentComparer = numbers.get(i);
        i += 1;
    }

    if (result.0 == 0 && result.0 == 0) {
        println!("No numbers found adding up to 2020.");
    }
    println!("The numbers are: {0}, {1}", result.0, result.1);
    println!("They have a product of: {}", result.0 * result.1);
}
