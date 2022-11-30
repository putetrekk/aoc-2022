use std::{fs::File, io::Read};

fn main() {
    let input = read_string_from_file("day01/input.txt");

    let numbers: Vec<i32> = input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    let sum: i32 = numbers.iter().sum();

    println!("sum: {}", sum);
}

fn read_string_from_file(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Can't open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Can't read file contents...");

    return contents;
}
