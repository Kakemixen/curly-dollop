use aoclib::fileops;
use itertools::Itertools;
use std::str::Chars;

fn main() {
    part1();
    part2();
}

fn part1()
    -> ()
{
    let lines = fileops::get_file_lines("input.txt");
    let parsed = parse_text(lines);
    let easies = get_easies(&parsed);
    println!("part1 {}", easies.len());
}

fn part2()
    -> ()
{
    println!("part2 {}", "TODO");
}

#[derive(Debug,PartialEq,Eq)]
enum Segment
{
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug)]
struct Digit
{
    segments: Vec<Segment>,
}

impl Digit
{
    fn new(text_rep: Chars)
        -> Digit
    {
        Digit { 
            segments: text_rep.map(|x| {
                match x 
                {
                    'a' => Segment::A,
                    'b' => Segment::B,
                    'c' => Segment::C,
                    'd' => Segment::D,
                    'e' => Segment::E,
                    'f' => Segment::F,
                    'g' => Segment::G,
                     x  => unreachable!("Cannot parse char: {}", x),
                }
            }).collect(),
        }
    }
}

#[derive(Debug)]
struct Entry
{
    uniques: Vec<Digit>,
    output: Vec<Digit>,
}

fn parse_text(lines: impl Iterator<Item = String>)
    -> Vec<Entry>
{
    let parse_digits = | digits: &str | {
        digits.split(" ")
            .filter(|x| { !x.is_empty() })
            .map(|x| { Digit::new(x.chars()) })
            .collect()
    };

    lines.map(|x| {
        let (uniques, output) = x.split("|").next_tuple()
            .expect("malformed input");
        Entry {
            uniques: parse_digits(uniques),
            output: parse_digits(output),
        }
    }).collect()
}

                                       //1,7,4,8
const UNIQUE_NUM_SEGMENTS: [usize; 4] = [2,3,4,7];

fn get_easies(entries: &Vec<Entry>)
    -> Vec<&Digit>
{
    let mut ret = Vec::new();
    for entry in entries {
        for o in &entry.output{
            if UNIQUE_NUM_SEGMENTS.contains(&o.segments.len()) {
                ret.push(o);
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests
{
    use super::*;

    fn test_input()
        -> impl Iterator<Item = String>
    {
        fileops::get_file_lines("test_input.txt")
    }

    #[test]
    /// parse text properly
    fn test_0x0001()
    {
        let input_text = test_input();
        let parsed = parse_text(input_text);
        assert_eq!(parsed.len(), 10);
        assert_eq!(parsed[0].uniques.len(), 10);
        assert_eq!(parsed[0].uniques[0].segments.len(), 2);
        assert_eq!(parsed[0].uniques[0].segments[0], Segment::B);
        assert_eq!(parsed[0].uniques[0].segments[1], Segment::E);
        assert_eq!(parsed[0].output.len(), 4);
        assert_eq!(parsed[0].output[1].segments.len(), 5);
        assert_eq!(parsed[0].output[1].segments[0], Segment::C);
        assert_eq!(parsed[0].output[1].segments[1], Segment::E);
        assert_eq!(parsed[0].output[1].segments[2], Segment::F);
        assert_eq!(parsed[0].output[1].segments[3], Segment::D);
        assert_eq!(parsed[0].output[1].segments[4], Segment::B);
    }

    #[test]
    /// parse count easy
    fn test_0x0002()
    {
        let input_text = test_input();
        let parsed = parse_text(input_text);
        let easies = get_easies(&parsed);
        assert_eq!(easies.len(), 26);
    }
}
