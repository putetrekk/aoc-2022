use std::{fs::File, io::Read, vec};

fn main() {
    let input = read_string_from_file("day01/input.txt");

    let numbers: Vec<Vec<i32>> = input
        .split("\r\n\r\n")
        .map(|s| s
            .lines()
            .map(|s| s.parse().unwrap())
            .collect())
        .collect();

    let mut sums = vec![0; numbers.len()];
    for i in 0..numbers.len() {
        let sum = numbers[i].iter().sum();
        sums[i] = sum;
    }

    println!("calories of top elf: {}", sums.iter().max().unwrap());

    sums.sort_by(|a, b| b.cmp(a));

    let top_three = &sums[..3];
    let top_three_sum: i32 = top_three.iter().sum();

    println!("calories of top three elves: {}", top_three_sum);    
}

fn read_string_from_file(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Can't open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Can't read file contents...");

    return contents;
}
