use std::fs::read_to_string;
// AOC Day4A

fn main() {
    //-------------  Build backlog of numbers to call -------------------------------

    let data_string = read_to_string("./4a-numbers.txt").unwrap();

    let backlog_nums = data_string
        .split(",")
        .filter(|x| *x != "\n")
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    println!("backlog_nums: {:?}", backlog_nums);

    //----------------- Build one vec of boards nums ---------------------------------

    let boards_string = read_to_string("./4a-bingo-boards.txt").unwrap();
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

    // let all_boards_vec = build_vec_of_boards(all_board_nums, Vec::new());
    // println!("all_boards_vec: {:?}", all_boards_vec);
    // println!("first board: {:?}", all_boards_vec[0]);

    //---------- Build vec of board number tuples with 'picked' bool (u8,bool) -----------

    let tuple_vec = all_boards_tuple(all_board_nums);
    let check_all_boards = backlog_nums
        .iter()
        .map(|&n| check_backlog_against_boards(&tuple_vec, n))
        .collect::<Vec<(u8, bool)>>();
    println!("check_all_boards: {:?}", check_all_boards);
}

//---------- Build vec of boards -------------------------------------

fn build_vec_of_boards(all_boards: Vec<u8>, ret_vec: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut return_vec: Vec<Vec<u8>> = ret_vec;
    let split = all_boards.split_at(25);
    return_vec.push(split.0.to_vec());
    let remaining_boards = split.1.to_vec();
    if remaining_boards.len() > 25 {
        build_vec_of_boards(remaining_boards, return_vec)
    } else {
        return_vec
    }
}

//--- take all boards (flat) and initialise with (n, false) ---------------

pub fn all_boards_tuple(boards: Vec<u8>) -> Vec<(u8, bool)> {
    let mut checked_tuple_vec: Vec<(u8, bool)> = Vec::new();
    boards
        .iter()
        .for_each(|n| checked_tuple_vec.push((*n, false)));
    checked_tuple_vec
}

//------ for each backlog_nums------------------------------------

pub fn check_backlog_against_boards(tuple_boards: &Vec<(u8, bool)>, backlog_num: u8) -> (u8, bool) {
    //let mut checked: Vec<(u8, bool)> = Vec::new();
    let mut tup: (u8, bool) = (0, false);
    tuple_boards.iter().for_each(|&t| {
        if backlog_num == t.0 {
            tup = (t.0, true);
        } else {
            tup = t;
        }
    });
    tup
}
