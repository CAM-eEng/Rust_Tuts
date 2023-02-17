use std::{env, io, io::Write, collections::HashMap, cmp::Ordering};

fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    
    //@Descritption:
    // Given a list of ints, use a vector and return the mean (avg val), 
    // median (when sorted, val in middle position) and mode (val that occurs most often),
    // of the list (use hashmaps).
    //
    //@Approach:
    // Use a hashmap that looks as follow for example [1, 0, 2, 0, 3, 0, 4]:
    // {
    //  "Mean":     10/7,
    //  "Median":   1,
    //  "Mode":     0,
    // }
    'mains: loop {
        let mut _user_vals: Vec<i32> = Vec::new();
        let mut mode_hash = HashMap::new();
        let mut triple_m = HashMap::new();

        'usr_in: loop {
            println!("Please enter a list of integers or q to quit.");
            io::stdout().flush().unwrap();
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read input");
            
            // Parse it.
            match user_input.trim() {
                "q" => break 'mains,
                "" => break 'mains,
                user_input => {
                    if let Ok(numbers) = parse_input(user_input) {
                        _user_vals = numbers;
                        break 'usr_in;
                    }
                    else {
                        println!("Please only enter numbers...");
                    }
                }
            }
        }
        
        // Calculate mean
        let mut vec_len_float = _user_vals.len() as f32;
        let mean_str = String::from("mean");
        // let mean_val = sum(_user_vals) / _user_vals.len();
        let mut mean_val: f32;
        let mut _mean_val = 0;
        for elem in _user_vals.iter_mut() {
            _mean_val += *elem;
            // println!("Mean_val = {:?}", &_mean_val); //DEBUG
        }
        if vec_len_float == 0.0 {
            vec_len_float = 1.0;
        }
        mean_val = _mean_val as f32;
        mean_val /= vec_len_float;
        // println!("Vector values = {:?}", &_user_vals); //DEBUG
        // println!("Vector length = {:?}", &vec_len_float); //DEBUG
        // println!("Mean_val = {:?}", &mean_val); //DEBUG
        triple_m.insert(&mean_str, mean_val);

        // Calculate median
        let mut vec_len_int = _user_vals.len() as i32;
        if vec_len_int == 0 {
            vec_len_int = 1;
        }
        let median_str = String::from("median");
        let median_val: f32;
        let mut _temp_vec = _user_vals.sort();
        let _half_vec_even = (vec_len_int/2) as usize;
        // let _half_vec_odd = (vec_len/2) as usize;
        match vec_len_int%2 {
            0 => median_val = (_user_vals[_half_vec_even] + _user_vals[(_half_vec_even)+1]) as f32/2.0,
            _ => median_val = _user_vals[_half_vec_even] as f32
        }
        // println!("Median val = {:?}", median_val); //DEBUG
        triple_m.insert(&median_str, median_val);

        // Calculate mode
        let mode_str = String::from("mode");
        let mode_val: f32;
        let mut _mode_val = 0;
        let mut _temp_mode = 0;

        for number in _user_vals {
            let count = mode_hash.entry(number).or_insert(0);
            *count += 1;
        }

        for (key, value) in &mode_hash {
            println!("{}", format!("Key: {} \t Value: {} --> Mode: {} / Temp Mode: {}", key, value, _mode_val, _temp_mode));
            match value.cmp(&_mode_val) {
                Ordering::Less => continue,
                Ordering::Greater => _mode_val = *key,
                Ordering::Equal => { 
                    println!("Cannot have two modes!");
                    _temp_mode = *key;
                }
            }
        }

        if _mode_val == _temp_mode {
            _mode_val = 999;
        }
        // match _mode_val {
        //     _temp_mode => {
        //         _mode_val = 999;
        //     }
        // }
        mode_val = _mode_val as f32;

        println!("{:?}", mode_hash); //DEBUG
        triple_m.insert(&mode_str, mode_val);

        let output = format!("Mean = {:?} Median = {:?} Mode = {:?}", triple_m.get(&mean_str), triple_m.get(&median_str), triple_m.get(&mode_str));
        println!("{}", output);
    }
}

fn parse_input(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    input
        .split_whitespace()
        .map(|token| token.parse())
        .collect()
}