use aoclib::fileops;
use itertools::Itertools;
use std::cmp;

fn main() {
    part1();
}

#[derive(Debug, Clone)]
struct Point
{
    x: usize,
    y: usize,
}


#[derive(Debug)]
struct Line
{
    start: Point,
    end:   Point,
}

impl Line
{
    fn is_straight(&self)
        -> bool
    {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn iterate_points(&self)
        -> LineIterator
    {
        LineIterator {
            start: self.start.clone(),
            end:   self.end.clone(),
            i:     0,
        }
    }

}

struct LineIterator
{
    start: Point,
    end:   Point,
    i:     i32,
}

impl Iterator for LineIterator
{
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item>
    {
        let diff_x = self.end.x as i32 - self.start.x as i32;
        let diff_y = self.end.y as i32 - self.start.y as i32;
        if self.i > diff_x.abs() && self.i > diff_y.abs() {
            return None;
        }

        let dir_x = diff_x.signum();
        let dir_y = diff_y.signum();

        let point = Point {
            x: (self.start.x as i32 + ( self.i * dir_x )) as usize,
            y: (self.start.y as i32 + ( self.i * dir_y )) as usize,
        };

        self.i += 1;

        Some(point)
    }
}

struct Map
{
    map: Vec<usize>,
    size_x: usize,
    #[cfg(test)]
    size_y: usize,
}

impl Map
{
    fn new(x: usize, y: usize)
        -> Map
    {
        Map {
            map: vec![0; (x+1) * (y+1)],
            size_x: x+1,
            #[cfg(test)]
            size_y: y+1,
        }
    }

    fn get(&mut self, x: usize, y: usize)
        -> &mut usize
    {
        &mut self.map[self.size_x * y + x]
    }

    #[cfg(test)]
    fn print(&self)
         -> ()
    {
        for i in 0..self.size_y {
            println!("{:?}", &self.map[self.size_x * i .. self.size_x * (i + 1)]);
        }
    }
}

fn part1()
    -> ()
{
    let file_lines = fileops::get_file_lines("input.txt");
    let lines = get_lines(file_lines, false);
    let map = map_lines(lines);
    println!("part1: {}", map.map.iter().fold(0, |acc, p| {
            if *p > 1 {
                return acc + 1;
            }
            acc
        }));
}

fn map_lines(lines: Vec<Line>)
    -> Map
{
    let max_x = lines.iter().fold(0, |acc, l| {
            let extreme = cmp::max(l.end.x, l.start.x);
            if extreme > acc {
                return extreme;
            }
            acc
        }) as usize;
    let max_y = lines.iter().fold(0, |acc, l| {
            let extreme = cmp::max(l.end.y, l.start.y);
            if extreme > acc {
                return extreme;
            }
            acc
        }) as usize;
    let mut map = Map::new(max_x, max_y);

    for line in lines {
        for p in line.iterate_points() {
            let coord = map.get(p.x, p.y);
            *coord += 1;
        }
    }
    map
}

fn get_lines(lines: impl Iterator<Item = String>, allow_diagonal: bool)
    -> Vec<Line>
{
    lines.map(|x| {
        let (start, end) = x.split(" -> ")
            .map(|p| {
                let (x,y) = p.split(",")
                    .map(|x| {
                        x.parse::<usize>().expect("fail to convert number")
                    })
                    .next_tuple() .expect("less than two elements in point");
                Point { x, y }
            }).next_tuple().expect("no next tuple");
        Line { start, end }
    })
    .filter(|l| {
        allow_diagonal || l.is_straight()
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0x0001()
    {
        // test 5x5 grid where all lines go from top left to bot right
        let file = vec![
            "0,0 -> 5,0",
            "0,0 -> 0,5",
            "0,3 -> 2,3",
            "0,5 -> 4,5",
            "2,2 -> 2,5",
        ];
        let file_lines = file.iter()
            .map(|x| {x.to_string()});
        let lines = get_lines(file_lines, false);
        let map = map_lines(lines);
        map.print();
        let score = map.map.iter().fold(0, |acc, p| {
                if *p > 1 {
                    return acc + 1;
                }
                acc
            });
        println!("part1: {}", score);
        assert_eq!(score, 5);
    }

    #[test]
    fn test_0x0002()
    {
        // test 5x5 grid where some lines go from bot right to top left
        let file = vec![
            "0,0 -> 5,0",
            "0,0 -> 0,5",
            "0,3 -> 2,3",
            "4,5 -> 0,5",
            "2,5 -> 2,2",
        ];
        let file_lines = file.iter()
            .map(|x| {x.to_string()});
        let lines = get_lines(file_lines, false);
        let map = map_lines(lines);
        map.print();
        let score = map.map.iter().fold(0, |acc, p| {
                if *p > 1 {
                    return acc + 1;
                }
                acc
            });
        println!("part1: {}", score);
        assert_eq!(score, 5);
    }
}
