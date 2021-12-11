use aoclib::{fileops,bitops};

const NUM_BITS_IN_INPUT: i32 = 11;

fn main() {
    part1();
    part2();
}

fn part1()
    -> ()
{
    let mut lines = fileops::get_file_lines("input.txt");
    let mut length = 1;
    let mut bitsums: Vec<i32> = bitops::bitstr_to_vec(&lines.next().unwrap());
    for line in lines {
        length += 1;
        let bits = bitops::bitstr_to_vec(&line);
        for i in 0..bits.len() {
            bitsums[i] += bits[i];
        }
    }
    let most_common_bits: Vec<u8> = bitsums.iter().map(|x| {
        if x >= &(length / 2) { 1 } else { 0 }
    }).collect();
    let least_common_bits: Vec<u8> = bitsums.iter().map(|x| {
        if x >= &(length / 2) { 0 } else { 1 }
    }).collect();
    let gamma_rate = bitops::bitvec_to_num(&most_common_bits);
    let epsilon_rate = bitops::bitvec_to_num(&least_common_bits);
    println!("part1: {}", gamma_rate as u64 * epsilon_rate as u64);
}

fn part2()
    -> ()
{
    let lines: Vec<i32> = fileops::get_file_lines("input.txt")
        .map(|x| {
            bitops::bitstr_to_num(&x)
        }).collect();
    let oxygen = do_filter(&lines, NUM_BITS_IN_INPUT, true);
    let scrubber = do_filter(&lines, NUM_BITS_IN_INPUT, false);
    println!("part2: {}", oxygen[0] * scrubber[0]);
}

fn do_filter(vec: &[i32], bit: i32, most_common: bool)
    -> Vec<i32>
{
    if vec.len() <= 1  || bit < 0 {
        return vec.to_vec();
    }
    let bitsum = vec.iter()
        .fold(0, |acc, x| {
            if x & (1 << bit) != 0 {
                acc + 1
            } else {
                acc
            }
        });
    let mcb = if bitsum as f32 >= vec.len() as f32 / 2.0 {
        1
    } else {
        0
    };
    let mut choice = if most_common {
        mcb
    } else {
        (mcb + 1) % 2
    };
    if bitsum == 0 {
        choice = 0;
    }
    if bitsum == vec.len() as i32 {
        choice = 1;
    }
    let filtered: Vec<i32> = vec.iter()
            .filter(|&&x| { ((x >> bit) & 1) == choice })
            .map(|x| { *x }) //TODO why?
            .collect();
    do_filter(
        &filtered,
        bit - 1,
        most_common)
}
