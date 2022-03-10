use std::fs::read_to_string;

fn main() {
    // advent of code day 1
    // return the number of how many times the depth increases (how many times the number goes up)

    // array with one index which is a giant string
    let mut new_vec = Vec::new();

    let data_raw = read_to_string("./data.txt");
    match &data_raw {
        Ok(v) => new_vec.push(v), // println!("data is : {}", v),
        Err(e) => println!("Some Error: {}", e),
    };

    //    fn remove_slash_n(string: String) -> String {
    //        string.replace("\n", "")
    //    }
    //
    //    let no_whitespace = remove_slash_n(new_vec[0].to_owned());
    let split_it: Vec<&str> = new_vec[0].split("\n").collect();
    println!("split it: {:?} ", split_it);
    let array_iter = split_it.iter();

    //  let mut parsed_vec: Vec<u32> = Vec::new();
    let mut count: u32 = 0;
    let mut stored: u32 = 0;
    for (i, val) in array_iter.enumerate() {
        //println!("i:{:?},val:{:?}", i, val);
        //  if val.parse::<u32>().unwrap() < 1 {
        //      break;
        //  }
        let parsed_val = val.parse::<u32>().unwrap();
        //  let matched_parsed_val: Option<u32> = match parsed_val {
        //      Some(x) => x,
        //  };

        // save i's value
        // if next i's value is bigger than saved i's, then add +1 to the count

        if i == 0 {
            //            println!("1");
            stored = parsed_val;
        //           println!("1 B");
        } else if parsed_val > stored {
            //          println!("2");
            count += 1;
            stored = parsed_val;
            //         println!("2B");
        }

        //    println!("out of  inner loop");
    }

    println!("{count}");
    // for (i, el) in array_iter.enumerate() {
    //     if el.parse::<u32>().unwrap() > split_it[i - 1].parse::<u32>().unwrap() {
    //         println!("{}", el);
    //     }
    // }
}
