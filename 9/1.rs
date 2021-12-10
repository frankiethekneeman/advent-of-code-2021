use std::fs;
use std::convert::TryFrom;

type ParseTarget = Vec<Vec<usize>>;
type Solution = usize;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 15)
];

const DAY: u8 = 9;

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
    return contents.lines()
        .into_iter()
        .map(str::chars)
        .map(|row| row.map(|c| c.to_digit(10)
            .map(usize::try_from)
            .unwrap()
            .map_err(|e| format!("{}", e))
        ))
        .map(Iterator::collect)
        .collect();
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    return Ok(parsed.iter()
        .enumerate()
        .map(|(x, row)| row.iter()
                .enumerate()
                .filter(|(y, height)| {
                    let up = *parsed.get(x.overflowing_sub(1).0)
                        .and_then(|r| r.get(*y))
                        .unwrap_or(&10);
                    let down = *parsed.get(x + 1)
                        .and_then(|r| r.get(*y))
                        .unwrap_or(&10);
                    let left = *row.get(y.overflowing_sub(1).0)
                        .unwrap_or(&10);
                    let right = *row.get(y + 1)
                        .unwrap_or(&10);
                    return [up, down, left, right].iter()
                        .all(|n| n > height);
                })
                .map(|(_, height)| *height + 1)
                .sum::<usize>()
        ).sum::<usize>());
}
