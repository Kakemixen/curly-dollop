use aoclib::fileops;

const HORIZON: usize = 100; //tuning parameter

fn main() {
    let lookup = create_lookup();
    part1(&lookup);
    part2(&lookup);
}

fn part1(lookup:     &Vec<Vec<u8>>)
    -> ()
{
    let total_steps = 80;
    let mut population = parse_input();
    let first_steps = total_steps % HORIZON;
    population = grow_population(population, first_steps);
    let population_size = forecast_population(population, total_steps/HORIZON, lookup);
    println!("part1 {}", population_size);
}

fn part2(lookup: &Vec<Vec<u8>>)
    -> ()
{
    let total_steps = 256;
    let mut population = parse_input();
    let first_steps = total_steps % HORIZON;
    population = grow_population(population, first_steps);
    let population_size = forecast_population(population, total_steps/HORIZON, lookup);
    println!("part2 {}", population_size);
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

fn create_lookup()
    -> Vec<Vec<u8>>
{
    let mut vec = Vec::new();
    for i in 0 ..= 8 {
        vec.push(
            grow_population(vec![i], HORIZON)
            );
    }
    vec
}

fn grow_population_lookup(
    mut population: Vec<u8>,
    iterations: usize,
    lookup:     &Vec<Vec<u8>>,
)
    -> Vec<u8>
{
    let mut tmp = Vec::new();
    for _ in 0..iterations {
        for fish in population {
            tmp.extend(&lookup[fish as usize].to_vec());
        }
        population = tmp;
        tmp = Vec::new();
    }
    population
}

fn forecast_population(
    population: Vec<u8>,
    iterations: usize,
    lookup:     &Vec<Vec<u8>>,
)
    -> usize
{
    return forecast_population_recursion(population, iterations, 0, lookup);
}

fn forecast_population_recursion(
    population: Vec<u8>,
    iterations: usize,
    current:    usize,
    lookup:     &Vec<Vec<u8>>,
)
    -> usize
{
    if current == iterations {
        return population.len();
    }

    let mut population_size = 0;

    //for fish in population {
    for i in 0..population.len() {
        if current == 0 {
            println!("{:05}/{:5} : {:12}", i, population.len(), population_size);
        }
        let fish = population[i];
        population_size += forecast_population_recursion(
                grow_population_lookup(vec![fish], 1, lookup),
                iterations,
                current + 1,
                lookup,
            );
    }

    population_size
}

//fn forecast_population_recursion_thread_split(
//        mut population: Vec<u8>,
//        steps:          usize,
//        split:          usize,
//        current:        usize,
//        limit:          usize,
//)
//    -> Vec<u8>
//{
//    let (tx, rx) = mpsc::channel::<Vec<u8>>();
//    let mut handles = Vec::new();
//
//    let size = population.len() / split;
//    println!("splitting at {}", current);
//
//    for i in 0 .. split {
//        let tx_clone = tx.clone();
//        let fishes = population[i*size .. (i+1)*size].to_vec();
//        handles.push(thread::spawn(move || {
//            let population = grow_population_parallell_impl(
//                fishes,
//                steps,
//                split,
//                current,
//                limit);
//            match tx_clone.send(population) {
//                Ok(_) => println!("worker done"),
//                Err(e) => println!("error: {}", e),
//            }
//        }));
//    }
//    population = Vec::new();
//
//    for handle in handles {
//        handle.join().unwrap();
//    }
//
//    while let Ok(fishes) = rx.try_recv() {
//        println!("{:?}",fishes);
//        population.extend(fishes);
//    }
//
//    population
//}


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

    #[test]
    fn test_0x002()
    {
        let lookup = create_lookup();
        let mut population = vec![3,4,3,1,2];
        let total_steps = 256;
        let first_steps = total_steps % HORIZON;
        population = grow_population(population, first_steps);
        let population_size = forecast_population(population, total_steps/HORIZON, &lookup);
        assert_eq!(population_size, 26984457539);
    }
}
