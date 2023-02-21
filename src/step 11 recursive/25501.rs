use std::io;
use std::fmt::Write;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim_end().parse::<usize>().unwrap();
    let mut output = String::new();
    for _ in 0..input {
        let mut count = 0usize;
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim_end().as_bytes();
        let result = is_palindrome(&input, 0, input.len() - 1, &mut count);
        write!(output, "{} {}\n", result, count).unwrap();
    }
    print!("{}", output);
}

fn is_palindrome(input: &[u8], start: usize, end: usize, count: &mut usize) -> usize {
    *count += 1;
    if start >= end {
        return 1;
    }
    if input[start] != input[end] {
        return 0;
    }
    is_palindrome(input, start + 1, end - 1, count)
}
