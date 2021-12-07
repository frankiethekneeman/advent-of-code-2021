use std::fs;

type ParseTarget = Vec<i32>;
type Solution = i32;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 168)
];

const DAY: u8 = 7;

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

fn parse(contents: String) -> Result<ParseTarget, String> {
    return contents.trim()
        .split(",")
        .map(str::parse)
        .collect::<Result<ParseTarget, _>>()
        .map_err(|e| format!("Parse Error: {}", e));
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    let min = *parsed.iter().min().ok_or("No minimum")?;
    let max = *parsed.iter().max().ok_or("No maximum")?;
    return (min..=max).map(|p|
            parsed.iter()
                .map(|c| i32::abs(p - c))
                .map(|n| n * (n + 1) / 2)
                .sum()
        )
        .min()
        .ok_or(String::from("Could not calculuate fuel usage"));

}
