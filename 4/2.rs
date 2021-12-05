use std::fs;
use std::collections::HashSet;

type ParseTarget = (Vec<i32>, Vec<Board>);
type Solution = i32;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 1924)
];

const DAY: u8 = 4;

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

trait Bingo {
	fn has_bingo(&self, called: &HashSet<i32>) -> bool;
	fn score(&self, last: i32, called: &HashSet<i32>) -> i32;
}
struct Board {
	nums: Vec<Vec<i32>> // Feels like it should be 5x5?
}

impl Board {
    fn from_strs(strs: &[&str]) -> Result<Board, String> {
        let nums = strs.iter()
            .map(|s| s.trim()
                .split_whitespace()
                .map(str::parse)
                .collect()
            ).collect::<Result<Vec<Vec<i32>>,_>>()
            .map_err(|e| format!("Error Parsing Board: {}", e))?;

        return Ok(Board {
            nums: nums
        })
    }
}

impl Bingo for Board {
	fn has_bingo(&self, called: &HashSet<i32>) -> bool {
        let matching_row = self.nums.iter()
            .any(|row| row.iter().all(|n| called.contains(n)));

        return matching_row || transpose(&self.nums).iter()
            .any(|col| col.iter().all(|n| called.contains(n)));
	}

	fn score(&self, last: i32, called: &HashSet<i32>) -> i32 {
        let mut points = 0;
        for row in &(self.nums) {
            for n in row {
                if !called.contains(n) {
                    points = points + n;
                }
            }
        }
        return points * last;
	}
}

fn transpose<T>(input: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone
{
    return (0..(input[0].len()))
        .map(|idx| input.iter()
            .map(|row| row[idx].clone())
            .collect()
        ).collect()
}

fn parse(contents: String) -> Result<ParseTarget, String> {
    let lines: Vec<&str> = contents.lines().collect();
    let draw_order: Vec<i32> = lines[0].split(",")
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()
        .map_err(|e| format!("Error parsing calls: {}", e))?;

    let mut boards: Vec<Board> = Vec::new();

    for start in (2..lines.len()).step_by(6) {
        boards.push(Board::from_strs(&lines[start..start+5])?)
    }

    return Ok((draw_order, boards))
}

fn solve((calls, mut boards): ParseTarget) -> Result<Solution, String> {
    let mut called = HashSet::new();
    for call in calls {
        called.insert(call);
        if boards.len() == 1 && boards[0].has_bingo(&called) {
            return Ok(boards[0].score(call, &called))
        } else {
            boards.retain(|b| !b.has_bingo(&called));
        }
    }
    return error("No last winner");
}

