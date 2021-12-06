use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("can't read file");
    let arr: Vec<&str> = contents.split("\n").collect();
    let mut acc = 0;
    for i in 1..arr.len()-1 {
        if arr[i].parse::<i32>().unwrap() > arr[i-1].parse::<i32>().unwrap() {
            acc += 1;
        }
    }
    println!("{}", acc);
}
