use std::fs;

fn main() {
    let ints = parse_input("./1/input");
    let increases = count_increases(ints);
    println!("{}", increases);
}

fn count_increases(readings: Vec<i32> ) -> usize
{
    return readings.windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count();
}

fn parse_input(filename: &str) -> Vec<i32> {
    match  fs::read_to_string(filename) {
        Err(msg) => panic!("{}", msg),
        Ok(contents) => {
            return contents.lines()
                .map(str::parse::<i32>)
                .map(|result| match result {
                    Err(msg) => panic!("{}", msg),
                    Ok(n) => return n
                })
                .collect::<Vec<i32>>()
        }
    }
}
