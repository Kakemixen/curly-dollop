use ndarray::prelude::*;
use std::path::Path;
use super::fileops::get_file_lines;

pub fn read_file_grid<P>(path: P)
    -> Array2<usize>
where P: AsRef<Path>
{
    let lines = get_file_lines(path);
    parse_lines(lines)
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

/// Get all indexes horizontally or vertically adjecent
/// Does not return diagonal adjecents
pub fn find_adjecents(grid: &Array2<usize>, pos: (usize, usize))
    -> Vec<(usize,usize)>
{
    let h = pos.0 as i32;
    let w = pos.1 as i32;
    let mut adjecents = Vec::new();

    for h in h-1..=h+1 {
        if h as usize == pos.0
            || h < 0
            || h as usize >= grid.dim().0 {
            continue;
        }
        adjecents.push((h as usize, pos.1));
    }
    for w in w-1..=w+1 {
        if w as usize == pos.1
            || w < 0
            || w as usize >= grid.dim().1 {
            continue;
        }
        adjecents.push((pos.0,w as usize));
    }
    adjecents
}

/// Get all indexes sorrounding pos, including diagonal
pub fn find_sorounding(grid: &Array2<usize>, pos: (usize, usize))
    -> Vec<(usize,usize)>
{
    let h = pos.0 as i32;
    let w = pos.1 as i32;
    let mut sorounding = Vec::new();

    for h in h-1..=h+1 {
        if h < 0
            || h as usize >= grid.dim().0 {
            continue;
        }
        for w in w-1..=w+1 {
            if (w as usize == pos.1 && h as usize == pos.0)
                || w < 0
                || w as usize >= grid.dim().1 {
                continue;
            }
            sorounding.push((h as usize, w as usize));
        }
    }
    sorounding
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_0x0001()
    {
        let grid = Array2::zeros((5,5));
        assert_eq!(find_adjecents(&grid, (0,0)),
            vec![(1,0), (0,1)],
        );
    }

    #[test]
    fn test_0x0002()
    {
        let grid = Array2::zeros((5,5));
        assert_eq!(find_adjecents(&grid, (2,2)),
            vec![(1,2), (3,2), (2,1), (2,3)],
        );
    }

    #[test]
    fn test_0x0003()
    {
        let grid = Array2::zeros((5,5));
        assert_eq!(find_sorounding(&grid, (4,4)),
            vec![(3,3), (3,4), (4,3)],
        );
    }

    #[test]
    fn test_0x0004()
    {
        let grid = Array2::zeros((5,5));
        assert_eq!(find_sorounding(&grid, (2,2)),
            vec![(1,1), (1,2), (1,3),
                 (2,1)       , (2,3),
                 (3,1), (3,2), (3,3),
            ],
        );
    }
}
