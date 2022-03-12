use std::fs::read_to_string;

// advent of code day 1
// return the number of how many times the depth increases (how many times the number goes up)

fn main() {
    let data_raw = read_to_string("./data.txt").unwrap();
    let data_string_vec: Vec<&str> = data_raw.split("\n").collect(); // this leaves an empty string at the last index

    println!("data_string_vec: {:?}", data_string_vec);

    let mut count: u32 = 0;
    let mut stored: u32 = 0;

    // hold new vec of u32s from strings
    let mut data_num_vec: Vec<u32> = Vec::new();

    // convert strings into nums
    for val in data_string_vec.iter() {
        if val.len() != 0 {
            let parsed = val.parse::<u32>().unwrap();
            data_num_vec.push(parsed);
        }
    }

    for (i, val) in data_num_vec.iter().enumerate() {
        // val at first index just gets stored
        if i == 0 {
            stored = val.clone();
        }
        if val > &stored {
            count += 1;
        }
        stored = val.clone();
    }

    println!("count: {count}");
}
