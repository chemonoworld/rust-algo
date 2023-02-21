use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim_end().parse::<usize>().unwrap();
    let result = fibonazzi(input);
    print!("{}", result);
}

fn fibonazzi(order: usize) -> usize {
    if order == 0 {
        0usize
    } else if order == 1 {
        1usize
    } else {
        fibonazzi(order - 1) + fibonazzi(order - 2)
    }
}