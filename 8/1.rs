use std::fs;
use std::collections::HashSet;

type ParseTarget = Vec<Display>;
type Solution = usize;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 26)
];

const DAY: u8 = 8;

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

//fn error<T>(msg: &str) -> Result<T, String> {
//    return Err(String::from(msg));
//}

fn operation(filename: String) -> Result<Solution, String> {
    return fs::read_to_string(filename)
        .map_err(|io_error| format!("{}", io_error))
        .and_then(parse)
        .and_then(solve);
}

struct Display {
    //signal_patterns: Vec<Digit>,
    output: Vec<Digit>
}

impl Display {
    fn from_string(line: &str) -> Result<Display, String> {
        let pieces: Vec<&str> = line.split(" | ").collect();
        //let patterns = pieces.get(0)
        //    .ok_or(String::from("No LHS"))
        //    .and_then(|s| Digit::from_string_many(*s))?;

        let out = pieces.get(1)
            .ok_or(String::from("No RHS"))
            .and_then(|s| Digit::from_string_many(*s))?;

        return Ok(Display {
            //signal_patterns: patterns,
            output: out
        })
    }
}

struct Digit {
    wires: HashSet<char>
}

impl Digit {
    fn from_string_many(raw: &str) -> Result<Vec<Digit>, String> {
        return raw.split_whitespace()
            .map(Digit::from_string)
            .collect();
    }
    fn from_string(raw: &str) -> Result<Digit, String> {
        return Ok(Digit {
            wires: raw.chars().collect()
        })
    }
}

fn parse(contents: String) -> Result<ParseTarget, String> {
    return contents.lines()
        .map(Display::from_string)
        .collect::<Result<Vec<Display>, _>>()
        .map_err(|e| format!("Parse Error: {}", e));
}

const UNIQUE_SIZES: [usize; 4] = [2,3,4,7];

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    return Ok(parsed.iter()
        .flat_map(|d| d.output.iter())
        .filter(|d| UNIQUE_SIZES.iter().any(|l| *l == d.wires.len()))
        .count());
}
