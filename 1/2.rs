use std::fs;

fn main() {
    let ints = parse_input("./1/input");
    let increases = count_increases(ints);
    println!("{}", increases);
}

fn count_increases(readings: Vec<i32> ) -> usize
{
    return readings.windows(3)
        .map(|triad| triad.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count()
}

fn parse_input(filename: &str) -> Vec<i32> {
    return fs::read_to_string(filename)
        .expect("IO Error")
        .lines()
        .map(str::parse::<i32>)
        .map(|result| result.expect("Parse Error"))
        .collect()
}
