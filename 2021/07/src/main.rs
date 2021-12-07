use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

fn min_fuel_part1(input: &Vec<i64>) -> Option<i64> {
    // Calculate minimum and maximum height of all crabs
    let min_height = *input.iter().min()?;
    let max_height = *input.iter().max()?;

    // For all target heights between the minimum and maximum of all crab heights...
    (min_height..=max_height).map(|target_height|
        // ... calculate the sum of the distance of each crab height to the target height
        input.iter()
            .map(|crab_height| i64::abs(target_height - crab_height))
            .sum()
        // and find the minimum
    ).min()
}

fn min_fuel_part2(input: &Vec<i64>) -> Option<i64> {
    // Calculate minimum and maximum height of all crabs
    let min = *input.iter().min()?;
    let max = *input.iter().max()?;

    // For all target heights between the minimum and maximum of all crab heights...
    (min..=max).map(|target_height|
        // ... calculate the gaussian summation formula of the distance of each crab height to the target height,
        // because the fuel cost is 1+2+3+4+...
        input.iter()
            .map(|crab_height| gaussian_summation_formula(i64::abs(target_height - crab_height)))
            .sum()
        // and find the minimum
    ).min()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?;

    println!("Minimum fuel part 1: {:?}", min_fuel_part1(&input));
    println!("Minimum fuel part 2: {:?}", min_fuel_part2(&input));

    Ok(())
}

/// Reads the input file and parses it.
fn read_input() -> Result<Vec<i64>, io::Error> {
    // Read file to string
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;
    // Parse comma seperated strings to numbers
    Ok(input.split(',')
        .map(|num_str| num_str.parse().unwrap())
        .collect()
    )
}

/// Calculates the sum of the first n integers with the gaussian summation formula.
/// Example: 1+2+3+4+5 == gaussian_summation_formula(5)
fn gaussian_summation_formula(n: i64) -> i64 {
    (n*n + n) / 2
}
