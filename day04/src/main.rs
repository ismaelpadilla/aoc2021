use regex::Regex;
use std::fs;

#[derive(Debug, Copy, Clone)]
struct BingoNumber {
    number: u32,
    drawn: bool,
}

impl BingoNumber {
    fn new() -> Self {
        BingoNumber {
            number: 0,
            drawn: false,
        }
    }

    fn line_to_bingo_number_array(line: &str) -> [BingoNumber; 5] {
        let mut ret: [BingoNumber; 5] = [BingoNumber::new(); 5];
        let re = Regex::new("\\s+").unwrap();
        // println!("parsing line {}", line);
        let numbers = re.split(line).filter(|x| !x.is_empty());
        for (pos, number) in numbers.enumerate() {
            // println!("number -{}-", number);
            match number.parse() {
                Ok(n) => ret[pos].number = n,
                Err(_) => break,
            }
        }
        ret
    }
}

#[derive(Debug)]
struct Board {
    board: [[BingoNumber; 5]; 5],
}

impl Board {
    fn check_number(&mut self, number: u32) -> bool {
        let mut checked = false;
        for row in self.board.iter_mut() {
            for mut bingo_number in row {
                if bingo_number.number == number {
                    bingo_number.drawn = true;
                    checked = true;
                    // println!("{:?}", bingo_number);
                }
            }
        }
        // if checked {
        //     println!("{:?}", self);
        // }
        checked
    }

    // returnes true iif all numbers in a row or column are marked as drawn
    fn is_complete(&self) -> bool {
        let mut completed_in_column = [0u32; 5];
        for (_rowpos, row) in self.board.iter().enumerate() {
            let mut completed_in_row = 0;
            for (pos, bingo_number) in row.iter().enumerate() {
                if bingo_number.drawn {
                    completed_in_column[pos] += 1;
                    completed_in_row += 1;
                }
            }
            if completed_in_row == 5 {
                return true
            }
            // if row.iter().all(|bingo_number| bingo_number.drawn) {
            //     return true
            // }
        }
        completed_in_column.iter().any(|x| x == &5)
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0u32;
        for row in self.board {
            for bingo_number in row {
                if !bingo_number.drawn {
                    sum += bingo_number.number;
                }
            }
        }
        sum
    }
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let contents = fs::read_to_string("input").expect("Error while reading file");

    let mut lines = contents.lines();
    let first_line = lines.next().expect("There should be at least one line");
    // consume blank line:
    lines.next();

    let numbers: Vec<u32> = first_line.split(",").map(|n| n.parse().unwrap()).collect();

    let mut boards: Vec<Board> = Vec::new();

    let lines_collected: Vec<&str> = lines.collect();
    let lines_chunks = lines_collected.chunks(6);
    for chunk in lines_chunks {
        // println!("{}", chunk[0]);
        // println!("{}", chunk[1]);
        // println!("{}", chunk[2]);
        // println!("{}", chunk[3]);
        // println!("{}", chunk[4]);
        let board = Board {
            board: [
                BingoNumber::line_to_bingo_number_array(chunk[0]),
                BingoNumber::line_to_bingo_number_array(chunk[1]),
                BingoNumber::line_to_bingo_number_array(chunk[2]),
                BingoNumber::line_to_bingo_number_array(chunk[3]),
                BingoNumber::line_to_bingo_number_array(chunk[4]),
            ],
        };
        boards.push(board);
    }

    'outer: for number in numbers {
        for (boardnr, board) in boards.iter_mut().enumerate() {
            if board.check_number(number) {
                if board.is_complete() {
                    println!("board {} complete", boardnr);
                    let unmarked_sum = board.sum_unmarked();
                    println!("score: {}", unmarked_sum*number);
                    break 'outer;
                }
            }
        }
    }
    // for board in boards.iter() {
    //     println!("{:?}\n", board);
    // }
}

fn part_2() {
    let contents = fs::read_to_string("input").expect("Error while reading file");

    let mut lines = contents.lines();
    let first_line = lines.next().expect("There should be at least one line");
    // consume blank line:
    lines.next();

    let numbers: Vec<u32> = first_line.split(",").map(|n| n.parse().unwrap()).collect();

    let mut boards: Vec<Board> = Vec::new();

    let lines_collected: Vec<&str> = lines.collect();
    let lines_chunks = lines_collected.chunks(6);
    for chunk in lines_chunks {
        // println!("{}", chunk[0]);
        // println!("{}", chunk[1]);
        // println!("{}", chunk[2]);
        // println!("{}", chunk[3]);
        // println!("{}", chunk[4]);
        let board = Board {
            board: [
                BingoNumber::line_to_bingo_number_array(chunk[0]),
                BingoNumber::line_to_bingo_number_array(chunk[1]),
                BingoNumber::line_to_bingo_number_array(chunk[2]),
                BingoNumber::line_to_bingo_number_array(chunk[3]),
                BingoNumber::line_to_bingo_number_array(chunk[4]),
            ],
        };
        boards.push(board);
    }

    let mut completed_boards = 0u32;
    let total_boards: u32 = boards.len().try_into().unwrap();
    let mut completed = vec!(false; boards.len()); 

    'outer: for number in numbers {
        for (boardnr, board) in boards.iter_mut().enumerate() {
            if board.check_number(number) {
                if !completed[boardnr] && board.is_complete() {
                    completed[boardnr] = true;
                    completed_boards += 1;
                    println!("board {} complete. {} boards completed so far", boardnr, completed_boards);

                    if completed_boards == total_boards { 
                        println!("This was the last board");
                        let unmarked_sum = board.sum_unmarked();
                        println!("score: {}", unmarked_sum*number);
                        break 'outer;
                    }
                }
            }
        }
    }
    // for board in boards.iter() {
    //     println!("{:?}\n", board);
    // }
}
