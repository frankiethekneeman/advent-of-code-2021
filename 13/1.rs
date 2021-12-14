use std::fs;
use std::collections::HashSet;

type ParseTarget = Page;
type Solution = usize;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 17)
];

const DAY: u8 = 13;

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

struct Page {
    points: HashSet<(usize, usize)>,
    lines: Vec<Line>
}

enum Line {
    X(usize),
    Y(usize)
}

fn parse(contents: String) -> Result<ParseTarget, String> {
    let mut points = HashSet::new();
    let mut lines = Vec::new();
    let mut input = contents.lines();

    loop {
        match input.next() {
            Some("") => break,
            Some(line) => {
                points.insert(parse_point(line)?);
            }
            None => return error("Unexpected end of input.")
        }
    }
    for line in input {
        lines.push(parse_fold_line(line)?);
        break; //Only the first part for now.
    }
    return Ok(Page{
        points: points,
        lines: lines
    });
}

fn parse_point(line: &str) -> Result<(usize, usize), String> {
    let parsed = line.split(",")
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()
        .map_err(|e| format!("{}", e))?;
    if let [x, y] = &parsed[..] {
        return Ok((*x, *y));
    }
    return error("Bad Point Parse");
}

fn parse_fold_line(line: &str) -> Result<Line, String> {
    let equation = &line[11..];
    let dir = equation.chars().nth(0);
    let val = equation[2..].parse()
        .map_err(|e| format!("{}", e))?;
    return match dir {
        Some('x') => Ok(Line::X(val)),
        Some('y') => Ok(Line::Y(val)),
        _ => error("Invalid equation")
    }
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    return Ok(parsed.lines
        .into_iter()
        .fold(parsed.points, fold_line)
        .len())
}

fn fold_line(points: HashSet<(usize, usize)>, line: Line) -> HashSet<(usize, usize)> {
    return points.into_iter()
        .map(|(x, y)| match line {
            Line::X(axis) => (if axis > x { x } else { 2 * axis - x}, y),
            Line::Y(axis) => (x, if axis > y { y } else { 2 * axis - y}),
        }).collect();
}
