use std::fs::read_to_string;

//------------------------------------------------------------------------------------------
// AOC Day3B
//
// 1. To find oxygen generator rating, determine the most common value (0 or 1) in the current
//    bit position, and keep only numbers with that bit in that position. If 0 and 1 are
//    equally common, keep values with a 1 in the position being considered.
//    ! after each index the next filter happens on the numbers kept from the previous index
//
// 2. To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit
//    position, and keep only numbers with that bit in that position. If 0 and 1 are equally
//    common, keep values with a 0 in the position being considered.
//
// 3. Multiply these(as decimals) together to get the final result
//------------------------------------------------------------------------------------------

fn main() {
    let data_string = read_to_string("./data.txt").unwrap();

    let line_count_div_2: u64 = &data_string.lines().count().try_into().unwrap() / 2 as u64;

    // total_zeros_vec is no of zeros in each column
    let total_zeros_vec = zeros_per_column(&data_string);
    println!("total_zeros_vec{:?}", total_zeros_vec);
    let gamma: String = total_zeros_vec
        .into_iter()
        // if equally common keep the 1
        .map(|x| if x > line_count_div_2 { "0" } else { "1" })
        .collect();

    println!("gamma {:?}", gamma);
    //let oxygen = total_zeros_vec.inter().map(|x| x)

    let epsilon: String = gamma
        .chars()
        // if equally common keep the 0
        .map(|c| if c == '1' { "0" } else { "1" })
        .collect();

    println!("epsilon{:?}", epsilon);

    let decimal_gamma = isize::from_str_radix(&gamma, 2);
    let decimal_epsilon = isize::from_str_radix(&epsilon, 2);
    // let solution = decimal_epsilon.unwrap() * decimal_gamma.unwrap();

    //println!("solution: {:?}", solution);

    let ox = func(
        data_string.lines().collect::<Vec<&str>>(),
        gamma,
        true,
        &mut 0,
    );
    println!("ox: {:?}", ox);

    let co2 = func(
        data_string.lines().collect::<Vec<&str>>(),
        epsilon,
        false,
        &mut 0,
    );
    println!("co2: {:?}", co2);

    let decimal_ox = isize::from_str_radix(&ox[0], 2).unwrap_or(7);
    println!("ox[0]: {}, decimal_ox: {:?}", ox[0], decimal_ox);
    let decimal_co2 = isize::from_str_radix(&co2[0], 2).unwrap_or(8);
    println!("co2[0]: {}, decimal_co2: {:?}", co2[0], decimal_co2);

    println!("{:?}", decimal_ox * decimal_co2);
}

// ------------
// you have to re-calculate the most common column after every filter loop
pub fn func<'a>(
    v: Vec<&'a str>,
    gamma_or_epsilon: String,
    gamma_bool: bool,
    i: &mut u8,
) -> Vec<&'a str> {
    let filtered = v
        .into_iter()
        .filter(|s| {
            let a = s.chars().nth(*i as usize).unwrap_or('x');
            let b = gamma_or_epsilon.chars().nth(*i as usize).unwrap();
            //println!("if a:{} = b:{}, keep line : {}", a, b, s);
            //       println!("index:{i}");
            a == b
        })
        .collect::<Vec<&str>>();
    // println!("{:?}", filtered);
    *i += 1;
    //println!("index{:?} , filtered len:{}", i, filtered.len());
    if i < &mut 12 && filtered.len() > 1 {
        let local_line_count_div_2 = filtered.len() / 2 as usize;
        let new_zeros = inner_zeros_per_column(&filtered);

        let new_gamma: String = new_zeros
            .into_iter()
            // if equally common keep the 1
            .map(|x| {
                if x as usize > local_line_count_div_2 {
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

        // now do this for epsilon

        func(filtered, new_gamma, gamma_bool, i)
    } else {
        filtered
    }
}

//-----
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

// ----------------------------------------------------------------------------------

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
