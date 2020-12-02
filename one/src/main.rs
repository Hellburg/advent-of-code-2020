use std::fs;
use std::io;
use std::env;

fn find_twty_twty(numbers: Vec<i32>, pos1: usize, pos2: usize) {
    let a = numbers.get(pos1).unwrap();
    let b = numbers.get(pos2).unwrap();
    println!("Comparing: {0} and {1}", a, b);
    if a + b == 2020 {
        println!("You found the answer: {0}, {1}, {2}", a, b, a*b);
        return;
    }
    else {
        if pos1 == (numbers.len() - 1) {
            println!("No answer found.");
            return;
        }
        else if pos2 == (numbers.len() - 1) {
            find_twty_twty(numbers, pos1 + 1, pos1 + 2)
        }
        else {
            find_twty_twty(numbers, pos1, pos2 + 1);
        }
    }
}

fn find_twty_twty_twty(numbers: Vec<i32>, pos1: usize, pos2: usize, pos3: usize) {
    let a = numbers.get(pos1).unwrap();
    let b = numbers.get(pos2).unwrap();
    println!("pos1: {}, pos2: {}, pos3: {}, length: {}",pos1, pos2, pos3, numbers.len());
    let c = numbers.get(pos3).unwrap();
    println!("Comparing {0}, {1}, {2}!", a, b, c);
    if a + b < 2020 && b+c < 2020{
        if (a + b + c == 2020) {
            println!("You found the answer: {0}, {1}, {2}, {3}", a, b, c, a*b);
            return;
        }
    } else {
        if (pos1 == numbers.len() - 2) {
            println!("No answer found.");
            return;
        }
        else if (pos2 == numbers.len() - 1) {
            find_twty_twty_twty(numbers, pos1 + 1, pos1 + 2, pos1 + 3);
        }
        else if (pos3 == numbers.len() - 1) {
            find_twty_twty_twty(numbers, pos1, pos2 + 1, pos2 + 2);
        } else {
            find_twty_twty_twty(numbers, pos1, pos2, pos3 + 1);
        }
    }

}

fn main() {

    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the inputs.");

    let mut split = input.split("\n");
    let mut numbers : Vec<i32> = Vec::new();
    for number_as_string  in split {
        if (!number_as_string.is_empty()) {
            let value = number_as_string.parse::<i32>().unwrap();
            numbers.push(value);
        }
    }

    let mut minima = 0;
    match numbers.iter().min() {
        Some(min) => minima = *min,
        None => println!("empty"),
    }

    let mut maxima = 0;
    match numbers.iter().max() {
        Some(max) => maxima = *max,
        None => println!("empty"),
    }

    let mut realistic_candidates = Vec::new();

    for value in numbers {
        if value < (2020-minima*2) {
            realistic_candidates.push(value);
        }
    }

    find_twty_twty_twty(realistic_candidates, 0, 1, 2);

    /*

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
    println!("They have a product of: {}", result.0 * result.1);*/
}
