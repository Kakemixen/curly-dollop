use aoclib::fileops;
use itertools::Itertools;
use std::str::Chars;
use std::collections::HashMap;

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
    let lines = fileops::get_file_lines("input.txt");
    let parsed = parse_text(lines);
    let decoded_outputs = decode_outputs(&parsed);
    println!("part2 {}", decoded_outputs.iter().sum::<usize>());
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Segment
{
    A = 0b0000001,
    B = 0b0000010,
    C = 0b0000100,
    D = 0b0001000,
    E = 0b0010000,
    F = 0b0100000,
    G = 0b1000000,
}

impl Segment
{
    fn to_bitmap(segments: &[Segment])
        -> usize
    {
        let mut ret = 0;
        for seg in segments {
            ret += seg.clone() as usize
        }
        ret
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
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

fn decode_outputs(entries: &Vec<Entry>)
    -> Vec<usize>
{
    let mut output = Vec::with_capacity(entries.len());
    for entry in entries {
        let configuration = find_configuration(&entry.uniques);
        let mut number = 0;

        for (i, digit) in entry.output.iter().rev().enumerate() {
            let num = configuration.get(&Segment::to_bitmap(&digit.segments)).expect("digit not configured");
            number += num * (10usize.pow(i as u32));
        }

        output.push(number);

    }
    output
}

fn find_configuration(uniques: &Vec<Digit>)
    -> HashMap<usize, usize>
{
    let mut configuration = HashMap::new();

    let mut right_side = Vec::new();
    let mut top = Vec::new();
    let mut four = Vec::new();

    for unique in uniques {
        let is_easy = UNIQUE_NUM_SEGMENTS.contains(&unique.segments.len());
        match unique.segments.len()
        {
            2 => {
                right_side = unique.segments.clone();
                configuration.insert(unique.segments.clone(), 1);
            },
            3 => {
                top = unique.segments.clone();
                configuration.insert(unique.segments.clone(), 7);
            },
            4 => {
                configuration.insert(unique.segments.clone(), 4);
                four = unique.segments.clone();
            },
            7 => {
                configuration.insert(unique.segments.clone(), 8);
            },
            _ => {},
        }
    }

    // top is 7 without the right side
    for seg in &right_side {
        if top.contains(&seg) {
            top.retain(|x| { x != seg });
        }
    }
    let top = top[0].clone();

    // to separate two segments on the right, count times they appear alone. top:1 bot:2
    let mut counts = vec![0,0];
    for unique in uniques {
        if unique.segments.contains(&right_side[0])
            && !unique.segments.contains(&right_side[1])
        {
            counts[0] += 1;
            continue;
        }

        if !unique.segments.contains(&right_side[0])
            && unique.segments.contains(&right_side[1])
        {
            counts[1] += 1;
        }
    }
    let right_bot = if counts[0] > counts[1] {
        right_side[0].clone()
    } else {
        right_side[1].clone()
    };
    right_side.retain(|x| { *x != right_bot });
    let right_top = right_side[0].clone();

    // we can now find 2
    let mut not_left_top = Vec::new();
    for unique in uniques {
        if unique.segments.contains(&right_top)
            && !unique.segments.contains(&right_bot)
        {
            configuration.insert(unique.segments.clone(), 2);
            not_left_top = unique.segments.clone();
            continue;
        }
    }

    // segment in 4 but not in 2 and not is right_bot is left_top
    let mut left_top = four[0].clone();
    for seg in &four {
        if !not_left_top.contains(&seg) 
            && *seg != right_bot
        {
            left_top = seg.clone();
            break;
        }
    }
    let left_top = left_top;

    // unidentified segment in four is mid
    let mut mid = four[0].clone();
    for seg in four {
        if seg != right_top
            && seg != left_top
            && seg != right_bot
        {
            mid = seg.clone();
        }
    }

    // the remainder of the segs in two can differ by count
    let mut remainder = Vec::new();
    for seg in not_left_top {
        if  seg != top
            && seg != right_top
            && seg != left_top
            && seg != right_bot
            && seg != mid
        {
            remainder.push(seg.clone());
        }
    }

    let mut counts = vec![0,0];
    for unique in uniques {
        if unique.segments.contains(&remainder[0]) {
            counts[0] += 1;
        }
        if unique.segments.contains(&remainder[1]) {
            counts[1] += 1;
        }
    }
    // bot mid is more common
    let bot = if counts[0] > counts[1] {
        remainder[0].clone()
    } else {
        remainder[1].clone()
    };
    remainder.retain(|x| { *x != bot });
    let left_bot = remainder[0].clone();

    // get the last ones 0,3,5,6,9
    for unique in uniques {
        let segments = &unique.segments;
        if segments.contains(&top)
            && segments.contains(&left_top)
            && segments.contains(&right_top)
            && segments.contains(&mid)
            && !segments.contains(&left_bot)
            && segments.contains(&right_bot)
            && segments.contains(&bot)
        {
            configuration.insert(segments.clone(), 9);
        }

        if segments.contains(&top)
            && segments.contains(&left_top)
            && !segments.contains(&right_top)
            && segments.contains(&mid)
            && segments.contains(&right_bot)
            && segments.contains(&bot)
        {
            configuration.insert(segments.clone(), 5);
        }

        if segments.contains(&top)
            && !segments.contains(&left_top)
            && segments.contains(&right_top)
            && segments.contains(&mid)
            && segments.contains(&right_bot)
            && segments.contains(&bot)
        {
            configuration.insert(segments.clone(), 3);
        }

        if segments.contains(&top)
            && segments.contains(&left_top)
            && !segments.contains(&right_top)
            && segments.contains(&mid)
            && segments.contains(&left_bot)
            && segments.contains(&right_bot)
            && segments.contains(&bot)
        {
            configuration.insert(segments.clone(), 6);
        }

        if segments.contains(&top)
            && segments.contains(&left_top)
            && segments.contains(&right_top)
            && !segments.contains(&mid)
            && segments.contains(&left_bot)
            && segments.contains(&right_bot)
            && segments.contains(&bot)
        {
            configuration.insert(segments.clone(), 0);
        }
    }


    //println!(" {:?} \n{:?} {:?}\n {:?} \n{:?} {:?}\n {:?}",
    //         top, left_top, right_top, mid, left_bot, right_bot, bot);

    let mut bitmap_configuration = HashMap::new();
    for (key, num) in &configuration {
        //println!("map: {:?} -> {:?}", key, num);
        bitmap_configuration.insert(Segment::to_bitmap(key), *num);
    }

    //assert!(false);
    bitmap_configuration
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

    #[test]
    /// decode outputs
    fn test_0x0003()
    {
        let input_text = test_input();
        let parsed = parse_text(input_text);
        let decoded_outputs = decode_outputs(&parsed);
        assert_eq!(decoded_outputs[0], 8394);
        assert_eq!(decoded_outputs[1], 9781);
        assert_eq!(decoded_outputs[2], 1197);
        assert_eq!(decoded_outputs.iter().sum::<usize>(), 61229);
    }
}
