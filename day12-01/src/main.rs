use std::fs::File;
use std::io::prelude::*;

fn read(file_name: &str) -> Result<String, bool> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Oops. No can read here?!");
    return Ok(contents);
}

fn parse(input: String) -> Vec<i32> {
    let mut vals: Vec<i32> = Vec::new();

    for part in input.split('\n') {
        let res = part.parse::<i32>();
        if res.is_err() {
            println!("Parse error at: '{}'", part);
            return vec![];
        }

        let num_val = res.unwrap();
        vals.push(num_val);
        //println!("{}", num_val);
    }

    return vals;
}

fn compute_freq(start: i32, changes: Vec<i32>) -> i32 {
    let mut freq = start;
    for change in changes {
        freq += change
    }
    return freq;
}

fn find_duplicate_freq(start: i32, changes: Vec<i32>) -> i32 {
    let mut freq = start;
    let mut freqs: Vec<i32> = Vec::new();

    loop {
        for change in &changes {
            freq += change;

            if freqs.contains(&freq) {
                return freq;
            }

            freqs.push(freq);
        }
    };
}

fn main() -> std::io::Result<()> {
    println!("Frequency: {}", compute_freq(0, parse(read("example").unwrap())));
    println!("Frequency: {}", compute_freq(0, parse(read("input1").unwrap())));
    println!("Duplicate freqency: {}", find_duplicate_freq(0, parse(read("example").unwrap())));
    println!("Duplicate freqency: {}", find_duplicate_freq(0, parse(read("input1").unwrap())));
    return Ok(());
}
