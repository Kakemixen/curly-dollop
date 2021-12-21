use aoclib::fileops;
use itertools::Itertools;

fn main() {
    part1();
    part2();
}

fn part1()
    -> ()
{
    let lines = fileops::get_file_lines("input.txt");
    let score: usize = lines.map( | x | {
        if let LineFix::Corrupted(corrupted) = find_error(&x){
            score_corruption(&corrupted)
        } else {
            0
        }
    }).sum();
    println!("part1 {}", score);
}

fn part2()
    -> ()
{
    let lines = fileops::get_file_lines("input.txt");
    let fixes: Vec<LineFix> = lines.map( |x| {
        find_error(&x)
    }) .collect();
    let score = score_completions(&fixes);
    println!("part2 {}", score);
}

#[derive(Debug, PartialEq, Eq)]
enum Bracket
{
    Round,   //Parentheses,
    Square,  //Bracket,
    Curly,   //Brace,
    Angular, //Chevron,
}

#[derive(Debug, PartialEq, Eq)]
enum Chunk
{
    Open(Bracket),
    Close(Bracket),
    Invalid,
}

impl Chunk
{
    fn new(c: char)
        -> Chunk
    {
        match c 
        {
        '(' => Chunk::Open(Bracket::Round),
        ')' => Chunk::Close(Bracket::Round),
        '[' => Chunk::Open(Bracket::Square),
        ']' => Chunk::Close(Bracket::Square),
        '{' => Chunk::Open(Bracket::Curly),
        '}' => Chunk::Close(Bracket::Curly),
        '<' => Chunk::Open(Bracket::Angular),
        '>' => Chunk::Close(Bracket::Angular),
        ___ => Chunk::Invalid,
        }
    }
}

enum LineFix
{
    Corrupted(Chunk),
    Incomplete(Vec<Bracket>),
}

fn find_error(line: &str)
    -> LineFix
{
    let mut stack = Vec::new();
    for c in line.chars() {
        let chunk = Chunk::new(c);

        match chunk
        {
        Chunk::Open(b) => {
            stack.push(b)
        },
        Chunk::Close(b) => {
            if let Some(top) = stack.pop() {
                if top != b {
                    return LineFix::Corrupted(Chunk::Close(b));
                }
            }
            else {
                return LineFix::Corrupted(Chunk::Close(b));
            }
        },
        Chunk::Invalid => {
            unreachable!("send umiddelbar hjelp");
        },
        }
    }
    LineFix::Incomplete(stack)
}

fn score_corruption(error_chunk: &Chunk)
    -> usize
{
    match error_chunk
    {
    Chunk::Close(Bracket::Round) => 3,
    Chunk::Close(Bracket::Square) => 57,
    Chunk::Close(Bracket::Curly) => 1197,
    Chunk::Close(Bracket::Angular) => 25137,
    _ => 0,
    }
}

fn score_completions(lines: &Vec<LineFix>)
    -> usize
{
    let scores: Vec<usize> = lines.iter().filter( | x | {
            if let LineFix::Incomplete(_) = x {
                true
            } else {
                false
            }
        }).map(|x| {
            if let LineFix::Incomplete(stack) = x {
                get_completion_score(stack)
            } else {
                panic!();
            }
        }).sorted().collect();
    assert!(scores.len() % 2 == 1);
    scores[scores.len() / 2]
}

fn get_completion_score(stack: &Vec<Bracket>)
    -> usize
{
    stack.iter().rev().fold(0, |acc, x| {
        let x_score = match x
            {
            &Bracket::Round => 1,
            &Bracket::Square => 2,
            &Bracket::Curly => 3,
            &Bracket::Angular => 4,
            };
        acc*5 + x_score
    })
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    /// test finding correct corruptions
    fn test_0x0001()
    {
        let lines = fileops::get_file_lines("test_input.txt");
        let corruptions = vec![
                Chunk::Invalid,
                Chunk::Invalid,
                Chunk::Close(Bracket::Curly),
                Chunk::Invalid,
                Chunk::Close(Bracket::Round),
                Chunk::Close(Bracket::Square),
                Chunk::Invalid,
                Chunk::Close(Bracket::Round),
                Chunk::Close(Bracket::Angular),
                Chunk::Invalid,
            ];
        for (i,line) in lines.enumerate() {
            let corrupted = find_error(&line);
            if let LineFix::Corrupted(error) = corrupted {
                eprintln!("got corrupted {:?}", error);
                assert_eq!(error, corruptions[i]);
            }
        }

    }

    #[test]
    /// test finding correct corruptions
    fn test_0x0002()
    {
        let lines = fileops::get_file_lines("test_input.txt");
        let score: usize = lines.map( | x | {
            if let LineFix::Corrupted(corrupted) = find_error(&x){
                score_corruption(&corrupted)
            } else {
                0
            }
        }).sum();
        assert_eq!(score, 26397);
    }

    #[test]
    /// test finding correct completion scores
    fn test_0x0003()
    {
        let lines = fileops::get_file_lines("test_input.txt");
        let fixes: Vec<LineFix> = lines.map( |x| {
            find_error(&x)
        }) .collect();
        let score = score_completions(&fixes);
        assert_eq!(score, 288957);
    }
}
