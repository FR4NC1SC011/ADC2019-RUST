use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn first_a() {
    let mut mass: i64;
    let mut fuel_required: i64;
    let mut sum: i64 = 0;
    /*
    let mut file = std::fs::File::open("example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
    */

    let lines = read_file();
    for line in lines {
        mass = line.parse::<i64>().unwrap();
        fuel_required = (mass / 3) - 2;
        sum += fuel_required;
    }

    println!("Sum of the fuel requirements: {}", sum);

}

fn read_file() -> Vec<String> {
    let file = File::open("../puzzle_input/input_1.txt").expect("No such file.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
