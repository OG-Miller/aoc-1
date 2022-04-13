use std::fs::read_to_string; // AOC Day4A

fn main() {
    //-------------  Build backlog of numbers to call -------------------------------

    let data_string = read_to_string("./4a-numbers.txt").unwrap();

    let backlog_nums = data_string
        .split(",")
        .filter(|x| *x != "\n")
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    //println!("backlog_nums: {:?}", backlog_nums);
    //----------------- Build one vec of boards nums ---------------------------------
    let boards_string = read_to_string("./4a-bingo-boards.txt").unwrap();
    let alternative = boards_string.replace("\n", ",");
    let replace_spaces = alternative.replace(",", " ");
    let split_stuff = replace_spaces.split(" ").collect::<Vec<&str>>();
    let remove_empties = split_stuff
        .into_iter()
        .filter(|x| x.len() > 0)
        .collect::<Vec<&str>>();
    let all_board_bool = remove_empties
        .iter()
        .map(|x| (x.parse::<u8>().unwrap(), false))
        .collect::<Vec<(u8, bool)>>();

    let all_boards_vec: Vec<Vec<(u8, bool)>> = build_vec_of_boards(all_board_bool, Vec::new());
    //println!("all_boards_vec: {:?}", all_boards_vec);

    //--- this will have to be recursive and take the returned vec from each backlog_num search and
    //use that in the next backlog_num search

    // let find_all_backlog_nums = backlog_nums
    //     .iter()
    //     .for_each(|&n| find_backlog_in_all_boards(&all_boards_vec, n));
    // //.collect();
    // println!("fin: {:?}", find_all_backlog_nums);
}

fn tester2(boards: Vec<Vec<(u8, bool)>>, backlog: Vec<u8>) {
    let mut inner_vec = Vec::new();
    inner_vec.push(backlog.split_at(1));
    backlog
        .iter()
        .for_each(|&n| find_backlog_in_all_boards(&all_boards_vec, n))
}

//search all boards for one called number(to test for now)then iterate all backlog with this fn
// This has to be recursive because each checked number bool has to be updated and passed into the
// same fn again....
//
pub fn find_backlog_in_all_boards(
    boards: &Vec<Vec<(u8, bool)>>,
    called: u8,
) -> Vec<Vec<(u8, bool)>> {
    let mut all_searched_boards: Vec<Vec<(u8, bool)>> = Vec::new();
    boards
        .iter()
        .for_each(|board| all_searched_boards.push(search_single_board(board, called)));
    all_searched_boards
}
//let tester = search_single_board(&all_boards_vec[0], backlog_nums[0]);
//println!("tester: {:?}", tester);
//-- For each backlog_num, find matches and update bool

pub fn search_single_board(board: &Vec<(u8, bool)>, called: u8) -> Vec<(u8, bool)> {
    let mut searched_board = Vec::new();
    board.iter().for_each(|&t| {
        if t.0 == called {
            searched_board.push((t.0, true))
        } else {
            searched_board.push(t)
        }
    });
    searched_board
}
//---------- Build vec of board number tuples with 'picked' bool (u8,bool) -----------

//let tuple_vec = all_boards_tuple(all_board_nums);
//    let check_all_boards = backlog_nums
//        .iter()
//        .map(|&n| check_backlog_against_boards(&tuple_vec, n))
//        .collect::<Vec<(u8, bool)>>();
//    println!("check_all_boards: {:?}", check_all_boards);

//---------- Build vec of boards -------------------------------------

fn build_vec_of_boards(
    all_boards: Vec<(u8, bool)>,
    ret_vec: Vec<Vec<(u8, bool)>>,
) -> Vec<Vec<(u8, bool)>> {
    let mut return_vec: Vec<Vec<(u8, bool)>> = ret_vec;
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

//pub fn check_backlog_against_boards(tuple_boards: &Vec<(u8, bool)>, backlog_num: u8) -> (u8, bool) {
//    //let mut checked: Vec<(u8, bool)> = Vec::new();
//    let mut tup: (u8, bool) = (0, false);
//    tuple_boards.iter().for_each(|&t| {
//        if backlog_num == t.0 {
//            tup = (t.0, true);
//        } else {
//            tup = t;
//        }
//    });
//    tup
//}
