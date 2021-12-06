use std::fs;
use std::collections::HashMap;

type ParseTarget = HashMap<usize, i32>;
type Solution = i32;

const EXAMPLES: [(&str, usize, Solution); 2] = [
    ("1", 18, 26),
    ("1", 80, 5934)
];

const DAY: u8 = 6;

fn main() {
    let results = EXAMPLES.iter()
        .zip(
            EXAMPLES.iter()
                .map(|t| (t.1, format!("{}/{}.ie", DAY, t.0)))
                .map(|(arg, parsed)| operation(arg, parsed))
        )
        .map(|((name, n, expected), result)| 
            (
                *name,
                *n,
                result
                    .and_then(|actual| if *expected == actual {
                        return Ok(());
                    } else {
                        return Err(format!("Expected {} but got {}", expected, actual));
                    })
            )
        )
        .collect::<Vec<(&str, usize, Result<(), String>)>>();
    results.iter()
        .for_each(|(name, n, result)| match result {
            Ok(()) => println!("Example {}({}) passed.", name, n),
            Err(msg) => println!("Example {}({}) failed: {}.", name, n, msg)
        });

    if results.iter().any(|t| t.2.is_err()) {
        panic!("Please address errors before attempting the problem.")
    }

    println!(
        "{}",
        operation(80, format!("{}/input", DAY)).expect("Unexpected Error in main input.")
    );
}

//fn error<T>(msg: &str) -> Result<T, String> {
//    return Err(String::from(msg));
//}

fn operation(n: usize, filename: String) -> Result<Solution, String> {
    return fs::read_to_string(filename)
        .map_err(|io_error| format!("{}", io_error))
        .and_then(parse)
        .and_then(|parsed| solve(n, parsed));
}

fn parse(contents: String) -> Result<ParseTarget, String> {
    let ages = contents.trim().split(",")
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()
        .map_err(|e| format!("Error parsing: {}", e))?;
    let mut to_return = HashMap::new();
    for age in ages {
        to_return.insert(
            age,
            to_return.get(&age).unwrap_or(&0) + 1
        );
    }
    return Ok(to_return);
}

fn solve(days: usize, parsed: ParseTarget) -> Result<Solution, String> {
    return Ok((0..days)
        .fold(parsed, |curr, _| next_day(curr))
        .values()
        .fold(0, |a, b| a + b)
        );
}

fn next_day(curr: HashMap<usize, i32>) -> HashMap<usize, i32> {
    let mut next = HashMap::new();
    for age in 0..=7 {
        next.insert(
            age,
            *curr.get(&(age + 1 as usize)).unwrap_or(&0)
        );
    }
    let spawns = curr.get(&0).unwrap_or(&0);
    next.insert(
        6,
        next.get(&6).unwrap_or(&0) + spawns
    );
    next.insert(8, *spawns);
    return next;
}


