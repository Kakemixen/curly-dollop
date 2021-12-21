use aoclib::fileops;

fn main() {
    part1();
    part2();
}

fn part1()
    -> ()
{
    let lines = fileops::get_file_lines("input.txt");
    let score: usize = lines.map( | x | {
        let corrupted = find_error(&x);
        score_error(&corrupted)
    }).sum();
    println!("part1 {}", score);
}

fn part2()
    -> ()
{
    println!("part2 {}", "TODO");
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

fn find_error(line: &str)
    -> Chunk
{
    let mut stack = Vec::new();
    for c in line.chars() {
        let chunk = Chunk::new(c);

        match chunk
        {
        Chunk::Open(b) => {
            println!("pushing opened chunk {:?}", b);
            stack.push(b)
        },
        Chunk::Close(b) => {
            if let Some(top) = stack.pop() {
                println!("got chunk {:?} - last opened chunk {:?}", b, top);
                if top != b {
                    return Chunk::Close(b);
                }
            }
            else {
                println!("no more stack, closing unopened chunk");
                return Chunk::Close(b);
            }
        },
        Chunk::Invalid => {
            unreachable!("send umiddelbar hjelp");
        },
        }
    }
    Chunk::Invalid
}

fn score_error(error_chunk: &Chunk)
    -> usize
{
    match error_chunk
    {
    // corruption errors
    &Chunk::Close(Bracket::Round) => 3,
    &Chunk::Close(Bracket::Square) => 57,
    &Chunk::Close(Bracket::Curly) => 1197,
    &Chunk::Close(Bracket::Angular) => 25137,

    _ => 0,
    }
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
            if let Chunk::Invalid = corrupted { continue; }
            eprintln!("got corrupted {:?}", corrupted);
            assert_eq!(corrupted, corruptions[i]);
        }

    }
    #[test]
    /// test finding correct corruptions
    fn test_0x0002()
    {
        let lines = fileops::get_file_lines("test_input.txt");
        let score: usize = lines.map( | x | {
            let corrupted = find_error(&x);
            score_error(&corrupted)
        }).sum();
        assert_eq!(score, 26397);
    }
}
