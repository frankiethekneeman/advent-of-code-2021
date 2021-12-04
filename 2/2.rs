use std::fs;

type ParseTarget = Vec<Instruction>;
type Solution = i32;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 900)
];

const DAY: u8 = 2;

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

    println!("{}", operation(format!("{}/input", DAY)).expect("Unexpected Error in main input."));
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

enum Instruction {
    Forward(u8),
    Down(u8),
    Up(u8)
}

fn parse(contents: String) -> Result<ParseTarget, String> {
    return contents.lines()
        .map(|line| if line.starts_with("forward ") {
            line.get(8..)
                .map(|n| n.parse().map(Instruction::Forward))
        } else if line.starts_with("down ") {
            line.get(5..)
                .map(|n| n.parse().map(Instruction::Down))
        } else if line.starts_with("up ") {
            line.get(3..)
                .map(|n| n.parse().map(Instruction::Up))
        } else {
            None
        })
        .map(|o| o.map(|r| r.map_err(|e| format!("{}", e)))
            .unwrap_or(error("Unrecognized Line"))
        )
        .collect();
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    let final_position = parsed.iter()
        .fold(Position {horizontal: 0, depth: 0, aim: 0}, |pos, ins| match ins {
            Instruction::Forward(n) => Position {
                horizontal: pos.horizontal + (*n as i32),
                depth: pos.depth + (pos.aim * (*n as i32)),
                aim: pos.aim
            },
            Instruction::Down(n) => Position {
                horizontal: pos.horizontal,
                depth: pos.depth,
                aim: pos.aim + (*n as i32)
            },
            Instruction::Up(n) => Position {
                horizontal: pos.horizontal,
                depth: pos.depth,
                aim: pos.aim - (*n as i32)
            }
        });
    return Ok(final_position.horizontal * final_position.depth);
}
