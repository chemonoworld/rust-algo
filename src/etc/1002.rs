use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let input1: Vec<i32> = input1
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse input"))
        .collect();
    let total_num = input1[0];

    let mut numbers: Vec<i32> = Vec::new();
    numbers.reserve(6 * total_num as usize);
    // Read input from standard input
    for _ in 0..total_num {
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read input");
        // Parse input as two integers
    }
    let numbers2: Vec<i32> = input2
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse input"))
        .collect();
    numbers.extend(numbers2);

    for i in 0..total_num {
        let offset = 6 * i as usize;
        let x1 = numbers[offset];
        let y1 = numbers[offset + 1];
        let r1 = numbers[offset + 2] as f64;
        let x2 = numbers[offset + 3];
        let y2 = numbers[offset + 4];
        let r2 = numbers[offset + 5] as f64;

        let distance = ((x1 - x2).pow(2) as f64 + (y1 - y2).pow(2) as f64).sqrt() as f64;

        if x1 == x2 && y1 == y2 && r1 == r2 {
            println!("-1");
        } else if distance > (r1 + r2) {
            println!("0");
        } else if distance == (r1 + r2) {
            println!("1");
        } else if distance < (r1 + r2) {
            if distance == (r1- r2).abs() {
                println!("1");
            } else if distance > (r1-r2).abs() {
                println!("2");
            } else if distance < (r1-r2).abs() {
                println!("0");
            }
        }
    }
}
