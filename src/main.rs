use std::fs::read_to_string;

fn main() {
    let data_string = read_to_string("./data.txt").unwrap();

    // this function should return most common digit for index on each line
    fn one(s: String) -> Vec<u64> {
        let mut commons_vec: Vec<u64> = vec![0; 12];
        let mut index: usize = 0;
        let mut num_of_zeros: u64 = 0;

        while index <= 13 {
            s.lines().for_each(|l| {
                if l.chars().nth(index).unwrap() == '0' {
                    //  println!("char == 0");
                    num_of_zeros += 1;
                } else {
                    println!("char is not 0");
                }
                commons_vec[index] = num_of_zeros;
            });

            num_of_zeros = 0;
            index += 1;
            println!("commons_vec: {:?}", commons_vec);
        }

        commons_vec
    }

    let commons_result = one(data_string);

    // finding gamma:
    //let mut gamma: String = String::new();
    let gamma = commons_result.iter().map(|n| n.to_string());
    println!("{:?}", gamma);
    //commons_result.iter().for_each(|n| gamma.push(n.to_char()));
}
