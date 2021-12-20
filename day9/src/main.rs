use aoclib::fileops;
use itertools::Itertools;
use ndarray::prelude::*;

fn main() {
    part1();
    part2();
}

fn part1()
    -> ()
{
    println!("part1 {}", "TODO");
}

fn part2()
    -> ()
{
    println!("part2 {}", "TODO");
}

fn parse_lines(lines: impl Iterator<Item = String>)
    -> Array2<u8>
{
    let rows: Vec<Vec<u8>> = lines.map(|x| {
            x.chars().map(|c| { c.to_digit(10).unwrap() as u8 })
                .collect()
        }).collect();
    let h = rows.len();
    let w = rows[0].len();
    let rows = rows.concat();
    Array2::from_shape_vec((h,w), rows).expect("err")
}

#[cfg(test)]
mod tests
{
    use super::*;

    fn get_test_input()
        -> Array2<u8>
    {
        let lines = fileops::get_file_lines("test_input.txt");
        parse_lines(lines)
    }

    #[test]
    /// test parse input
    fn test_0x0001()
    {
        let input = get_test_input();
        assert_eq!(input.ndim(), 2);
        assert_eq!(input.shape(), [5,10]);
        assert_eq!(input,
                   arr2(&[
                        [2,1,9,9,9,4,3,2,1,0],
                        [3,9,8,7,8,9,4,9,2,1],
                        [9,8,5,6,7,8,9,8,9,2],
                        [8,7,6,7,8,9,6,7,8,9],
                        [9,8,9,9,9,6,5,6,7,8]
                   ]));
    }
}
