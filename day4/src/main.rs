use aoclib::fileops;

#[derive(Debug,Clone,Copy)]
struct BingoEntry
{
    value: u32,
    matched: bool,
}


struct BingoBoard
{
    board: Vec<BingoEntry>,
    score: u32,
    winturn: u32,
}

fn main() {
    let part1 = best_board_score();
    println!("part1: {}", part1);
    let part2 = last_board_score();
    println!("part2: {}", part2);
}

fn best_board_score()
    -> u32
{
    let (draws, mut boards) = get_bingo_boards();
    for board in &mut boards {
        calculate_board_score(&draws, board).unwrap();
    }

    let initial = BingoBoard {
        board: Vec::new(), score: 0, winturn: draws.len() as u32,
    };
    let best = boards.iter().fold( &initial,
    |acc, b| {
        if b.winturn < acc.winturn {
            b
        } else {
            acc
        }
    });

    best.score
}

fn last_board_score()
    -> u32
{
    let (draws, mut boards) = get_bingo_boards();
    for board in &mut boards {
        calculate_board_score(&draws, board).unwrap();
    }

    let initial = BingoBoard {
        board: Vec::new(), score: 0, winturn: 0 as u32,
    };
    let best = boards.iter().fold( &initial,
    |acc, b| {
        if b.winturn > acc.winturn {
            b
        } else {
            acc
        }
    });

    best.score
}

fn calculate_board_score(draws: &Vec<u32>, board: &mut BingoBoard)
    -> Result<(u32, u32), &'static str>
{
    for (i, draw) in draws.iter().enumerate() {
        mark_draw(draw, board);
        if check_win(board) {
            board.score = board.board.iter()
                .filter(|x| { !x.matched })
                .fold(0, |sum, x| { sum + x.value })
                * draw;
            board.winturn = i as u32 + 1;
            return Ok((board.winturn, board.score));
        }
    }
    Err("Board did not win!")
}

fn mark_draw(draw: &u32, board: &mut BingoBoard)
    -> ()
{
    for entry in &mut board.board {
        if entry.value == *draw {
            if entry.matched == true {
                panic!("matching on matched number {:?}", entry);
            }
            entry.matched = true;
        }
    }
}

fn check_win(board: &mut BingoBoard)
    -> bool
{
    let board_dim = (board.board.len() as f64).sqrt() as usize;
    let matches: Vec<u8> = board.board.iter().map(|x| { x.matched as u8 }).collect();
    for i in 0..board_dim {
        let row = &matches[(i*board_dim) .. ((i+1)*board_dim)];
        if row.iter().sum::<u8>() == board_dim as u8 {
            return true;
        }

        let c = i;
        let mut c_i = -1;
        let col: Vec<u8> = matches.iter().filter(|_| {
                c_i+=1; c_i as usize % board_dim == c
            }).map(|x| {*x}).collect();
        if col.iter().sum::<u8>() == board_dim as u8 {
            return true;
        }
    }

    false
}


fn get_bingo_boards()
    -> (Vec<u32>, Vec<BingoBoard>)
{
    let mut lines = fileops::get_file_lines("input.txt");
    let numberdraws: Vec<u32> = lines.next().unwrap()
        .split(",")
        .map(|x| {
            x.parse::<u32>().expect("could not convert to u32")
        })
        .collect();
    lines.next(); // empty line after draws
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut board_acc: Vec<BingoEntry> = Vec::new();
    for line in lines {
        match line.as_str() {
            "" => { // I added a newline on end of input to make this work
                boards.push(BingoBoard {
                    board: board_acc,
                    score: 0,
                    winturn: 0,
                });
                board_acc = Vec::new();
            },
            vals => {
                let bingovals: Vec<BingoEntry> = vals.split(" ")
                    .filter(|x| {
                        // if only one digit, two spaces between characters
                        !x.is_empty()
                    })
                    .map(|x| {
                        BingoEntry {
                            value: x.parse::<u32>()
                                .expect("u32 conversion board error"),
                            matched: false,
                        }
                    }).collect();
                board_acc.extend(bingovals);
            },
        }
    }
    (numberdraws, boards)
}

