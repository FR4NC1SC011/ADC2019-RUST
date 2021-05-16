use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn second_a() {
    let mut vec_int = Vec::new();
    let input = read_file();
    let int_code = str::replace(&input, ",", "\n");

    let s = int_code.lines();

    for x in s {
        let i = x.parse::<i32>().unwrap();
        vec_int.push(i);
    }

    eprintln!("{:?}", vec_int);


    let mut code = vec_int[0];
    let mut j = 0;

    for i in 0..vec_int.len() {
        if vec_int[j] == 99 {
            break;
        }

        eprintln!("code: {}", code);
        match code {
            1 => {
                let pos1 = vec_int[j + 1];
                let pos2 = vec_int[j + 2];
                let pos3 = vec_int[j + 3];
                let add = vec_int[pos1 as usize] + vec_int[pos2 as usize];
                vec_int[pos3 as usize] = add;
                eprintln!("Add: {}", add);
            },
            2 => {
                let pos1 = vec_int[j + 1];
                let pos2 = vec_int[j + 2];
                let pos3 = vec_int[j + 3];
                let mul = vec_int[pos1 as usize] * vec_int[pos2 as usize];
                vec_int[pos3 as usize] = mul;
                eprintln!("Mul: {}", mul);
            },
            99 => {
                eprintln!("break");
                break;
            },
            _ => eprintln!("values"),
        } 
        
        j += 4;
        code = vec_int[j];
    }


    eprintln!("{:?}", vec_int);

    
}



fn read_file() -> String {

    let mut file = File::open("/home/fco/Desktop/Rust/ADC2019-RUST/program_alert1202/input1.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    contents
}
