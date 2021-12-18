use aoclib::fileops;
use itertools::{self, Itertools};

fn main() {
    part1();
    part2();
}

fn part1()
    -> ()
{
    let positions = parse_input();
    let (_, fuel) = choose_best_alignment_position(&positions);
    println!("part1 {}", fuel);

}

fn part2()
    -> ()
{
    println!("part2 {}", "TODO");
}

fn parse_input()
    -> Vec<usize>
{
    let line = fileops::get_file_lines("input.txt").next().unwrap();
    line.split(",").map(|x| { x.parse::<usize>().unwrap() } ).collect_vec()
}

fn choose_best_alignment_position(positions: &Vec<usize>)
    -> (usize, usize)
{
    let median = find_median(positions);
    let total_diff = find_total_diff(positions, median);
    (median, total_diff)
}

fn find_total_diff(positions: &[usize], target: usize)
    -> usize
{
    positions.iter().fold(0, |acc, x| {
        acc + (*x as f32 - target as f32).abs() as usize
    })
}

fn find_median(vec: &[usize])
    -> usize
{
    let sorted: Vec<&usize> = itertools::sorted(vec).collect();
    *sorted[vec.len()/2]
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_0x0001()
    {
        let input = vec![16,1,2,0,4,2,7,1,2,14];
        let (pos, fuel_cost) = choose_best_alignment_position(&input);
        println!("pos:{}, fuel_cost:{}", pos, fuel_cost);
        assert_eq!(pos, 2);
        assert_eq!(fuel_cost, 37);

    }
}
