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
    let mut list_of_winners = Vec::new();

    build_vec_of_boards(&all_board_bool, Vec::new())
        .iter()
        .for_each(|board| {
            list_of_winners.push(check_board_for_winner(board.to_vec(), &backlog_nums))
        });

    println!("list_of_winners: {:?}", list_of_winners);
}
#[derive(Debug)]
pub struct WinnerCandidate {
    board: Vec<(u8, bool)>,    // to use in final calculation
    winning_number: u8,        // to use in final calculation
    winning_backlog_index: u8, // so we know what order boards win in.
}

pub fn check_board_for_winner(
    one_board: Vec<(u8, bool)>,
    backlog: &Vec<u8>,
) -> Option<WinnerCandidate> {
    let mut candidate_board: Vec<(u8, bool)> = vec![(0, false); 25];
    //let mut current_backlog_num: u8 = 0;
    let mut winner_backlog_i: usize = 0;
    let mut winner_backlog_n: u8 = 0;
    let mut found_a_winner: bool = false;
    let mut row_win: bool = false;
    let mut winning_number = 0;
    let mut winning_index: u8 = 0;
    let mut winning_board_index: usize = 0;
    let mut winning_board_index_vec: Vec<u8> = Vec::new();
    let mut winning_backlog_num_vec: Vec<u8> = Vec::new();

    backlog
        .iter()
        .enumerate()
        .for_each(|(backlog_i, backlog_n)| {
            one_board
                .iter()
                .enumerate()
                .for_each(|(board_i, board_value)| {
                    // EVERYTHING IN HERE IS FOR ONE BOARD VALUE----
                    if &board_value.0 == backlog_n {
                        // WE HAVE A MATCH..NOW MARK IT ON THE BOARD
                        candidate_board[board_i] = (board_value.0, true);
                        let check = check_current_iter_for_winner(&candidate_board);
                        // check.0 true is a win, check.1 true is a row win
                        if check.0 == true {
                            // start of winning block ----------------

                            row_win = check.1;

                            winning_number = if row_win == true {
                                winner_backlog_n
                            } else if row_win == false {
                                winner_backlog_n
                            } else {
                                100 // debugging
                            };

                            winning_board_index_vec.push(backlog_i as u8);
                            winning_backlog_num_vec.push(*backlog_n);
                            // if found_a_winner == false {
                            found_a_winner = true;
                            winner_backlog_i = backlog_i;
                            println!("first winner_backlog_i: {backlog_i}");
                            winner_backlog_n = *backlog_n;
                            println!("first winner_backlog_n: {backlog_n}");

                            //------------------------------------------------------------- end of winning case block
                        }

                        //winning_number = winner

                        winning_index = if row_win { *backlog_n } else { *backlog_n };
                    }
                    // -----------------^ FOR EACH BOARD NUMBER ----
                });
        });
    if winning_backlog_num_vec.len() > 0 {
        let w = WinnerCandidate {
            board: candidate_board,
            winning_number: winning_backlog_num_vec[0],
            winning_backlog_index: winning_board_index_vec[0],
        };
        return Some(w);
    } else {
        return None;
    }
}

pub fn check_current_iter_for_winner(board: &Vec<(u8, bool)>) -> (bool, bool) {
    let mut is_true: u8 = 0;
    let mut row_win = false;
    // let mut column_win_count = 0;
    // let mut row_win_count = 0;

    println!("board.len() {:?}", board.len());
    //----------------- logic to check column
    board.iter().enumerate().for_each(|(i, num)| {
        println!("{i}");
        if (i < 5 && is_true == 0)
            && (num.1 == true)
            && (board[i + 5].1 == true)
            && (board[i + 9].1 == true)
            && (board[i + 14].1 == true)
            && (board[i + 19].1 == true)
        {
            //   println!("i is ---> {i}");
            println!(" num.0 {:?}: ", num.0);
            println!(" board[i + 5].0 {:?}: ", board[i + 5].0);
            println!(" board[i + 10].0 {:?}: ", board[i + 10].0);
            println!(" board[i + 15].0 {:?}: ", board[i + 15].0);
            println!(" board[i + 20].0 {:?}: ", board[i + 20].0);
            is_true += 1;
            //        column_win_count += 1;
        }
    });

    //----------------- logic to check rows
    let row_vec = vec![0, 5, 10, 15, 20];
    board.iter().enumerate().for_each(|(i, num)| {
        //println!("trying to check row... ");
        if row_vec.contains(&i)
            && (num.1 == true)
            && (board[i + 1].1 == true)
            && (board[i + 2].1 == true)
            && (board[i + 3].1 == true)
            && (board[i + 4].1 == true)
        {
            //       row_win_count += 1;
            is_true += 1;
            row_win = true;
        }
    });
    // return (win?: bool, row win: bool)
    if is_true > 0 {
        (true, row_win)
    } else {
        (false, row_win)
    }
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
    if remaining_boards.len() >= 25 {
        build_vec_of_boards(&remaining_boards, return_vec)
    } else {
        //println!("return_vec !ABC::: {:?}", return_vec);

        return_vec
    }
}

//1 97 1 1 1
//1 7 1 1 1
//1 75 1 1 1
//1 66 1 1 1
//1 8 1 1 67

//97 7 75 66 8
//1 1 1 1 1
//1 1 1 1 1
//1 1 1 1 1
//1 1 1 1 67
//
//97 7 75 66 8
//1 1 1 1 1
//1 1 1 1 1
//1 1 1 1 1
//1 1 1 1 67
//
//2 1 1 1 5
//4 5 5 5 4
//4 5 5 5 4
//4 5 5 5 4
//5 1 1 1 67
