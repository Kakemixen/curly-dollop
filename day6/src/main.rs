use aoclib::fileops;
use std::thread;
use std::sync::mpsc;

fn main() {
    part1();
    part2();
}

fn part1()
    -> ()
{
    let initial = parse_input();
    let population = grow_population_parallell(initial, 80);
    println!("part1 {}", population.len());
}

fn part2()
    -> ()
{
    let initial = parse_input();
    let population = grow_population_parallell(initial, 256);
    println!("part1 {}", population.len());
}

fn parse_input()
    -> Vec<u8>
{
    fileops::get_file_lines("input.txt")
        .next().unwrap()
        .split(",")
        .map(|x| {
            x.parse::<u8>().expect("not a number")
        })
        .collect()
}

fn grow_population_parallell(population: Vec<u8>, steps: usize)
    -> Vec<u8>
{
    let limit = 500000;
    let split = 4;

    grow_population_parallell_impl(population, steps, split, 0, limit)
}

fn grow_population_parallell_impl(
        mut population: Vec<u8>,
        steps:          usize,
        split:          usize,
        current:        usize,
        limit:          usize,
)
    -> Vec<u8>
{
    let limit = 10000000;
    let split = 4;

    for i in current .. steps {
        population = grow_population(population, 1);

        if population.len() > limit {
            return grow_population_parallell_split(
                population,
                steps,
                split,
                i,
                limit);
        }
    }
    population
}

fn grow_population_parallell_split(
        mut population: Vec<u8>,
        steps:          usize,
        split:          usize,
        current:        usize,
        limit:          usize,
)
    -> Vec<u8>
{
    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    let mut handles = Vec::new();

    let size = population.len() / split;
    println!("splitting at {}", current);

    for i in 0 .. split {
        let tx_clone = tx.clone();
        let fishes = population[i*size .. (i+1)*size].to_vec();
        handles.push(thread::spawn(move || {
            let population = grow_population_parallell_impl(
                fishes,
                steps,
                split,
                current,
                limit);
            match tx_clone.send(population) {
                Ok(_) => println!("worker done"),
                Err(e) => println!("error: {}", e),
            }
        }));
    }
    population = Vec::new();

    for handle in handles {
        handle.join().unwrap();
    }

    while let Ok(fishes) = rx.try_recv() {
        println!("{:?}",fishes);
        population.extend(fishes);
    }

    population
}

fn grow_population(mut population: Vec<u8>, steps: usize)
    -> Vec<u8>
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

    #[test]
    fn test_0x002()
    {
        let initial = vec![3,4,3,1,2];
        let final_population = grow_population_parallell(initial, 18);
        println!("{:?}", final_population);
        assert_eq!(final_population.len(), 26);
        let final_population = grow_population(final_population, 80-18);
        assert_eq!(final_population.len(), 5934);
    }

    #[test]
    fn test_slice()
    {
        let vec = vec![1,2,3,4,5,6,7,8,9,10,11,12,13];
        let split = 4;
        let size = vec.len() / split;
        for i in 0 .. split-1{
            println!("{:?}", &vec[i*size .. (i+1)*size]);
        }
        println!("{:?}", &vec[(split-1)*size .. vec.len()]);
    }

    //#[test]
    //fn test_0x003()
    //{
    //    let initial = vec![3,4,3,1,2];
    //    let final_population = grow_population_parallell(initial, 256);
    //    println!("{:?}", final_population);
    //    assert_eq!(final_population.len(), 26984457539);
    //}
}
