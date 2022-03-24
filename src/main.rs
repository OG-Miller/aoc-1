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

    let ox = func(data_string.lines().collect::<Vec<&str>>(), gamma, &mut 0);
    println!("ox: {:?}", ox);

    let co2 = func(data_string.lines().collect::<Vec<&str>>(), epsilon, &mut 0);
    println!("co2: {:?}", co2);

    let decimal_ox = isize::from_str_radix(&ox[0], 2).unwrap();
    println!("ox[0]: {} , decimal_ox: {:?}", ox[0], decimal_ox);
    let decimal_co2 = isize::from_str_radix(&co2[0], 2).unwrap();
    println!("decimal_co2: {:?}", decimal_co2);

    println!("{:?}", decimal_ox * decimal_co2);
}

// ------------

pub fn func<'a>(v: Vec<&'a str>, gamma: String, i: &mut u8) -> Vec<&'a str> {
    let filtered = v
        .into_iter()
        .filter(|s| {
            let a = s.chars().nth(*i as usize).unwrap_or('x');
            let b = gamma.chars().nth(*i as usize).unwrap();
            //println!("if a:{} = b:{}, keep line : {}", a, b, s);
            //       println!("index:{i}");
            a == b
        })
        .collect::<Vec<&str>>();
    println!("{:?}", filtered);
    *i += 1;
    println!("index{:?}", i);
    if i < &mut 12 && filtered.len() > 0 {
        func(filtered, gamma, i)
    } else {
        filtered
    }
}

// ------------

pub fn zeros_per_column(s: &String) -> Vec<u64> {
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
