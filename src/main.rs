use std::fs::read_to_string;

// AOC Day3B

fn main() {
    let data_string = read_to_string("./data.txt").unwrap();
    let half_of_data_len: u64 = &data_string.lines().count().try_into().unwrap() / 2 as u64;
    let total_zeros_vec = zeros_per_column(&data_string);

    //---- calculate gamma & epsilon -----------
    let gamma: String = total_zeros_vec
        .into_iter()
        .map(|x| if x > half_of_data_len { "0" } else { "1" })
        .collect();

    let epsilon: String = gamma
        .chars()
        .map(|c| if c == '1' { "0" } else { "1" })
        .collect();

    //---- calculate oxygen & co2 -----------
    let ox = get_o2_or_co2(
        data_string.lines().collect::<Vec<&str>>(),
        gamma,
        true,
        &mut 0,
    );

    let co2 = get_o2_or_co2(
        data_string.lines().collect::<Vec<&str>>(),
        epsilon,
        false,
        &mut 0,
    );

    let decimal_ox = isize::from_str_radix(&ox[0], 2).unwrap_or(7);
    let decimal_co2 = isize::from_str_radix(&co2[0], 2).unwrap_or(8);

    println!("SOLUTION:{:?}", decimal_ox * decimal_co2);
}

// --------------------------------------------------

pub fn zeros_per_column(s: &String) -> Vec<u64> {
    let mut commons_vec: Vec<u64> = vec![0; 12];
    let mut index: usize = 0;
    let mut num_of_zeros: u64 = 0;

    while index < 12 {
        s.lines().for_each(|l| {
            if l.chars().nth(index).unwrap() == '0' {
                num_of_zeros += 1;
            }
            commons_vec[index] = num_of_zeros;
        });

        num_of_zeros = 0;
        index += 1;
    }

    commons_vec
}

// --------------------------------------------------

pub fn get_o2_or_co2<'a>(
    // data
    v: Vec<&'a str>,
    // gamma or epsilon to compare against
    gamma_or_epsilon: String,
    gamma_bool: bool,
    i: &mut u8,
) -> Vec<&'a str> {
    let filtered = v
        .into_iter()
        .filter(|s| {
            let a = s.chars().nth(*i as usize).unwrap_or('x');
            let b = gamma_or_epsilon.chars().nth(*i as usize).unwrap();
            a == b
        })
        .collect::<Vec<&str>>();
    *i += 1;
    if i < &mut 12 && filtered.len() > 1 {
        let local_half_of_data_len = filtered.len() / 2 as usize;
        let new_zeros = inner_zeros_per_column(&filtered);

        let new_gamma: String = new_zeros
            .into_iter()
            .map(|x| {
                if x as usize > local_half_of_data_len {
                    if gamma_bool == true {
                        "0"
                    } else {
                        "1"
                    }
                } else {
                    if gamma_bool == true {
                        "1"
                    } else {
                        "0"
                    }
                }
            })
            .collect();
        get_o2_or_co2(filtered, new_gamma, gamma_bool, i)
    } else {
        filtered
    }
}

//--------------------------

pub fn inner_zeros_per_column(v: &Vec<&str>) -> Vec<u64> {
    let mut commons_vec: Vec<u64> = vec![0; 12];
    let mut index: usize = 0;
    let mut num_of_zeros: u64 = 0;

    while index < 12 {
        v.iter().for_each(|x| {
            if x.chars().nth(index).unwrap() == '0' {
                num_of_zeros += 1;
            }
            commons_vec[index] = num_of_zeros;
        });

        num_of_zeros = 0;
        index += 1;
    }

    commons_vec
}
