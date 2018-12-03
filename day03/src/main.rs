use std::collections::HashMap;
use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;
use std::string::String;

#[derive(Debug)]
struct Rectangle {
    id: String,
    x: u16,
    y: u16,
    w: u16,
    h: u16
}

impl Rectangle {
    pub fn new() -> Rectangle {
        Rectangle { id: String::new(), x: 0, y: 0, w: 0, h: 0 }
    }

    pub fn from_values(x: u16, y: u16, w: u16, h: u16) -> Rectangle {
        Rectangle { id: String::new(), x, y, w, h }
    }

    pub fn intersected(&self, other: &Rectangle) -> Rectangle {
        // Map ours to Qt's implementation
        let x1 = self.x;
        let y1 = self.y;
        let x2 = self.x + self.w - 1;
        let y2 = self.y + self.h - 1;
        
        // Qt's QRect::intersected() implementation (slightly simplified)
        let l1 = x1;
        let r1 = x2;

        let l2 = other.x;
        let r2 = other.x + other.w - 1;

        if l1 > r2 || l2 > r1 {
            return Rectangle::new();
        }

        let t1 = y1;
        let b1 = y2;

        let t2 = other.y;
        let b2 = other.y + other.h - 1;

        if t1 > b2 || t2 > b1 {
            return Rectangle::new();
        }

        let x1 = cmp::max(l1, l2);
        let y1 = cmp::max(t1, t2);
        let x2 = cmp::min(r1, r2);
        let y2 = cmp::min(b1, b2);
        let w = x2 - x1 + 1;
        let h = y2 - y1 + 1;
        Rectangle::from_values(x1, y1, w, h)
    }

    pub fn is_null(&self) -> bool {
        self.w == 0 && self.h == 0
    }
}

impl FromStr for Rectangle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example: #1 @ 1,3: 4x4

        let perc: Vec<&str> = s.split('@').collect();
        let id: String = perc[0].trim_matches(|c| c == ' ').to_string();
        let dimensions: Vec<&str> = perc[1].split(':').collect();
        let pos: Vec<&str> = dimensions[0].split(',').collect();
        let size: Vec<&str> = dimensions[1].split('x').collect();

        let x: u16 = pos[0].trim_matches(|c| c == ' ')
                           .parse()?;
        let y: u16 = pos[1].parse()?;
        let w: u16 = size[0].trim_matches(|c| c == ' ')
                            .parse()?;
        let h: u16 = size[1].parse()?;

        Ok(Rectangle { id: id, x: x, y: y, w: w, h: h })
    }
}

fn read(file_name: &str) -> Result<String, bool> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Oops. No can read here?!");
    return Ok(contents);
}

fn parse(s: String) -> Vec<Rectangle> {
    let mut rects: Vec<Rectangle> = Vec::new();

    for line in s.lines() {
        rects.push(line.parse().expect("Failed to parse a Rectangle"));
    }

    return rects;
}

fn build_occupancy_map(rects: &Vec<Rectangle>) -> HashMap<u32, u32> {
    let mut inches: HashMap<u32, u32> = HashMap::new();
    for rect in rects {
        for x in rect.x..(rect.x + rect.w) {
            for y in rect.y..(rect.y + rect.h) {
                let occupants = inches.entry(y as u32 * 1000 + x as u32).or_insert(0);
                *occupants += 1;
            }
        }
    }
    return inches;
}

fn find_conflicting_inches(inches: &HashMap<u32, u32>) -> u32 {
    let mut conflicts: u32 = 0;
    for inch in inches.values() {
        if inch > &1 {
            conflicts += 1;
        }
    }
    return conflicts;
}

fn find_lonely_rect(rects: &Vec<Rectangle>) -> Result<String, bool> {
    let mut conflicts: HashMap<String, u16> = HashMap::new();

    // This approach avoids the n² intersections. Alas, I cannot currently see
    // how to handle the two borrows for num_conflicts_{a,b} correctly.
    /*let mut start_idx: usize = 1;
    for rect_a in rects {
        for i in start_idx..rects.len() {
            let rect_b = &rects[i];
            
            // Fetch the counts already to make sure we always insert a zero
            let num_conflicts_a = conflicts.entry(rect_a.id.to_string()).or_insert(0);
            let num_conflicts_b = conflicts.entry(rect_b.id.to_string()).or_insert(0);
            
            let intersection = rect_a.intersected(rect_b);
            if !intersection.is_null() {
                //println!("{} x {}: {:?}", rect_a.id, rect_b.id, intersection);
                *num_conflicts_a += 1;
                *num_conflicts_b += 1;
            }
        }

        start_idx += 1;
    }*/

    // Yeah, sure, O(n²), but it is simple.
    for rect_a in rects {
        for rect_b in rects {
            if rect_a.id == rect_b.id {
                continue;
            }

            let num_conflicts = conflicts.entry(rect_a.id.to_string()).or_insert(0);
            let intersection = rect_a.intersected(rect_b);
            if !intersection.is_null() {
                //println!("{} x {}: {:?}", rect_a.id, rect_b.id, intersection);
                *num_conflicts += 1;
            }
        }
    }

    for id in conflicts.keys() {
        if conflicts[id] == 0 {
            return Ok(id.to_string());
        }
    }

    Err(false)
}

fn main() {
    let example_rects = parse(read("example").unwrap());
    println!("Overlapped In²: {:?}", find_conflicting_inches(&build_occupancy_map(&example_rects)));
    println!("Lonely rect: {}", find_lonely_rect(&example_rects).unwrap());

    let input_rects = parse(read("input1").unwrap());
    println!("Overlapped In²: {:?}", find_conflicting_inches(&build_occupancy_map(&input_rects)));
    println!("Lonely rect: {}", find_lonely_rect(&input_rects).unwrap());
}
