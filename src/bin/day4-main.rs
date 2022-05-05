use std::fs::read_to_string; // AOC Day4A

fn main() {
    //-------  Build backlog of numbers to call ---
    let data_string = read_to_string("./4a-numbers.txt").unwrap();

    let backlog_nums = data_string
        .split(",")
        .filter(|&x| x != "\n")
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    //------ Build single vec of unsplit boards (num, bool) ----------------
    let boards_string = read_to_string("./4a-bingo-boards.txt").unwrap();
    let replace_linebreak = boards_string.replace("\n", ",");
    let remove_commas = replace_linebreak.replace(",", " ");
    let split_stuff = remove_commas.split(" ").collect::<Vec<&str>>();
    let remove_empties = split_stuff
        .into_iter()
        .filter(|x| x.len() > 0)
        .collect::<Vec<&str>>();
    // add boolean value to each board value
    let all_board_bool: Vec<(u8, bool)> = remove_empties
        .iter()
        .map(|x| (x.parse::<u8>().unwrap(), false))
        .collect();

    //--------------------------------------------------

    let mut list_of_winners: Vec<Option<WinnerCandidate>> = Vec::new();

    // create a list of winning boards
    build_vec_of_boards(&all_board_bool, Vec::new())
        .iter()
        .for_each(|board| {
            let result = check_board_for_winner(board.to_vec(), &backlog_nums);
            list_of_winners.push(result);
        });

    let list_of_winners_winning_backlog_indexes = list_of_winners
        .iter()
        .map(|winner| winner.as_ref().unwrap().winning_backlog_index)
        .collect::<Vec<u8>>();
    let winning_board_location = get_winning_board(list_of_winners_winning_backlog_indexes);

    let winning_board_index_for_calculation = winning_board_location.1;
    let winning_board = &list_of_winners[winning_board_index_for_calculation];
    let answer = calculate_unmarked_total(
        winning_board.as_ref().unwrap().board.clone(),
        winning_board.as_ref().unwrap().winning_number.clone(),
    );
    println!("answer: {:?}", answer);
}

pub fn get_winning_board(vec_of_backlog_ids: Vec<u8>) -> (u8, usize) {
    let mut accumulated_lowest_count: u8 = 50;
    let mut index_of_current_lowest_winner: usize = 50;
    vec_of_backlog_ids.iter().enumerate().for_each(|(i, &w)| {
        // swap arrow below to '<' for lowest (4a winner), and '>' for highest (4b winner)
        if w > accumulated_lowest_count {
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
    let mut total: u32 = 0;
    winner_board_vec.iter().for_each(|n| {
        if n.1 == false {
            total += n.0 as u32;
        }
    });
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
    let mut winner_backlog_i: Vec<usize> = Vec::new();
    let mut found_a_winner: bool = false;
    let mut winning_index: u8 = 0;
    let mut winning_board_index: Vec<usize> = vec![0];

    let mut winning_backlog_num_vec: Vec<u8> = Vec::new();
    backlog
        .iter()
        .enumerate()
        .for_each(|(backlog_i, backlog_n)| {
            one_board
                .iter()
                .enumerate()
                .for_each(|(board_i, board_value)| {
                    if &board_value.0 == backlog_n && candidate_board[board_i].1 == false {
                        candidate_board[board_i] = (board_value.0, true);
                        if check_current_iter_for_winner(&candidate_board) == true {
                            found_a_winner = true;
                            if found_a_winner == true {
                                winning_board_index[0] = board_i;
                            }
                            if found_a_winner == true && winner_backlog_i.len() == 0 {
                                winner_backlog_i.push(backlog_i);
                            }
                            if candidate_board_wrapper.len() == 0 {
                                candidate_board_wrapper.push(candidate_board.to_vec());
                            };
                            if winning_backlog_num_vec.len() == 0 {
                                winning_backlog_num_vec.push(*backlog_n);
                            }
                        };
                    } else if &board_value.0 != backlog_n && candidate_board[board_i].1 == false {
                        candidate_board[board_i] = (board_value.0, false);
                    }
                    winning_index = *backlog_n;
                });
        });
    if found_a_winner == true {
        let w = WinnerCandidate {
            board: candidate_board_wrapper[0].to_vec(),
            winning_number: winning_backlog_num_vec[0],
            winning_backlog_index: winner_backlog_i[0] as u8,
        };
        return Some(w);
    } else {
        return None;
    }
}

pub fn check_current_iter_for_winner(board: &Vec<(u8, bool)>) -> bool {
    let mut win_count: u8 = 0;

    //--------- logic to check column
    board.iter().enumerate().for_each(|(i, num)| {
        if i > 4 {
            return;
        }
        if (i < 4 && win_count == 0)
            && (num.1 == true)
            && (board[i + 5].1 == true)
            && (board[i + 10].1 == true)
            && (board[i + 15].1 == true)
            && (board[i + 20].1 == true)
        {
            win_count += 1;
        }
    });

    //--------- logic to check rows
    let row_vec = vec![0, 5, 10, 15, 20];
    board.iter().enumerate().for_each(|(i, num)| {
        if i > 20 {
            return;
        }

        if row_vec.contains(&i)
            && (num.1 == true)
            && (board[i + 1].1 == true)
            && (board[i + 2].1 == true)
            && (board[i + 3].1 == true)
            && (board[i + 4].1 == true)
        {
            win_count += 1;
        }
    });
    if win_count > 0 {
        true
    } else {
        false
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
        return_vec
    }
}
