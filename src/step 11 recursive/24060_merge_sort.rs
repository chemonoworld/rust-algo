use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers = input.trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let size = numbers[0];
    let mut temp = vec![0; size];
    temp.reserve(size);

    let save_num = numbers[1];

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let mut list = input2.trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut count = 0;
    merge_sort(&mut list, 0, size - 1, &mut temp, &mut count, &save_num);
    if count < save_num {
        println!("-1");
    }
}

fn merge_sort(
    list: &mut Vec<usize>,
    start: usize,
    end: usize,
    temp: &mut Vec<usize>,
    count: &mut usize,
    save_num: &usize,
) {
    if start < end {
        let mid = (start + end) / 2;
        merge_sort(list, start, mid, temp, count, &save_num);
        merge_sort(list, mid + 1, end, temp, count, &save_num);
        merge(list, start, mid, end, temp, count, &save_num);
    }
}

fn merge(
    list: &mut Vec<usize>,
    start: usize,
    mid: usize,
    end: usize,
    temp: &mut Vec<usize>,
    count: &mut usize,
    save_num: &usize,
) {
    let mut i = start;
    let mut j = mid + 1;
    let mut t = start;

    while i <= mid && j <= end {
        if list[i] <= list[j] {
            temp[t] = list[i];
            i += 1;
            *count += 1;
            if count == save_num {
                print!("{}", temp[t]);
            }
        } else {
            temp[t] = list[j];
            j += 1;
            *count += 1;
            if count == save_num {
                print!("{}", temp[t]);
            }
        }
        t += 1; 
    }

    if i > mid {
        for n in j..=end {
            temp[t] = list[n];
            *count += 1;
            if count == save_num {
                print!("{}", temp[t]);
            }
            t += 1;
        }
    } else {
        for n in i..=mid {
            temp[t] = list[n];
            *count += 1;
            if count == save_num {
                print!("{}", temp[t]);
            }
            t += 1;
        }
    }

    for n in start..=end {
        list[n] = temp[n];
    }

}