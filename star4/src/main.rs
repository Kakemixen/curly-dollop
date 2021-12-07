use aoclib::fileops;
use itertools::Itertools;

fn main() {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    let lines = fileops::get_file_lines("input2.txt");
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
    println!("pos:{}, depth:{}, prod:{}", pos, depth, pos*depth);
}
