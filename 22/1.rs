use std::fs;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;

type ParseTarget = Vec<Command>;
type Solution = usize;

const EXAMPLES: [(&str, Solution); 2] = [
    ("1", 39),
    ("2", 590784)
];

const DAY: u8 = 22;

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

// fn error<T>(msg: &str) -> Result<T, String> {
//     return Err(String::from(msg));
// }

fn operation(filename: String) -> Result<Solution, String> {
    return fs::read_to_string(filename)
        .map_err(|io_error| format!("{}", io_error))
        .and_then(parse)
        .and_then(solve);
}

struct Command {
    on: bool,
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize)
}

impl Command {
    fn from_input(line: &str) -> Result<Command, String> {
        let mut bits = line.split(&[' ', ','][..]);
        let on = bits.next()
            .ok_or(String::from("No on/off"))
            .and_then(|on| match on {
                "on" => Ok(true),
                "off" => Ok(false),
                other => Err(format!("Unrecognized on/off: '{}'", other))
            })?;
        let x = bits.next()
            .ok_or(String::from("insufficient segments for x range."))
            .and_then(|s| parse_range("x=", s))?;
        let y = bits.next()
            .ok_or(String::from("insufficient segments for y range."))
            .and_then(|s| parse_range("y=", s))?;
        let z = bits.next()
            .ok_or(String::from("insufficient segments for z range."))
            .and_then(|s| parse_range("z=", s))?;
        return Ok(Command {
            on: on,
            x: x,
            y: y,
            z: z
        })
    }

    fn points(&self, from: isize, to: isize) -> HashSet<(isize, isize, isize)> {
        if self.x.0 > to || self.x.1 < from || self.y.0 > to || self.y.1 < from || self.z.0 > to || self.z.1 < from {
            return HashSet::new()
        }
        return (max(self.x.0, from)..=min(self.x.1, to)).into_iter().flat_map(|x|
            (max(self.y.0, from)..=min(self.y.1, to)).into_iter().flat_map(move |y|
                (max(self.z.0, from)..=min(self.z.1, to)).into_iter().map(move |z|
                    (x, y, z)
                 )
            )
        )
        .collect();
    }
}

fn parse_range(prefix: &str, line: &str) -> Result<(isize, isize), String> {
    let bounds = line.strip_prefix(prefix)
        .ok_or(format!("Expected prefix '{}' on '{}'", prefix, line))
        .and_then(|rest| rest.split("..")
            .map(str::parse)
            .collect::<Result<Vec<isize>, _>>()
            .map_err(|e| format!("{}", e))
        )?;

    if bounds.len() == 2 {
        return Ok((bounds[0], bounds[1]))
    }

    return Err(format!("Got the wrong number of bounds from '{}'.", line));
}

fn parse(contents: String) -> Result<ParseTarget, String> {
    return contents.lines()
        .map(Command::from_input)
        .collect();
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    return Ok(parsed.into_iter().fold(HashSet::new(), |points: HashSet<(isize, isize, isize)>, command| {
        let to_change = command.points(-50, 50);
        if command.on {
            return &points | &to_change;
        } else {
            return &points - &to_change;
        }
    }).len());
}
