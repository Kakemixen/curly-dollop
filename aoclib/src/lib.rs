pub mod threadpool;

pub mod fileops {
    use std::fs::File;
    use std::path::Path;
    use std::io::{self, BufRead};

    pub fn get_file_lines<P>(path: P)
        -> impl Iterator<Item = String>
    where P: AsRef<Path>
    {
        let file = File::open(path).expect("could open file!");
        io::BufReader::new(file).lines()
            .map(|x| {
                x.expect("couldn't read line!")
            })
    }
}

pub mod bitops {

    pub fn bitvec_to_num(bits: &[u8])
        -> i32
    {
        bits.iter().fold(0, |acc, x| {
            let res = (acc << 1) + *x as i32;
            res
        })
    }

    pub fn bitstr_to_vec(line: &str)
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

    pub fn bitstr_to_num(bitstr: &str)
        -> i32
    {
        let vec: Vec<u8> = bitstr_to_vec(bitstr).iter()
            .map(|x| { *x as u8 }).collect();
        bitvec_to_num(&vec)
    }
}
