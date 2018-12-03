use std::collections::HashMap;
use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Rectangle {
    x: u16,
    y: u16,
    w: u16,
    h: u16
}

impl Rectangle {
    pub fn new() -> Rectangle {
        Rectangle { x: 0, y: 0, w: 0, h: 0 }
    }

    pub fn fromValues(x: u16, y: u16, w: u16, h: u16) -> Rectangle {
        Rectangle { x, y, w, h }
    }

    fn contains(&self, x: u16, y: u16) -> bool {
        (self.x <= x && self.x + self.w >= x) &&
        (self.y <= y && self.y + self.w >= y)
    }

    pub fn intersected(&self, other: &Rectangle) -> Rectangle {
        // Map ours to Qt's implementation
        let x1 = self.x;
        let y1 = self.y;
        let x2 = self.x + self.w;
        let y2 = self.y + self.h;
        
        // Qt's QRect::intersected() implementation (slightly simplified)
        let mut l1 = x1;
        let mut r1 = x1;
        if x2 - x1 + 1 < 0 { // TODO Remove
            l1 = x2;
        } else {
            r1 = x2;
        }

        let mut l2 = other.x;
        let mut r2 = other.x;
        if other.w + 1 < 0 {
            l2 = other.x + other.w;
        } else {
            r2 = other.x + other.w;
        }

        if l1 > r2 || l2 > r1 {
            return Rectangle::new();
        }

        let mut t1 = y1;
        let mut b1 = y1;
        if y2 - y1 + 1 < 0 {
            t1 = y2;
        } else {
            b1 = y2;
        }

        let mut t2 = other.y;
        let mut b2 = other.y;
        if other.h + 1 < 0 {
            t2 = other.y + other.h;
        } else {
            b2 = other.y + other.h;
        }

        if t1 > b2 || t2 > b1 {
            return Rectangle::new();
        }

        let x1 = cmp::max(l1, l2);
        let y1 = cmp::max(t1, t2);
        let x2 = cmp::min(r1, r2);
        let y2 = cmp::min(b1, b2);
        let w = x2 - x1;
        let h = y2 - y1;
        Rectangle::fromValues(x1, y1, w, h)
    }

    pub fn area(&self) -> u32 {
        self.w as u32 * self.h as u32
    }
}

impl FromStr for Rectangle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example: #1 @ 1,3: 4x4

        // Ignore the ID for now.
        let perc: Vec<&str> = s.split('@').collect();
        let dimensions: Vec<&str> = perc[1].split(':').collect();
        let pos: Vec<&str> = dimensions[0].split(',').collect();
        let size: Vec<&str> = dimensions[1].split('x').collect();

        let x: u16 = pos[0].trim_matches(|c| c == ' ')
                           .parse()?;
        let y: u16 = pos[1].parse()?;
        let w: u16 = size[0].trim_matches(|c| c == ' ')
                            .parse()?;
        let h: u16 = size[1].parse()?;

        Ok(Rectangle { x: x, y: y, w: w, h: h })
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

fn intersect(rects: &Vec<Rectangle>) -> HashMap<u32, u32> {
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

fn main() {
    println!("Overlapped In²: {:?}", find_conflicting_inches(&intersect(&parse(read("example").unwrap()))));
    println!("Overlapped In²: {:?}", find_conflicting_inches(&intersect(&parse(read("input1").unwrap()))));
}
