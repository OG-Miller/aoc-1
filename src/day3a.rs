use std::fs::read_to_string;

fn main() {
    // AOC Day3A
    let data_string = read_to_string("./data.txt").unwrap();
    let line_count_div_2: u64 = &data_string.lines().count().try_into().unwrap() / 2 as u64;
    let commons_result = zeros_per_column(data_string);
    let gamma: String = commons_result
        .into_iter()
        .map(|x| if x > line_count_div_2 { "0" } else { "1" })
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

pub fn zeros_per_column(s: String) -> Vec<u64> {
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
