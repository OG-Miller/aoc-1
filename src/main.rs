use std::fs::read_to_string;

// advent of code day 2
// calculate the final horizontal & depth positions, return them multiplied together

fn main() {
    struct Position {
        depth: u8,
        horizontal: u8,
    }
    // overall position to be updated
    let mut store_position = Position {
        depth: 0,
        horizontal: 0,
    };
    #[derive(Debug)]
    struct Command {
        direction: String,
        val: u8,
    }

    let data_string = read_to_string("./data.txt").unwrap();
    let data_string_split: Vec<&str> = data_string.split("\n").collect();
    //println!("{:?}", data_string_split); // eg  ["forward 8", "down 6", "down 6"]

    let command_vec_result: Vec<Command> = string_vec_to_command_vec(data_string_split);
    println!("command_vec_result: {:?}", command_vec_result);

    fn string_vec_to_command_vec(string_vec: Vec<&str>) -> Vec<Command> {
        let mut inner_command_vec: Vec<Command> = vec![];

        string_vec.iter().for_each(|s| {
            let dir_and_val_string_vec: Vec<&str> = s.split(" ").collect();
            // build current_command
            let current_command = Command {
                direction: String::from(dir_and_val_string_vec[0]),
                val: if s.len() > 0 {
                    dir_and_val_string_vec[1].parse::<u8>().unwrap()
                } else {
                    0
                },
            };
            inner_command_vec.push(current_command);
        });
        // return vec of commands
        inner_command_vec
    }
}
