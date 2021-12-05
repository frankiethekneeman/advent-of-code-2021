use std::fs;
use std::cmp;
use std::collections::HashMap;

type ParseTarget = Vec<Line>;
type Solution = usize;

const EXAMPLES: [(&str, Solution); 3] = [
    ("1", 12),
    ("2", 0),
    ("3", 1)
];

const DAY: u8 = 5;

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

struct Line {
    start: (i32, i32),
    end: (i32, i32)
}

impl Line {
    fn points(&self) -> Vec<(i32, i32)> {
        if self.start.0 == self.end.0 {
            return closed_range(self.start.1, self.end.1)
                .into_iter()
                .map(|y| (self.start.0, y))
                .collect();
        } else if self.start.1 == self.end.1 {
            return closed_range(self.start.0, self.end.0)
                .into_iter()
                .map(|x| (x, self.start.1))
                .collect();
        } else {
            let xs = closed_range(self.start.0, self.end.0);
            let ys = closed_range(self.start.1, self.end.1);
            return xs.into_iter().zip(ys.into_iter()).collect();
        }
    }
    fn from_str(s: &str) -> Result<Line, String> {
        let mut tuples = s.split(" -> ");
        let lhs = tuples.next().ok_or(format!("No LHS? {}", s))?;
        let rhs = tuples.next().ok_or(format!("No RHS: {}", s))?;

        return Ok(Line {
            start: get_tuple(lhs)?,
            end: get_tuple(rhs)?
        });
        
    }
}

fn get_tuple(s: &str) -> Result<(i32, i32), String> {
    let pieces: Vec<i32> = s.split(",")
        .map(str::parse)
        .collect::<Result<Vec<i32>,_>>()
        .map_err(|e| format!("Parse Error: {}", e))?;
    return Ok((pieces[0], pieces[1]))
}

fn closed_range(left: i32, right: i32) -> Vec<i32> {
    let base = cmp::min(left, right)..=cmp::max(left, right);
    if left < right {
        return base.collect();
    } else {
        return base.rev().collect();
    }
}

fn parse(contents: String) -> Result<ParseTarget, String> {
    return contents.lines()
        .map(Line::from_str)
        .collect();
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();
    for point in parsed.iter().flat_map(Line::points) {
        points.insert(
            point,
            *points.get(&point).unwrap_or(&0) + 1
        );
    }

    return Ok(points.iter()
        .filter(|(_,v)| **v >= 2)
        .count());
}
