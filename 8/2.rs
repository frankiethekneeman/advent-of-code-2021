use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::Hash;

type ParseTarget = Vec<Display>;
type Solution = usize;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 61229)
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
    signal_patterns: Vec<Digit>,
    output: Vec<Digit>
}

impl Display {
    fn from_string(line: &str) -> Result<Display, String> {
        let pieces: Vec<&str> = line.split(" | ").collect();
        let patterns = pieces.get(0)
            .ok_or(String::from("No LHS"))
            .and_then(|s| Digit::from_string_many(*s))?;

        let out = pieces.get(1)
            .ok_or(String::from("No RHS"))
            .and_then(|s| Digit::from_string_many(*s))?;

        return Ok(Display {
            signal_patterns: patterns,
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
/*
 000
1   2
1   2
 333
4   5
4   5
 666
*/

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$($v,)*]))
    }};
}


fn solve(parsed: ParseTarget) -> Result<Solution, String> {

    let num_patterns: HashMap<usize, HashSet<usize>> = collection!{ //Should be a const
        0 => collection!{0, 1, 2, 4, 5, 6},
    	1 => collection!{2, 5},
        2 => collection!{0, 2, 3, 4, 6},
        3 => collection!{0, 2, 3, 5, 6},
        4 => collection!{1, 2, 3, 5},
        5 => collection!{0, 1, 3, 5, 6},
        6 => collection!{0, 1, 3, 4, 5, 6},
        7 => collection!{0, 2, 5},
        8 => collection!{0, 1, 2, 3, 4, 5, 6},
        9 => collection!{0, 1, 2, 3, 5, 6}
    };

    let configurations = permutations(collection!{'a', 'b', 'c', 'd', 'e', 'f', 'g'});
    
    let lookups: Vec<Vec<(HashSet<char>, usize)>> = configurations.into_iter()
        .map(|c| num_patterns.iter()
            .map(|(n, wires)| 
                (
                    wires.iter().map(|w| c[*w]).collect(),
                    *n
                )
            ).collect()
        ).collect();

    return Ok(parsed.iter()
        .map(|display| (display, lookups.iter()
            .filter(|l| sane(&display.signal_patterns, l))
            .find(|l| sane(&display.output, l))
            .unwrap())
        ).map(|(display, lookup)| decode(&display.output, lookup))
        .sum()
        );
}

fn sane(nums: &Vec<Digit>, lookup: &Vec<(HashSet<char>, usize)>) -> bool {
    return nums.iter()
        .all(|digit| lookup.iter()
            .any(|(display, _)| *display == digit.wires)
        );
}

fn decode(nums: &Vec<Digit>, lookup: &Vec<(HashSet<char>, usize)>) -> usize {
    let visual: String = nums.iter().map(|digit| lookup.iter()
        .find(|(display, _)| *display == digit.wires)
        .map(|(_, val)| format!("{}", val))
        .unwrap()
    ).collect();

    return visual.parse().expect("This can probably never fail.");
}
fn permutations<T>(set: HashSet<T>) -> HashSet<Vec<T>>
    where T: Eq + Hash + Clone
{
    if set.len() == 0 {
        return collection!{Vec::new()};
    }
    //println!("{}", set.len());
    return set.clone().into_iter()
        .flat_map(|i| permutations(
                set.clone().into_iter().filter(|j| i != *j).collect()
            ).into_iter()
            .map(move |perm| {
                let mut result = perm.clone();
                result.push(i.clone());
                return result;
            })
        ).collect();
}

