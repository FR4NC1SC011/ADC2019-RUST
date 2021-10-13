use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::ptr::null_mut;

pub fn third_a() {
    let mut is_wire_1: bool = true;

    let wires: String = read_file();
    let mut coords_1: HashMap<i32, i32> = HashMap::new();
    let mut coords_2: HashMap<i32, i32> = HashMap::new();

    // let (mut x, mut y): (i32, i32) = (0, 0);
    let (mut a, mut b): (i32, i32) = (0, 0);

    for wire in wires.lines() {
        if is_wire_1 {
            let w: Vec<&str> = wire.split(",").collect();
            let mut i = 0;
            loop {
                if i >= w.len() {
                    break;
                } else if i == w.len() - 1 {
                    // Last element
                    let (x, y) = update_single_coords(w[i], a, b);
                    a = x;
                    b = y;
                    coords_1.insert(a, b);
                    println!("X: {} : Y: {}", a, b);
                    println!("{:?}", coords_1);
                    break;
                }

                let (x, y) = update_both_coords(w[i], w[i + 1], a, b);
                a = x;
                b = y;
                coords_1.insert(a, b);

                println!("X: {} : Y: {}", a, b);
                println!("{:?}", coords_1);

                i += 2;
            }
            is_wire_1 = false;
        }
    }
}

fn update_both_coords(p1: &str, p2: &str, mut x: i32, mut y: i32) -> (i32, i32) {
    // Update X & Y coords

    let sign1 = &p1[..1];
    let number_str1 = &p1[1..];
    let number_int1: i32 = number_str1.parse().unwrap();

    if sign1 == "R" {
        x += number_int1;
    } else if sign1 == "L" {
        x -= number_int1;
    } else if sign1 == "U" {
        y += number_int1;
    } else if sign1 == "D" {
        y -= number_int1;
    }

    let sign2 = &p2[..1];
    let number_str2 = &p2[1..];
    let number_int2: i32 = number_str2.parse().unwrap();

    if sign2 == "U" {
        y += number_int2;
    } else if sign2 == "D" {
        y -= number_int2;
    } else if sign2 == "R" {
        x += number_int2;
    } else if sign2 == "L" {
        x -= number_int2;
    }

    // println!("{}, {}", x, y);

    (x, y)
}

fn update_single_coords(p1: &str, mut x: i32, mut y: i32) -> (i32, i32) {
    // Update X & Y coords
    let sign1 = &p1[..1];
    let number_str1 = &p1[1..];
    let number_int1: i32 = number_str1.parse().unwrap();

    if sign1 == "R" {
        x += number_int1;
    } else if sign1 == "L" {
        x -= number_int1;
    } else if sign1 == "U" {
        y += number_int1;
    } else if sign1 == "D" {
        y -= number_int1;
    }
    // println!("{}, {}", x, y);

    (x, y)
}

fn read_file() -> String {
    let file = "../puzzle_input/example3.txt";

    let data = fs::read_to_string(file).expect("Something went wrong when reading the file");

    data
}
