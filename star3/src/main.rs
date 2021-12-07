use aoclib::fileops;
use itertools::Itertools;

fn main() {
    let mut x = 0;
    let mut y = 0;
    let lines = fileops::get_file_lines("input2.txt");
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
    println!("x:{}, y:{}, prod:{}", x, y, x*y);
}
