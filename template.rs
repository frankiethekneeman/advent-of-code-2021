use std::fs;
use std::error::Error;

type ParseTarget = Vec<i32>;
type Solution = usize;
type AoC<T> = Result<T, Box<dyn Error>>;

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
        .map(|((name, expected), result)| 
            (
                *name,
                result
                    .and_then(|actual| if *expected == actual {
                        return Ok(());
                    } else {
                        return error(format!("Expected {} but got {}", expected, actual));
                    })
            )
        )
        .collect::<Vec<(&str, AoC<()>)>>();
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

fn errorize<S: Into<Box<dyn Error>>>(msg: S) -> Box<dyn Error> {
    return msg.into();
}

fn error<T, S: Into<Box<dyn Error>>>(err: S) -> AoC<T> {
    return Err(err.into());
}

fn operation(filename: String) -> AoC<Solution> {
    return fs::read_to_string(filename)
        .map_err(errorize)
        .and_then(parse)
        .and_then(solve);
}

fn parse(contents: String) -> AoC<ParseTarget> {
    return error("Parse Not Yet Implemented");
}

fn solve(parsed: ParseTarget) -> AoC<Solution> {
    return error("Solve Not Yet Implemented");
}
