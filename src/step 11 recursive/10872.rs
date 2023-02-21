use std::{io};
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim_end().parse::<usize>().unwrap();
    let result = factorial(input);
    print!("{}", result);
}

fn factorial(input: usize) -> usize {
    if input == 0 {
        return 1;
    } else {
        return input * factorial(input - 1);
    }
}