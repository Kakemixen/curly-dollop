use aoclib::fileops;
use itertools::Itertools;

fn main() {
    part1();
    part2();
}

fn part1()
    -> ()
{
    let mut x = 0;
    let mut y = 0;
    let lines = fileops::get_file_lines("input.txt");
    for line in lines {
        let (direction, length) = line.split(" ").next_tuple()
            .expect("less than two elements");
        match direction {
            "forward" => x += length.parse::<i32>().unwrap(),
            "down" => y += length.parse::<i32>().unwrap(),
            "up" => y -= length.parse::<i32>().unwrap(),
            &_ => unreachable!("undefined direction!")
        }
    }
    println!("part1: {}", x*y);
}

fn part2()
    -> ()
{
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    let lines = fileops::get_file_lines("input.txt");
    for line in lines {
        let (direction, val) = line.split(" ").next_tuple()
            .expect("less than two elements");
        match direction {
            "down" => aim += val.parse::<i32>().unwrap(),
            "up" => aim -= val.parse::<i32>().unwrap(),
            "forward" => {
                let v = val.parse::<i32>().unwrap();
                pos += v;
                depth += aim * v;
            },
            &_ => unreachable!("undefined direction!")
        }
    }
    println!("part2: {}", pos*depth);
}
