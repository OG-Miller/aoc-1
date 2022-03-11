use std::fs::read_to_string;

fn main() {
    // advent of code day 1
    // return the number of how many times the depth increases (how many times the number goes up)

    // array with one index which is a giant string
    let mut slash_n_vec = Vec::new();

    let data_raw = read_to_string("./data.txt");
    match &data_raw {
        Ok(v) => slash_n_vec.push(v), // println!("data is : {}", v),
        Err(e) => println!("Some Error: {}", e),
    };

    // array of numbers
    let string_nums: Vec<&str> = slash_n_vec[0].split("\n").collect();
    let array_iter = string_nums.iter();

    let mut count: u32 = 0;
    let mut stored: u32 = 0;

    // hold new array of u32s from strings
    let mut vec_of_nums: Vec<u32> = Vec::new();

    // convert strings into nums
    for val in array_iter {
        if val.len() != 0 {
            let parsed = val.parse::<u32>().unwrap();
            //  println!("current i: {i}");
            vec_of_nums.push(parsed);
        }
    }

    let iter_nums = vec_of_nums.iter();

    for (i, val) in iter_nums.enumerate() {
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
