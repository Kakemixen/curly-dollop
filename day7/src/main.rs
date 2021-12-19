use aoclib::fileops;
use itertools::{self, Itertools};

/*
 * TODO just create this mapping as a hashmap or smth and use that to calculate distance
 * diff  = 0,1,2,3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,16
 * total = 0,1,3,6,10,15,21,28,36,45
 */

fn main() {
    part1();
    part2();
}

fn part1()
    -> ()
{
    let positions = parse_input();
    let (_, fuel) = choose_best_alignment_position(&positions, FuelModel::Constant);
    println!("part1 {}", fuel);
}

fn part2()
    -> ()
{
    let positions = parse_input();
    let (_, fuel) = choose_best_alignment_position(&positions, FuelModel::Linear);
    println!("part2 {}", fuel);
}

fn parse_input()
    -> Vec<usize>
{
    let line = fileops::get_file_lines("input.txt").next().unwrap();
    line.split(",").map(|x| { x.parse::<usize>().unwrap() } ).collect_vec()
}

enum FuelModel
{
    Constant,
    Linear,
}

fn choose_best_alignment_position(positions: &Vec<usize>, fuel_model: FuelModel)
    -> (usize, usize)
{
    let target = greedy_best_search(positions, &fuel_model);
    let total_diff = find_total_diff(positions, target, &fuel_model);
    (target, total_diff)
}

fn find_total_diff(positions: &[usize], target: usize, fuel_model: &FuelModel)
    -> usize
{
    static mut TABLE: Vec<usize> = Vec::new();

    positions.iter().fold(0, |acc, x| {
        match fuel_model
        {
        FuelModel::Constant => {
            acc + (*x as f32 - target as f32).abs() as usize
        },
        FuelModel::Linear => { unsafe {
            if TABLE.is_empty() {
                TABLE = gen_lookup_table(positions, &fuel_model);
            }
            let diff = (*x as f32 - target as f32).abs() as usize;
            acc + TABLE[diff]
        } },
        }
    })
}

fn find_median(vec: &[usize])
    -> usize
{
    let sorted: Vec<&usize> = itertools::sorted(vec).collect();
    *sorted[vec.len()/2]
}

fn greedy_best_search(positions: &[usize], fuel_model: &FuelModel)
    -> usize
{
    if let FuelModel::Constant = fuel_model {
        return find_median(positions);
    }

    let mut best = 0;
    let mut total_cost = usize::MAX;
    for p in 0..positions.len() {
        let p_cost = find_total_diff(positions, p, fuel_model);
        if p_cost > total_cost {
            break;
        }
        total_cost = p_cost;
        best = p;
    }
    best
}

fn gen_lookup_table(positions: &[usize], fuel_model: &FuelModel)
    -> Vec<usize>
{
    let max_pos = positions.iter().max().unwrap();
    let mut table = Vec::with_capacity(*max_pos);
    table.push(0);
    let mut cost = 0;
    for i in 0..*max_pos {
        table.push(match fuel_model {
            FuelModel::Constant =>  table[i] + 1,
            FuelModel::Linear => {
                cost += 1;
                table[i] + cost
            },
        });
    }
    table
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    /// test constant fuel
    fn test_0x0001()
    {
        let input = vec![16,1,2,0,4,2,7,1,2,14];
        let (pos, fuel_cost) = choose_best_alignment_position(&input, FuelModel::Constant);
        println!("pos:{}, fuel_cost:{}", pos, fuel_cost);
        assert_eq!(pos, 2);
        assert_eq!(fuel_cost, 37);

    }

    #[test]
    /// test fuel cost lookup
    fn test_0x0002()
    {
        let input = vec![16,1,2,0,4,2,7,1,2,14];
        let constant_lookup = gen_lookup_table(&input, &FuelModel::Constant);
        assert_eq!(constant_lookup.len(), 17); //0 ..= 16
        assert_eq!(constant_lookup, vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);

        let linear_lookup = gen_lookup_table(&input, &FuelModel::Linear);
        assert_eq!(linear_lookup.len(), 17); //0 ..= 16
        assert_eq!(linear_lookup[0],  0);
        assert_eq!(linear_lookup[1],  1);
        assert_eq!(linear_lookup[2],  1+2);
        assert_eq!(linear_lookup[3],  1+2+3);
        assert_eq!(linear_lookup[4],  1+2+3+4);
        assert_eq!(linear_lookup[5],  1+2+3+4+5);
        assert_eq!(linear_lookup[6],  1+2+3+4+5+6);
        assert_eq!(linear_lookup[7],  1+2+3+4+5+6+7);
        assert_eq!(linear_lookup[8],  1+2+3+4+5+6+7+8);
        assert_eq!(linear_lookup[9],  1+2+3+4+5+6+7+8+9);
        assert_eq!(linear_lookup[10], 1+2+3+4+5+6+7+8+9+10);
        assert_eq!(linear_lookup[11], 1+2+3+4+5+6+7+8+9+10+11);
        assert_eq!(linear_lookup[12], 1+2+3+4+5+6+7+8+9+10+11+12);
        assert_eq!(linear_lookup[13], 1+2+3+4+5+6+7+8+9+10+11+12+13);
        assert_eq!(linear_lookup[14], 1+2+3+4+5+6+7+8+9+10+11+12+13+14);
        assert_eq!(linear_lookup[15], 1+2+3+4+5+6+7+8+9+10+11+12+13+14+15);
        assert_eq!(linear_lookup[16], 1+2+3+4+5+6+7+8+9+10+11+12+13+14+15+16);

    }

    #[test]
    /// test linear fuel
    fn test_0x0003()
    {
        let input = vec![16,1,2,0,4,2,7,1,2,14];
        let (pos, fuel_cost) = choose_best_alignment_position(&input, FuelModel::Linear);
        println!("pos:{}, fuel_cost:{}", pos, fuel_cost);
        assert_eq!(pos, 5);
        assert_eq!(fuel_cost, 168);

    }
}
