use std::fs::read_to_string;

// advent of code day 2
// calculate the final horizontal & depth positions, return them multiplied together

fn main() {
    // [0] = depth [1] = horizontal
    let mut tuple_store: (u32, u32) = (0, 0);

    #[derive(Debug)]
    struct Command {
        direction: String,
        val: u32,
    }

    let data_string = read_to_string("./day2a-data.txt").unwrap();
    let data_string_split: Vec<&str> = data_string.split("\n").collect();

    let command_vec_result: Vec<Command> = make_command_vec(data_string_split);

    fn make_command_vec(string_vec: Vec<&str>) -> Vec<Command> {
        let mut inner_command_vec: Vec<Command> = vec![];

        string_vec.iter().for_each(|s| {
            let dir_and_val_string_vec: Vec<&str> = s.split(" ").collect();

            // build current_command
            let current_command = Command {
                direction: String::from(dir_and_val_string_vec[0]),
                val: if s.len() > 0 {
                    dir_and_val_string_vec[1].parse::<u32>().unwrap()
                } else {
                    0
                },
            };
            inner_command_vec.push(current_command);
        });
        inner_command_vec
    }

    command_vec_result.iter().for_each(|com| {
        // [0] = depth [1] = horizontal
        if com.direction == "forward" {
            tuple_store.1 += com.val;
        } else if com.direction == "down" {
            tuple_store.0 += com.val;
        } else if com.direction == "up" {
            tuple_store.0 -= com.val;
        } else {
            println!(" in else block, case not covered");
        }
    });

    println!(
        "tuple_store multiplied: {:?}",
        tuple_store.0 * tuple_store.1
    );
}
