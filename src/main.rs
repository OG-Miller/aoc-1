use std::fs::read_to_string; // AOC Day4A

fn main() {
    //-------------  Build backlog of numbers to call -------------------------------

    let data_string = read_to_string("./test-bingo-numbers.txt").unwrap();

    let backlog_nums = data_string
        .split(",")
        .filter(|x| *x != "\n")
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    //----------------- Build one vec of boards nums ---------------------------------
    let boards_string = read_to_string("./test-bingo-boards.txt").unwrap();
    let alternative = boards_string.replace("\n", ",");
    let replace_spaces = alternative.replace(",", " ");
    let split_stuff = replace_spaces.split(" ").collect::<Vec<&str>>();
    let remove_empties = split_stuff
        .into_iter()
        .filter(|x| x.len() > 0)
        .collect::<Vec<&str>>();
    let all_board_bool: Vec<(u8, bool)> = remove_empties
        .iter()
        .map(|x| (x.parse::<u8>().unwrap(), false))
        .collect();

    //--------------------------------------------------
    //(vec_of_invidual_boards: Vec<Vec<(u8, bool)>>, backlog_nums: Vec<u8>) {
    //let ret_vec: Vec<Vec<(u8, bool)>> = Vec::new();
    //let vec_of_boards = build_vec_of_boards(&all_board_bool, Vec::new());
    //    vec_of_boards
    //        .iter()
    //        .for_each(|board| check_for_winner(board.to_vec(), backlog_nums));
    let mut v = Vec::new();
    // let x = vec_of_boards
    //     .iter()
    //     .for_each(|board| v.push(check_for_winner(board.to_vec(), backlog_nums)));
    let winners = build_vec_of_boards(&all_board_bool, Vec::new())
        .iter()
        .for_each(|board| v.push(check_for_winner(board.to_vec(), &backlog_nums)));

    println!("vvvvvvvvvvvvvvvvvvvvvv: {:?}", v);
}
#[derive(Debug)]
pub struct WinnerCandidate {
    board: Vec<(u8, bool)>, // to use in final calculation
    winning_number: u8,     // to use in final calculation
    called_num_count: u8,   // so we know what order boards win in.
}

pub fn check_for_winner(one_board: Vec<(u8, bool)>, backlog: &Vec<u8>) -> Option<WinnerCandidate> {
    // board for struct
    let mut candidate_board: Vec<(u8, bool)> = Vec::new();
    // winning_number for struct
    let mut current_backlog_num: u8 = 0; // this is returning last number in list (2) but it should actually be 4
    let mut called_num_count: u8 = 0; // this is currently returning 25, this should be how many to get to a win...
    let mut is_a_winner: bool = false;
    //let mut winning_board_index: usize = 26;

    //  let w = WinnerCandidate {
    //      board: candidate_board,
    //      winning_number: current_backlog_num,
    //      called_num_count,
    //  };
    //candidate_board = one_board;
    backlog
        .iter()
        .enumerate()
        .for_each(|(_backlog_i, backlog_n)| {
            called_num_count += 1;
            one_board
                .iter()
                .enumerate()
                .for_each(|(board_i, board_value)| {
                    println!("board_value:{:?}, backlog n: {:?}", board_value, backlog_n);
                    current_backlog_num = *backlog_n; //.clone();
                    println!("current_backlog_num: {:?}", current_backlog_num);
                    // populate canditate board
                    if candidate_board.len() < 25 {
                        if backlog_n == &board_value.0 {
                            //println!(" MATCH!");
                            //println!("one_board: {:?}", &one_board);
                            candidate_board.push((board_value.0, true));
                            // check if board is winner----------------------------------<<<<<<
                            check_current_iter_for_winner(&candidate_board);
                        } else {
                            // no match
                            candidate_board.push((board_value.0, false));
                        }
                    } else {
                        //println!("candidate_board.len() is >= 25");
                        if backlog_n == &board_value.0 {
                            candidate_board[board_i] = (board_value.0, true);
                            // check if board is winner----------------------------------<<<<<<
                            check_current_iter_for_winner(&candidate_board);
                        }
                    }
                });
            //-------------- logic to check columns
            //       candidate_board.iter().enumerate().for_each(|(i, num)| {
            //           if (i < 5 )//&& is_a_winner == false)
            //               && (num.1 == true)
            //               && (candidate_board[i + 5].1 == true)
            //               && (candidate_board[i + 10].1 == true)
            //               && (candidate_board[i + 15].1 == true)
            //               && (candidate_board[i + 20].1 == true)
            //           {
            //               //println!("column bingo"); // you can return after the first bingo, currently still checks after first
            //               is_a_winner = true;
            //           }
            //           //else {
            //           //println!("no bingo");
            //           //is_a_winner = false;
            //           //   return;
            //           //}
            //       });

            //       //----------------- logic to check rows
            //       candidate_board.iter().enumerate().for_each(|(i, num)| {
            //           if (i == 0 | 5 | 10 | 15 | 20)
            //               && (num.1 == true)
            //               && (candidate_board[i + 5].1 == true)
            //               && (candidate_board[i + 10].1 == true)
            //               && (candidate_board[i + 15].1 == true)
            //               && (candidate_board[i + 20].1 == true)
            //           {
            //               //println!("row bingo");
            //               //println!("candidate_board: {:?}", candidate_board);
            //               is_a_winner = true;
            //               // println!("is_a_winner:{:?}", &is_a_winner);
            //           } else {
            //               //        println!("no bingo");
            //               //is_a_winner = false;
            //               return;
            //           }
            //       })
        });
    // if board is a winning canditate return it, else...don't?
    //println!("candidate_board:{:?}", candidate_board);
    let w = WinnerCandidate {
        board: candidate_board,
        winning_number: current_backlog_num,
        called_num_count,
    };
    //if is_a_winner == true {
    Some(w)
    //Some(w)
    //} else {
    //    println!("is_a_winner:{:?}", &is_a_winner);
    //None
    //}
}

pub fn check_current_iter_for_winner(cand_board: &Vec<(u8, bool)>) -> bool {
    if cand_board.len() < 25 {
        false
    } else {
        let mut is_a_winner: bool = false;
        //----------------- logic to check column
        cand_board.iter().enumerate().for_each(|(i, num)| {
            if (i < 5)
                && (num.1 == true)
                && (cand_board[i + 5].1 == true)
                && (cand_board[i + 10].1 == true)
                && (cand_board[i + 15].1 == true)
                && (cand_board[i + 20].1 == true)
            {
                //println!("column bingo"); // you can return after the first bingo, currently still checks after first
                is_a_winner = true;
            } //else {
              //println!("no bingo");
              //is_a_winner = false;
              //   return;
              //}
        });

        //----------------- logic to check rows
        cand_board.iter().enumerate().for_each(|(i, num)| {
            if (i == 0 | 5 | 10 | 15 | 20)
                && (num.1 == true)
                && (cand_board[i + 5].1 == true)
                && (cand_board[i + 10].1 == true)
                && (cand_board[i + 15].1 == true)
                && (cand_board[i + 20].1 == true)
            {
                //println!("row bingo");
                //println!("candidate_board: {:?}", candidate_board);
                is_a_winner = true;
                // println!("is_a_winner:{:?}", &is_a_winner);
                //} else {
                //        println!("no bingo");
                //is_a_winner = false;
                //return;
                //   false;
            }
        });
        is_a_winner
    }
}

// here I want to return maybe the index and the value of each match, maybe the whole vec
pub fn all_indexes_of_one_backlog(boards: &Vec<(u8, bool)>, num: u8) -> Vec<(u8, bool)> {
    let mut inner_vec = Vec::new();
    boards.iter().enumerate().for_each(|(_i, &x)| {
        if x.0 == num {
            inner_vec.push((x.0, true))
        } else {
            inner_vec.push(x)
        }
    });
    inner_vec
}

//---------- Build vec of boards -------------------------------------

fn build_vec_of_boards(
    all_boards: &Vec<(u8, bool)>,
    ret_vec: Vec<Vec<(u8, bool)>>,
) -> Vec<Vec<(u8, bool)>> {
    let mut return_vec: Vec<Vec<(u8, bool)>> = ret_vec;
    let split = all_boards.split_at(25);
    return_vec.push(split.0.to_vec());
    let remaining_boards = split.1.to_vec();
    if remaining_boards.len() > 25 {
        build_vec_of_boards(&remaining_boards, return_vec)
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
