use std::fs;

fn main() {
    part1();
    part2();
}

fn part1 ()
    -> ()
{
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("can't read file");
    let arr: Vec<&str> = contents.split("\n").collect();
    let mut acc = 0;
    for i in 1..arr.len()-1 {
        if arr[i].parse::<i32>().unwrap() > arr[i-1].parse::<i32>().unwrap() {
            acc += 1;
        }
    }
    println!("part1: {}", acc);
}

fn part2()
    -> ()
{
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("can't read file");
    let arr = contents.split("\n")
        .filter(|x| {!x.is_empty()});
    let arr: Vec<i32> = arr.map(|x| { 
        x.parse::<i32>().expect("can't convert string to int")
    }).collect();
    let mut acc = 0;
    for i in 1..arr.len()-2 {
        if trisum(&arr, i) > trisum(&arr, i-1) {
            acc += 1;
        }
    }
    println!("part2: {}", acc);
}

fn trisum<'a, T>(arr: &'a [T], start: usize) -> T
    where T: num::Integer + std::iter::Sum<&'a T>
{
    arr[start..start+3].iter().sum()
}
