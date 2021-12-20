use aoclib::fileops;
use itertools::Itertools;
use ndarray::prelude::*;
use std::collections::VecDeque;

fn main() {
    part1();
    part2();
}

fn part1()
    -> ()
{
    let input = fileops::get_file_lines("input.txt");
    let map = parse_lines(input);
    let lowest_points = find_lowest_points(&map);
    let risk_sum = calc_risk(&map, &lowest_points);
    println!("part1 {}", risk_sum);
}

fn part2()
    -> ()
{
    let input = fileops::get_file_lines("input.txt");
    let map = parse_lines(input);
    let lowest_points = find_lowest_points(&map);
    let basins = find_basins(&map, &lowest_points);
    let basin_score = calc_biggest_basin_prod(&basins);
    println!("part2 {}", basin_score);
}

fn parse_lines(lines: impl Iterator<Item = String>)
    -> Array2<usize>
{
    let rows: Vec<Vec<usize>> = lines.map(|x| {
            x.chars().map(|c| { c.to_digit(10).unwrap() as usize })
                .collect()
        }).collect();
    let h = rows.len();
    let w = rows[0].len();
    let rows = rows.concat();
    Array2::from_shape_vec((h,w), rows).expect("err")
}

fn find_lowest_points(map: &Array2<usize>)
    -> Vec<(usize, usize)>
{
    let mut lowest_map = Vec::new();
    for (index, _) in map.indexed_iter() {
        let lowest = is_lowest(map, index);
        if lowest {
            lowest_map.push(index);
        }
    }

    lowest_map
}

fn is_lowest(map: &Array2<usize>, pos: (usize, usize))
    -> bool
{
    let h = pos.0 as i32;
    let w = pos.1 as i32;

    for h in h-1..=h+1 {
        if h as usize == pos.0
            || h < 0
            || h as usize >= map.dim().0 {
            continue;
        }
        if map[pos] >= map[(h as usize,pos.1)] {
            return false;
        }
    }
    for w in w-1..=w+1 {
        if w as usize == pos.1
            || w < 0
            || w as usize >= map.dim().1 {
            continue;
        }
        if map[pos] >= map[(pos.0,w as usize)] {
            return false;
        }
    }
    true
}

fn find_adjecents(map: &Array2<usize>, pos: (usize, usize))
    -> Vec<(usize,usize)>
{
    let h = pos.0 as i32;
    let w = pos.1 as i32;
    let mut adjecents = Vec::new();

    for h in h-1..=h+1 {
        if h as usize == pos.0
            || h < 0
            || h as usize >= map.dim().0 {
            continue;
        }
        adjecents.push((h as usize, pos.1));
    }
    for w in w-1..=w+1 {
        if w as usize == pos.1
            || w < 0
            || w as usize >= map.dim().1 {
            continue;
        }
        adjecents.push((pos.0,w as usize));
    }
    adjecents
}

fn find_basins(map: &Array2<usize>, lowest_points: &Vec<(usize, usize)>)
    -> Vec<Vec<(usize, usize)>>
{
    let mut basins = Vec::new();
    for lowest in lowest_points {
        basins.push(expand_basin(map, VecDeque::from(vec![*lowest]), Vec::new()));
    }
    basins
}

fn expand_basin(map: &Array2<usize>
        , mut horizon: VecDeque<(usize, usize)>
        , mut total: Vec<(usize, usize)>
)
    -> Vec<(usize,usize)>
{
    while let Some(point) = horizon.pop_front() {
        total.push(point);
        for adjecent in find_adjecents(map, point) {
            if map[adjecent] != 9
                && !total.contains(&adjecent)
                && !horizon.contains(&adjecent)
            {
                horizon.push_back(adjecent);
            }
        }
    }
    total
}

fn calc_biggest_basin_prod(basins: &Vec<Vec<(usize,usize)>>)
    -> usize
{
    let mut basin_scores: Vec<usize> = basins.iter().map(|x| {
            x.len()
        }).sorted().collect();
    assert!(basin_scores.len() >= 3);
    basin_scores.pop().unwrap() * basin_scores.pop().unwrap() * basin_scores.pop().unwrap()
}

fn calc_risk(map: &Array2<usize>, lowest_points: &Vec<(usize,usize)>)
    -> usize
{
    let mut lowest_map:Array2<usize> = Array2::zeros(map.raw_dim());
    for idx in lowest_points {
        lowest_map[*idx] = 1;
    }
    (map * &lowest_map).sum() as usize + lowest_map.sum() as usize
}

#[cfg(test)]
mod tests
{
    use super::*;

    fn get_test_input()
        -> Array2<usize>
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

    #[test]
    /// test finding the lowest point
    fn test_0x0002()
    {
        let input = get_test_input();
        let lowest_points = find_lowest_points(&input);
        assert_eq!(lowest_points.len(), 4);
        assert_eq!(lowest_points, vec![(0,1), (0,9), (2,2), (4,6)]);
    }

    #[test]
    /// test finding the lowest point
    fn test_0x0003()
    {
        let input = get_test_input();
        let lowest_points = find_lowest_points(&input);
        let risk_sum = calc_risk(&input, &lowest_points);
        assert_eq!(risk_sum, 15);
    }

    #[test]
    /// test finding the lowest point
    fn test_0x0004()
    {
        let input = get_test_input();
        let lowest_points = find_lowest_points(&input);
        let basins = find_basins(&input, &lowest_points);
        assert_eq!(basins.len(), 4);
        assert_eq!(calc_biggest_basin_prod(&basins), 1134);
    }
}
