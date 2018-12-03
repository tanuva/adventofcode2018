use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn read(file_name: &str) -> Result<String, bool> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Oops. No can read here?!");
    return Ok(contents);
}

fn count_same_letters(id: &str) -> HashMap<char, u8> {
    let mut counts: HashMap<char, u8> = HashMap::new();
    
    for c in id.chars() {
        // Now that's a fancy idiom.
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    return counts;
}

// Take the id list as one string so that we can use the efficient String::lines()
fn find_similar_ids(id_list: String, expected_letter_counts: &Vec<u8>) -> HashMap<u8, u8> {
    let ids = id_list.lines();
    let mut id_counts: HashMap<u8, u8> = HashMap::new();

    // Prepare the output map
    for count in expected_letter_counts {
        id_counts.insert(*count, 0);
    }
    
    for id in ids {
        let letter_counts = count_same_letters(id);
        //println!("ID: {} Counts: {:?}", id, letter_counts);
        //println!("id_counts: {:?}", id_counts);
        
        for expected_letter_count in expected_letter_counts {
            for letter_count in letter_counts.values() {
                if *letter_count == *expected_letter_count {
                    let id_count = id_counts.entry(*letter_count).or_insert(0);
                    *id_count += 1;
                    break; // Try to be equivalent to something alike "letter_counts.unique_values()"
                }
            }
        }
    }

    return id_counts;
}

fn count_equal_chars(a: &str, b: &str) -> u8 {
    // TODO
    return 0;
}

fn find_more_similar_ids(id_list: String) -> Vec<String> {
    let mut start_idx = 0;
    let mut result: Vec<String> = Vec::new();

    // First, make sure to compare every id only to the ids coming after it
    // to half the number of comparisons made.
    for id in id_list.lines() {
        let mut tmp = 0;
        let other = id_list.lines().skip_while(move |_id| { // Does the 'move' really do what I expect?
            if tmp < start_idx {
                tmp += 1;
                return true;
            }
            return false;
        });
        start_idx += 1;

        while other.has_next() {
            let equal_chars = count_equal_chars(id, other);
            other.next();
        }
    }

    return result;
}

fn main() {
    let inputs = vec!["example", "input1"];
    let interesting_counts = vec![2, 3];
    for input in inputs {
        let counts = find_similar_ids(read(input).unwrap(), &interesting_counts);
        println!("{} * {} = {}", counts[&2], counts[&3], counts[&2] as u16 * counts[&3] as u16);
    }

    /*let counts = find_similar_ids(read("example").unwrap(), vec![2, 3]);
    println!("{} * {} = {}", counts[&2], counts[&3], counts[&2] * counts[&3]);*/
}
