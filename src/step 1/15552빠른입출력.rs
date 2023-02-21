use std::io;
use std::fmt::Write;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let count = input.trim_end().parse::<usize>().unwrap();
    let mut output = String::new();
    for _ in 0..count {
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).unwrap();
        let numbers = input2.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
        let x1 = numbers[0];
        let y1 = numbers[1];
        write!(output, "{}\n", x1 + y1).unwrap();
    }
    print!("{}", output);
}