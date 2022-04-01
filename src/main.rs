use std::fs::read;
use std::fs::read_to_string;
// AOC Day4A

fn main() {
    //-----------------------------------------------------------------------
    // after the 4th number is called, a win is possible so start the checks
    //
    // checks: (per board)
    // for each called number, take a board and check line by line if it matches,
    // if yes: check if number to the right is in the called number list
    // !(only numbers up to the current called number)
    // if index 0 matches: keep counting  (no index zero then quit the line)
    // if no line match, go to column match check:
    // for each line check if index 0 is a match, yes? keep going, no, quit column
    //-----------------------------------------------------------------------

    //-------------  Build backlog of numbers to call -----------------------------------

    let data_string = read_to_string("./4a-numbers.txt").unwrap();

    let backlog_nums = data_string
        .split(",")
        .filter(|x| *x != "\n")
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    println!("backlog_nums: {:?}", backlog_nums);

    //----------------- Build boards ---------------------------------------------------

    let boards_string = read_to_string("./test-data.txt").unwrap();
    let alternative = boards_string.replace("\n", ",");
    let replace_spaces = alternative.replace(",", " ");
    let split_stuff = replace_spaces.split(" ").collect::<Vec<&str>>();
    let remove_empties = split_stuff
        .into_iter()
        .filter(|x| x.len() > 0)
        .collect::<Vec<&str>>();
    let all_board_nums = remove_empties
        .iter()
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    println!("all_board_nums{:?}", all_board_nums);

    let result = build_single_row(all_board_nums);
    println!("result{:?}", result);
}

fn build_single_row(boards: Vec<u8>) -> Vec<u8> {
    //let mut board_vec: Vec<Vec<u8>> = Vec::new();
    let mut count: u8 = 0;
    let mut row_vec: Vec<u8> = Vec::new();
    boards.iter().enumerate().for_each(|(i, &n)| {
        if count < 5 {
            println!("n: {:?}", &n);
            count += 1;
            row_vec.push(n);
        }
    });
    row_vec
}
