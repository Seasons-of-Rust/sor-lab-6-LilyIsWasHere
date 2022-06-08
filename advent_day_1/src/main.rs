use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let module_masses: Vec<u32> = BufReader::new(File::open("input.txt").expect("file not found"))
        .lines() // Go through each line
        .map(|line| {
            line.expect("could not parse line") // Unwrap the result of the line
                .parse() // Try to parse it to what we expect (i32 from the annotation)
                .expect("could not parse number") // Unwrap the result of the parse
        })
        .collect();

    println!("{:?}", module_masses);

    let fuel_requirements: Vec<u32> = module_masses
        .iter()
        .map(fuel_required)
        .collect();

    let total_fuel: u32 = fuel_requirements.iter().sum();

    println!("Total fuel required: {:?}", total_fuel);
}

fn fuel_required(mass: &u32) -> u32 {
    let fuel = (mass / 3).saturating_sub(2);
    if fuel == 0 {
        0
    } else {
        fuel + fuel_required(&fuel)
    }
}
