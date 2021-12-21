use aoclib::gridops;
use ndarray::prelude::*;

fn main() {
    part1();
    part2();
}

fn part1()
    -> ()
{
    let mut grid = gridops::read_file_grid("input.txt");
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += simulate_step(&mut grid);
    }
    println!("part1 {}", flashes);
}

fn part2()
    -> ()
{
    let mut grid = gridops::read_file_grid("input.txt");
    let mut i = 0;
    while grid != Array2::zeros((10,10)) {
        simulate_step(&mut grid);
        i += 1;
    }
    println!("part2 {}", i);
}

fn simulate_step(grid: &mut Array2<usize>)
    -> usize
{
    for value in grid.iter_mut() {
        *value += 1;
    }

    let who_flashed = flash(grid, vec![]);

    for flasher in &who_flashed {
        grid[*flasher] = 0;
    }

    who_flashed.len()
}

fn flash(grid: &mut Array2<usize>
        , mut who_flashed: Vec<(usize,usize)>)
    -> Vec<(usize,usize)>
{
    let mut someone_flashed = false;
    let mut new_flashers = Vec::new();

    for (idx, value) in grid.indexed_iter() {
        if *value > 9 && !who_flashed.contains(&idx) {
            new_flashers.push(idx);
            someone_flashed = true;
        }
    }

    for flasher in &new_flashers {
        let sorounding = gridops::find_sorounding(grid, *flasher);
        for point in sorounding {
            grid[point] += 1;
        }
    }

    who_flashed.extend(new_flashers);
    if someone_flashed {
        who_flashed = flash(grid, who_flashed);
    }
    who_flashed
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    /// early iterations
    fn test_0x0001()
    {
        let mut grid = gridops::read_file_grid("test_input.txt");
        let mut flashes = 0;
        flashes += simulate_step(&mut grid);
        assert_eq!(flashes, 0);
        flashes += simulate_step(&mut grid);
        assert_eq!(flashes, 35);
        flashes += simulate_step(&mut grid);
        assert_eq!(flashes, 35+45);
    }

    #[test]
    /// early iterations
    fn test_0x0002()
    {
        let mut grid = gridops::read_file_grid("test_input.txt");
        let mut flashes = 0;
        for _ in 0..100 {
            flashes += simulate_step(&mut grid);
        }
        assert_eq!(flashes, 1656);
    }

    #[test]
    /// test synchronizing assumption in text
    fn test_0x0003()
    {
        let mut grid = gridops::read_file_grid("test_input.txt");
        let mut flashes = 0;
        for _ in 0..195 {
            flashes += simulate_step(&mut grid);
        }
        assert_eq!(grid, Array2::zeros((10,10)));
    }
}
