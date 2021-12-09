use aoclib::{fileops,bitops};

const NUM_BITS_IN_INPUT: i32 = 11;

fn main() {
    let lines: Vec<i32> = fileops::get_file_lines("input3.txt")
        .map(|x| {
            bitops::bitstr_to_num(&x)
        }).collect();
    let oxygen = do_filter(&lines, NUM_BITS_IN_INPUT, true);
    println!("{:?}", oxygen);
    println!("{:b}", oxygen[0]);
    let scrubber = do_filter(&lines, NUM_BITS_IN_INPUT, false);
    println!();
    println!("{:?}", scrubber);
    println!("{:b}", scrubber[0]);
    println!("{}", oxygen[0] * scrubber[0]);
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
    println!("{:12b}", choice << bit);
    let filtered: Vec<i32> = vec.iter()
            .filter(|&&x| { ((x >> bit) & 1) == choice })
            .map(|x| { *x }) //TODO why?
            .collect();
    println!("{:12b}", filtered[0]);
    do_filter(
        &filtered,
        bit - 1,
        most_common)
}
