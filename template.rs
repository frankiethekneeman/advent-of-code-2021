use std::fs;

type ParseTarget = Vec<i32>;
type Solution = usize;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 150)
];

const DAY: u8 = 1;

fn main() {
    let results = EXAMPLES.iter()
        .zip(
            EXAMPLES.iter()
                .map(|t| format!("{}/{}.ie", DAY, t.0))
                .map(operation)
        )
        .map(|tuple| match tuple {
            ((name, expected), result) => (*name, result
                .and_then(|actual| if *expected == actual {
                    return Ok(());
                } else {
                    return Err(format!("Expected {} but got {}", expected, actual));
                }))
        })
        .collect::<Vec<(&str, Result<(), String>)>>();
    results.iter()
        .for_each(|tuple| match tuple {
            (name, Ok(())) => println!("Example {} passed.", name),
            (name, Err(msg)) => println!("Example {} failed: {}.", name, msg)
        });

    if results.iter().any(|t| t.1.is_err()) {
        panic!("Please address errors before attempting the problem.")
    }

    operation(format!("{}/input", DAY)).expect("Unexpected Error in main input.");
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
    return error("Not Yet Implemented");
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    return error("Not Yet Implemented");
}
