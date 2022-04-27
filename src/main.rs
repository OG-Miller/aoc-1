use std::fs::read_to_string; // AOC Day4A

fn main() {
    //-------  Build backlog of numbers to call ---

    //let data_string = read_to_string("./test-bingo-numbers.txt").unwrap();
    let data_string = read_to_string("./4a-numbers.txt").unwrap();

    let backlog_nums = data_string
        .split(",")
        .filter(|x| *x != "\n")
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    //------ Build one vec of boards nums ----------------
    //let boards_string = read_to_string("./test-bingo-boards.txt").unwrap();
    let boards_string = read_to_string("./4a-bingo-boards.txt").unwrap();
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
    //println!("all_board_bool: {:?}", all_board_bool);

    //--------------------------------------------------

    let mut list_of_winners: Vec<Option<WinnerCandidate>> = Vec::new();

    build_vec_of_boards(&all_board_bool, Vec::new())
        .iter()
        .for_each(|board| {
            let result = check_board_for_winner(board.to_vec(), &backlog_nums);
            list_of_winners.push(result);
        });
    println!("list_of_winners: {:?}", list_of_winners);

    let list_of_winners_winning_backlog_indexes = list_of_winners
        .iter()
        .map(|winner| winner.as_ref().unwrap().winning_backlog_index)
        .collect::<Vec<u8>>();
    println!(
        "list_of_winners_winning_backlog_indexes: {:?}",
        list_of_winners_winning_backlog_indexes
    );

    let winning_board_location = get_winning_board(list_of_winners_winning_backlog_indexes);
    println!("winning_board_location: {:?}", winning_board_location);

    let winning_board_index_for_calculation = winning_board_location.1;
    let winning_board = &list_of_winners[winning_board_index_for_calculation];
    println!("winning_board: {:?}", winning_board);
    // correct up to this point
    let answer = calculate_unmarked_total(
        winning_board.as_ref().unwrap().board.clone(),
        winning_board.as_ref().unwrap().winning_number.clone(),
    );
    println!("answer: {:?}", answer);
}

// fn tested & working ✓
pub fn get_winning_board(vec_of_backlog_ids: Vec<u8>) -> (u8, usize) {
    let mut accumulated_lowest_count: u8 = 50;
    let mut index_of_current_lowest_winner: usize = 50;
    vec_of_backlog_ids.iter().enumerate().for_each(|(i, &w)| {
        if w < accumulated_lowest_count {
            accumulated_lowest_count = w;
            index_of_current_lowest_winner = i;
        }
    });
    (
        accumulated_lowest_count.clone(),
        index_of_current_lowest_winner,
    )
}

pub fn calculate_unmarked_total(winner_board_vec: Vec<(u8, bool)>, winning_board_num: u8) -> u32 {
    println!(
        "winner_board_vec: {:?}, winning_board_num: {:?}",
        winner_board_vec, winning_board_num
    );
    let mut total: u32 = 0;
    winner_board_vec.iter().for_each(|n| {
        if n.1 == false {
            total += n.0 as u32;
        }
    });
    //total = n.0 as u32;
    let result: u32 = total * winning_board_num.clone() as u32;
    result
}

#[derive(Debug)]
pub struct WinnerCandidate {
    board: Vec<(u8, bool)>,    // to use in final calculation
    winning_number: u8,        // to use in final calculation
    winning_backlog_index: u8, // what position winning number is in called list   so we know what order boards win in.
}

pub fn check_board_for_winner(
    one_board: Vec<(u8, bool)>,
    backlog: &Vec<u8>,
) -> Option<WinnerCandidate> {
    let mut candidate_board: Vec<(u8, bool)> = vec![(0, false); 25];
    let mut candidate_board_wrapper: Vec<Vec<(u8, bool)>> = Vec::new();
    //let mut current_backlog_num: u8 = 0;
    let mut winner_backlog_i: Vec<usize> = Vec::new();
    let mut winner_backlog_n: u8 = 0;
    let mut found_a_winner: bool = false;
    let mut row_win: bool = false;
    let mut winning_number = 0;
    let mut winning_index: u8 = 0;
    let mut winning_board_index: Vec<usize> = vec![0]; // the board_index at FREEZE, to calc with
                                                       // let mut current_board_index_vec: Vec<usize> = Vec::new();
    let mut winning_backlog_num_vec: Vec<u8> = Vec::new();
    println!("one_board inside check fn : {:?}", one_board);
    backlog
        .iter()
        .enumerate()
        .for_each(|(backlog_i, backlog_n)| {
            one_board
                .iter()
                .enumerate()
                .for_each(|(board_i, board_value)| {
                    // EVERYTHING IN HERE IS FOR ONE BOARD VALUE----
                    //current_board_index_vec.push(board_i);
                    println!("board_value:{:?} , backlog_n:{:?}", board_value, backlog_n);
                    if &board_value.0 == backlog_n && candidate_board[board_i].1 == false {
                        //&& found_a_winner {
                        println!("match!");
                        candidate_board[board_i] = (board_value.0, true);
                        println!("candidate_board: {:?}", candidate_board);
                        if check_current_iter_for_winner(&candidate_board).0 == true {
                            // stop everything and record the state of the board here. this is the winning state
                            println!("candidate_board FREEZE: {:?}", candidate_board);
                            found_a_winner = true;
                            if found_a_winner == true {
                                winning_board_index[0] = board_i;
                            }
                            if found_a_winner == true && winner_backlog_i.len() == 0 {
                                winner_backlog_i.push(backlog_i);
                            }
                            println!("pre-push backlog vec: {:?}", winning_backlog_num_vec);
                            println!("should push {:?} to backlog", &backlog_n);
                            if candidate_board_wrapper.len() == 0 {
                                candidate_board_wrapper.push(candidate_board.to_vec());
                                println!("post-push backlog vec {:?}", winning_backlog_num_vec);
                            };
                            if winning_backlog_num_vec.len() == 0 {
                                winning_backlog_num_vec.push(*backlog_n);
                                println!("post-push backlog vec {:?}", winning_backlog_num_vec);
                            }
                        };
                    } else if &board_value.0 != backlog_n && candidate_board[board_i].1 == false {
                        candidate_board[board_i] = (board_value.0, false);
                    } //------------------------------------- end of winning case block
                    winning_index = *backlog_n;
                });
        });
    if found_a_winner == true {
        //if true == true {
        println!(" found_a_winner is true BLOCK");
        println!(
            "candidate_board_wrapper[0]: {:?}",
            candidate_board_wrapper[0]
        );
        let w = WinnerCandidate {
            board: candidate_board_wrapper[0].to_vec(),
            winning_number: winning_backlog_num_vec[0], // should be 24
            //winning_backlog_index: backlog[winner_backlog_i + 4], // should be 11
            winning_backlog_index: winner_backlog_i[0] as u8,
        };
        return Some(w);
    } else {
        return None;
    }
}

pub fn check_current_iter_for_winner(board: &Vec<(u8, bool)>) -> (bool, bool) {
    let mut is_true: u8 = 0;
    let mut row_win = false;

    //----------------- logic to check column
    board.iter().enumerate().for_each(|(i, num)| {
        println!("{i}");
        if i > 4 {
            println!("i > 4, should return false ");
            return;
        }
        println!("trying to check column... ");
        println!(" num.0 {:?}: ", num.0);
        println!(
            " board[i + 5].0 {:?}: , board[i + 5].1 {:?}: ",
            board[i + 5].0,
            board[i + 5].1
        );
        println!(
            " board[i + 10].0 {:?}:, board[i + 10].1 {:?}:",
            board[i + 10].0,
            board[i + 10].1
        );
        println!(
            " board[i + 15].0 {:?}:, board[i + 15].1 {:?}:",
            board[i + 15].0,
            board[i + 15].1
        );
        println!(
            " board[i + 20].0 {:?}:, board[i + 20].1 {:?}:",
            board[i + 20].0,
            board[i + 20].1
        );
        if (i < 4 && is_true == 0)
            && (num.1 == true)
            && (board[i + 5].1 == true)
            && (board[i + 10].1 == true)
            && (board[i + 15].1 == true)
            && (board[i + 20].1 == true)
        {
            is_true += 1;
            println!("column win");
            //        column_win_count += 1;
        }
    });

    //----------------- logic to check rows
    let row_vec = vec![0, 5, 10, 15, 20];
    board.iter().enumerate().for_each(|(i, num)| {
        println!("trying to check row... ");
        if i > 20 {
            return;
        }

        println!(" num.0 {:?}: ", num.0);
        println!(
            " board[i + 1].0 {:?}, board[i + 1].1 {:?}",
            board[i + 1].0,
            board[i + 1].1
        );
        println!(
            " board[i + 2].0 {:?}, board[i + 2].1 {:?}",
            board[i + 2].0,
            board[i + 2].1
        );
        println!(
            " board[i + 3].0 {:?}, board[i + 3].1 {:?}",
            board[i + 3].0,
            board[i + 3].1
        );
        println!(
            " board[i + 4].0 {:?}, board[i + 4].1 {:?}",
            board[i + 4].0,
            board[i + 4].1
        );
        if row_vec.contains(&i)
            && (num.1 == true)
            && (board[i + 1].1 == true)
            && (board[i + 2].1 == true)
            && (board[i + 3].1 == true)
            && (board[i + 4].1 == true)
        {
            //       row_win_count += 1;
            println!("row vec contains i: {i}");
            is_true += 1;
            row_win = true;
            println!("row win");
        }
    });
    // return (win?: bool, row win: bool)
    println!(" is true :{:?}", is_true);
    if is_true > 0 {
        let x = (true, row_win);
        println!(" result of check is --> :{:?}", x);
        x
    } else {
        let y = (false, row_win);
        println!(" result of check is --> :{:?}", y);
        y
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
