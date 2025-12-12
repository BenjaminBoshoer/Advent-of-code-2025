use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

pub fn day_1() -> Result<i32, String> {
    //Read the file inputs/day_1.txt. On error return the error string.
    let file = match File::open("inputs/day_1.txt") {
        Ok(n) => n,
        Err(e) => {
            println!("Error opening file:");
            return Err(e.to_string());
        }
    };

    let reader = BufReader::new(file);

    // Define clicks and pointer for following dial rotations
    let mut clicks: i32 = 0;
    let mut pointer = 50;

    // Iterate the file line by line
    for line in reader.lines() {
        let line = line.unwrap();

        let degree = line[1..].trim().parse::<i32>().unwrap();
        let direction = &line[0..1];

        // Use the match keyword to distinct between R and L behavior
        match direction {
            "R" => {
                //print!("{}{}: {} ", direction, degree, pointer);

                if pointer + (degree % 100) > 99 {
                    clicks += 1;
                    //print!("click ");
                } 

                pointer = (pointer + (degree % 100)) % 100;

                //print!("-> {}", pointer);
                clicks += degree / 100;
                
                //println!(" Added {} clicks due to rotations", degree/100);

            }
            "L" => {
                //print!("{}{}: {} ->", direction, degree, pointer);
                if pointer == 0 {
                    pointer = 100 - (degree.abs() % 100);
                } else if pointer - (degree.abs() % 100) < 0{
                    clicks += 1;
                    //print!(" Click ");
                    pointer = 100 - (pointer - (degree.abs() % 100)).abs() % 100;
                } else if pointer - (degree.abs() % 100) == 0{
                    clicks += 1;
                    //print!(" Click ");
                    pointer = 0;
                } else {
                    pointer = pointer - (degree % 100);
                }
                //print!(" {}", pointer);
                clicks += degree / 100;
                //println!(" Added {} clicks due to rotations", degree/100);
            }
            _ => {}
        }
    }

    Ok(clicks)
}


pub fn day_2() -> Result<i64, String> {
    //Read the file inputs/day_1.txt. On error return the error string.
    let buffer = match fs::read_to_string("inputs/day_2.txt") {
        Ok(n) => n,
        Err(e) => {
            println!("Error opening file:");
            return Err(e.to_string());
        }
    };
    let mut sum: i64 = 0;
    let ranges: Vec<_> = buffer.split(',').collect();

    for range in ranges {
        let v: Vec<_> = range.split('-').collect();
        let start = v[0].trim().parse::<i64>().unwrap();
        let end = v[1].trim().parse::<i64>().unwrap();

        let mut ptr = start;
        for _ in 0..=(end-start) {
            let len = ptr.to_string().len();
            if len % 2 == 0 {
                if ptr.to_string()[0..len/2] == ptr.to_string()[len/2..] {
                    sum += ptr;
                    println!("invalid ID {}", ptr);
                }
            }
            ptr += 1
        }
    }
    Ok(sum)
}