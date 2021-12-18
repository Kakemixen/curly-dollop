use aoclib::{fileops, threadpool};
use std::ops::AddAssign;
use std::sync::{
    Arc, Mutex,
};
use std::time;

//const HORIZON: usize = 78; //tuning parameter
const HORIZON: usize = 120; //tuning parameter

type LookupTable = Arc<Mutex<Vec<Vec<u8>>>>;

fn main() {
    let lookup = create_lookup();
    part1(Arc::clone(&lookup));
    part2(Arc::clone(&lookup));
}

fn part1(lookup: LookupTable)
    -> ()
{
    let total_steps = 80;
    let mut population = parse_input();
    let first_steps = total_steps % HORIZON;
    population = grow_population(population, first_steps);
    let population_size = forecast_population(population, total_steps/HORIZON, lookup);
    println!("part1 {}", population_size);
}

fn part2(lookup: LookupTable)
    -> ()
{
    let now = time::Instant::now();
    let total_steps = 256;
    let mut population = parse_input();
    let first_steps = total_steps % HORIZON;
    population = grow_population(population, first_steps);
    let population_size = forecast_population(population, total_steps/HORIZON, lookup);
    println!("part2 {}", population_size);
    println!("part2 time {}", now.elapsed().as_secs());
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
    -> Arc<Mutex<Vec<Vec<u8>>>>
{
    let vec = Arc::new(Mutex::new(Vec::new()));
    for i in 0 ..= 8 {
        vec.lock().unwrap().push(
            grow_population(vec![i], HORIZON)
            );
    }
    vec
}

fn grow_population_lookup(
    mut population: Vec<u8>,
    iterations: usize,
    lookup:     LookupTable,
)
    -> Vec<u8>
{
    let mut tmp = Vec::new();
    for _ in 0..iterations {
        for fish in population {
            tmp.extend(&lookup.lock().expect("poisoned lookuptable!")[fish as usize].to_vec());
        }
        population = tmp;
        tmp = Vec::new();
    }
    population
}

fn forecast_population(
    population: Vec<u8>,
    iterations: usize,
    lookup:     LookupTable,
)
    -> usize
{
    // runs in two minutes with HORIZON=120
    return forecast_population_recursion(population, iterations, 0, lookup);

    // cant run with more than 80 horizon, cus stackoverflow. is slower
    //return forecast_population_recursion_thread_split(population, iterations, lookup);
}

#[allow(unused)]
fn forecast_population_recursion_thread_split(
    population: Vec<u8>,
    iterations: usize,
    lookup:     LookupTable,
)
    -> usize
{
    assert!(HORIZON < 80); // because of issues with thread stackoverflow
    let current = 0;
    let population_size = Arc::new(Mutex::new(0));
    { // scope for threadpool
        let threadpool = threadpool::ThreadPool::new(42);

        for i in 0..population.len() {
            let fish = population[i];
            let iters = iterations.clone();
            let next = current + 1;
            let lookup_clone = Arc::clone(&lookup);
            let population_size_clone = Arc::clone(&population_size);
            let func = move | | {
                let ret = forecast_population_recursion(
                    grow_population_lookup(vec![fish], 1, Arc::clone(&lookup_clone)),
                    iters,
                    next,
                    lookup_clone,
                );
                population_size_clone.lock().expect("cannot update popsize").add_assign(ret);
            };
            threadpool.execute(func);
        }
    }
    let x = population_size.lock().expect("can't return population_size").to_owned();
    x
}

fn forecast_population_recursion(
    population: Vec<u8>,
    iterations: usize,
    current:    usize,
    lookup:     LookupTable,
)
    -> usize
{
    if current == iterations {
        return population.len();
    }

    let mut population_size = 0;

    //for fish in population {
    for i in 0..population.len() {
        let fish = population[i];
        population_size += forecast_population_recursion(
                grow_population_lookup(vec![fish], 1, Arc::clone(&lookup)),
                iterations,
                current + 1,
                Arc::clone(&lookup),
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
        assert_eq!(final_population.len(), 26);
        let final_population = grow_population(final_population, 80-18);
        assert_eq!(final_population.len(), 5934);
    }

    #[test]
    fn test_0x002()
    {
        let lookup = create_lookup();
        let mut population = vec![3,4,3,1,2];
        let total_steps = 256;
        let first_steps = total_steps % HORIZON;
        population = grow_population(population, first_steps);
        let population_size = forecast_population(population, total_steps/HORIZON, lookup);
        assert_eq!(population_size, 26984457539);
    }
}
