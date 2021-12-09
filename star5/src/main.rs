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
    let most_common_bits: Vec<u8> = bitsums.iter().map(|x| {
        if x >= &(length / 2) { 1 } else { 0 }
    }).collect();
    let least_common_bits: Vec<u8> = bitsums.iter().map(|x| {
        if x >= &(length / 2) { 0 } else { 1 }
    }).collect();
    println!("{:?}", most_common_bits);
    let gamma_rate = bitvec_to_num(&most_common_bits);
    let epsilon_rate = bitvec_to_num(&least_common_bits);
    println!("gamma {:b}", gamma_rate);
    println!("epsilon {:b}", epsilon_rate);
    println!("{}", gamma_rate as u64 * epsilon_rate as u64);
}

fn bitvec_to_num(bits: &[u8])
    -> i32
{
    bits.iter().fold(0, |acc, x| {
        let res = (acc << 1) + *x as i32;
        res
    })
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
