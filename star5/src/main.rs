use std::process::exit;

use aoclib::fileops;

fn main() {
    let mut lines = fileops::get_file_lines("input3.txt");
    let mut length = 1;
    let mut bitsums: Vec<i32> = line_to_vec(&lines.next().unwrap());
    for line in lines {
        length += 1;
        let bits = line_to_vec(&line);
        for i in 0..bits.len() {
            bitsums[i] += bits[i];
        }
    }
    println!("{:?}", bitsums);
    println!("{}", length);
    let most_common_bits: Vec<i32> = bitsums.iter().map(|x| {
        if x >= &(length / 2) { 1 } else { 0 }
    })
    .collect();
    println!("{:?}", most_common_bits);

}

fn line_to_vec(line: &str)
    -> Vec<i32>
{
    line.chars()
        .map(|x| {
            match x {
                '0' => 0,
                '1' => 1,
                _ => unreachable!(),
            }
        })
        .collect()
}
