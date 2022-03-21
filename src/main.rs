use std::fs::read_to_string;

fn main() {
    // AOC Day3A
    let data_string = read_to_string("./data.txt").unwrap();
    let commons_result = one(data_string);
    let gamma: String = commons_result
        .into_iter()
        .map(|x| if x > 500 { "0" } else { "1" })
        .collect();

    let epsilon: String = gamma
        .chars()
        .map(|c| if c == '1' { "0" } else { "1" })
        .collect();

    let decimal_gamma = isize::from_str_radix(&gamma, 2);
    let decimal_epsilon = isize::from_str_radix(&epsilon, 2);
    let solution = decimal_epsilon.unwrap() * decimal_gamma.unwrap();

    println!("solution: {:?}", solution);
}

pub fn one(s: String) -> Vec<u64> {
    let mut commons_vec: Vec<u64> = vec![0; 12];
    let mut index: usize = 0;
    let mut num_of_zeros: u64 = 0;

    while index < 12 {
        s.lines().for_each(|l| {
            if l.chars().nth(index).unwrap_or('x') == '0' {
                num_of_zeros += 1;
            }
            commons_vec[index] = num_of_zeros;
        });

        num_of_zeros = 0;
        index += 1;
    }

    commons_vec
}
