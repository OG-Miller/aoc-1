use std::fs::read_to_string;

// advent of code day 1
// return the number of how many times the depth increases (how many times the number goes up)

fn main() {
    let data_string = read_to_string("./day1-data.txt").unwrap();
    let data_string_split = data_string.split("\n").collect();
    let number_vec: Vec<u32> = string_vec_to_num_vec(data_string_split);

    fn string_vec_to_num_vec(v: Vec<&str>) -> Vec<u32> {
        let mut num_vec: Vec<u32> = Vec::new();
        v.iter().for_each(|s| {
            if s.len() != 0 {
                num_vec.push(s.parse::<u32>().unwrap())
            }
        });
        num_vec
    }

    let mut count: u32 = 0;
    let mut stored: u32 = 0;

    for (i, val) in number_vec.iter().enumerate() {
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

    ///////////////////////////////////////////////////////////////////
    // advent of code day 1 Part B
    // return the number of times the depth (in groups of 3) increases
    ///////////////////////////////////////////////////////////////////

    let mut count2: u32 = 0;
    let mut stored_total: u32 = 0;

    for (i, val) in number_vec.iter().enumerate() {
        if i == 0 {
            stored_total = val + number_vec[i + 1] + number_vec[i + 2];
        } else if i < (number_vec.len() - 2) {
            let current_group_total = val + number_vec[i + 1] + number_vec[i + 2];
            if current_group_total > stored_total {
                count2 += 1;
            }
            stored_total = current_group_total;
        }
    }

    println!("count2: {count2}");
}
