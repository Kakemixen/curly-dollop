use aoclib::fileops;
use itertools::Itertools;
use std::cmp;

fn main() {
    part1();
    part2();
}

fn part1()
    -> ()
{
    let initial = parse_input();
    let population = grow_population(initial, 80);
}

fn part2()
    -> ()
{ }

fn parse_input()
    -> Vec<usize>
{
    fileops::get_file_lines("input.txt")
        .next().unwrap()
        .split(",")
        .map(|x| {
            x.parse::<usize>().expect("not a number")
        })
        .collect()
}

fn grow_population(mut population: Vec<usize>, steps: usize)
    -> Vec<usize>
{
    for _ in 0 .. steps {
        let old_len = population.len();
        for i in 0 .. old_len {
            if population[i] == 0 {
                population.push(8);
                population[i] = 6;
            } else {
                population[i] -= 1;
            }
        }
    }
    population
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_0x001()
    {
        let initial = vec![3,4,3,1,2];
        let final_population = grow_population(initial, 18);
        println!("{:?}", final_population);
        assert_eq!(final_population.len(), 26);
        let final_population = grow_population(final_population, 80-18);
        assert_eq!(final_population.len(), 5934);
    }
}
