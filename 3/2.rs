use std::fs;

type ParseTarget = Vec<String>;
type Solution = u32;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 230)
];

const DAY: u8 = 3;

fn main() {
    let results = EXAMPLES.iter()
        .zip(
            EXAMPLES.iter()
                .map(|t| format!("{}/{}.ie", DAY, t.0))
                .map(operation)
        )
        .map(|((name, expected), result)| 
            (
                *name,
                result
                    .and_then(|actual| if *expected == actual {
                        return Ok(());
                    } else {
                        return Err(format!("Expected {} but got {}", expected, actual));
                    })
            )
        )
        .collect::<Vec<(&str, Result<(), String>)>>();
    results.iter()
        .for_each(|(name, result)| match result {
            Ok(()) => println!("Example {} passed.", name),
            Err(msg) => println!("Example {} failed: {}.", name, msg)
        });

    if results.iter().any(|t| t.1.is_err()) {
        panic!("Please address errors before attempting the problem.")
    }

    println!(
        "{}",
        operation(format!("{}/input", DAY)).expect("Unexpected Error in main input.")
    );
}

fn error<T>(msg: &str) -> Result<T, String> {
    return Err(String::from(msg));
}

fn operation(filename: String) -> Result<Solution, String> {
    return fs::read_to_string(filename)
        .map_err(|io_error| format!("{}", io_error))
        .and_then(parse)
        .and_then(solve);
}

fn parse(contents: String) -> Result<ParseTarget, String> {
    return Ok(contents.lines()
        .map(String::from)
        .collect());
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    let o2 = bin_2_num(get_oxygen_rating(parsed.clone()))?;
    let co2 = bin_2_num(get_co2_rating(parsed))?;
    return Ok(o2 * co2);
}

fn bin_2_num(r: Result<String, String>) -> Result<u32, String> {
    return r.and_then(|s| u32::from_str_radix(&s, 2)
        .map_err(|e| format!("{}", e))
    );
}

fn get_oxygen_rating(readings: Vec<String>) -> Result<String, String> {
    return recursive_filter(readings, 0, |x| x);
}

fn get_co2_rating(readings: Vec<String>) -> Result<String, String> {
    return recursive_filter(readings, 0, not);
}

fn recursive_filter<F>(remaining: Vec<String>, pos: usize, transform: F) -> Result<String, String>
    where F: Fn(char) -> char {
    let bits = remaining.iter().map(|s| s.chars().nth(pos).unwrap_or(' ')).collect();
    let winner = transform(most_popular_bit(bits));
    let rest = remaining.into_iter()
        .filter(|s| s.chars().nth(pos).unwrap_or(' ') == winner)
        .collect::<Vec<String>>();
    if rest.len() == 1 {
        return Ok(rest[0].clone());
    } else if rest.len() == 0 {
        return error("Cannot decide")
    } else {
        return recursive_filter(rest, pos + 1, transform);
    }
}

fn most_popular_bit(bits: Vec<char>) -> char {
    let limit = (bits.len() + 1)/2;
    let ones = bits.into_iter()
        .filter(|c| *c == '1')
        .count();
    if ones >= limit {
        return '1'
    } else {
        return '0';
    }
}


fn not(c: char) -> char {
    return match c {
        '0' => '1',
        '1' => '0',
        _ => panic!("nonbinary character")
    }
}
