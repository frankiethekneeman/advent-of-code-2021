use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

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

fn error<T>(msg: &str) -> Result<T, String> {
    return Err(String::from(msg));
}

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

//I stole this shit from github!  I can't even read it.

type DigitLookup = Vec<(HashSet<char>, usize)>;

const LENGTH_LOOKUPS: [(usize, usize); 4] = [
    (2, 1), //a length of 2 means no 1
    (4, 4),
    (3, 7),
    (7, 8)
];
 
type SolvedSignals<'a> = HashMap<usize, &'a HashSet<char>>;

fn solve(parsed: ParseTarget) -> Result<Solution, String> {

    let outputs = parsed.iter()
        .map(|d| decode(&d.output, &build_lookup(&d.signal_patterns)?))
        .collect::<Result<Vec<usize>, String>>()?;
    return Ok(outputs
        .into_iter()
        .sum::<usize>());
}

fn build_lookup(signals: &Vec<Digit>) -> Result<DigitLookup, String> {
    let l5_lookups: [(usize, Vec<usize>, usize); 3] = [
        (8, vec![4, 7], 2),
        (7, Vec::new(), 3),
        (4, vec![1], 5),
    ];
    
    let l6_lookups: [(usize, Vec<usize>, usize); 3] = [
        (4, vec![1], 0),
        (1, Vec::new(), 6),
        (8, vec![4], 9)
    ];

    let base_sets = lookup_length_based_identities(signals)?;
    let fives = signals.iter()
        .filter(|s| s.wires.len() == 5)
        .map(|s| {
            one(l5_lookups.iter()
                .filter(|(starter, to_remove, _)|
                    build_indicator(&base_sets, starter, to_remove).is_subset(&s.wires)
                ).map(|(_, _, val)| (*val, &s.wires)) 
                .collect()
            )
        }).collect::<Result<SolvedSignals, String>>()?;
    let sixes = signals.iter()
        .filter(|s| s.wires.len() == 6)
        .map(|s| {
            one(l6_lookups.iter()
                .filter(|(starter, to_remove, _)|
                    ! build_indicator(&base_sets, starter, to_remove).is_subset(&s.wires)
                ).map(|(_, _, val)| (*val, &s.wires)) 
                .collect()
            )
        }).collect::<Result<SolvedSignals, String>>()?;

    return Ok(base_sets.into_iter().chain(fives).chain(sixes)
        .map(|(n, set)| (set.clone(), n))
        .collect());

}

fn lookup_length_based_identities(signals: &Vec<Digit>)
    -> Result<SolvedSignals, String>
{
    return LENGTH_LOOKUPS.iter()
        .map(|(length, val)| {
            let found =one(signals
                .iter()
                .filter(|s| s.wires.len() == *length)
                .collect()
            )?;
            return Ok((*val, &found.wires));
        }).collect::<Result<HashMap<usize, &HashSet<char>>, String>>();
}

fn build_indicator(
    base_sets: &HashMap<usize, &HashSet<char>>,
    starter: &usize,
    to_remove: &Vec<usize>
) -> HashSet<char> {
    let base = base_sets.get(starter).unwrap(); //guaranteed to be there
    let removal_set = to_remove.iter()
        .map(|n| base_sets.get(n).unwrap())
        .fold(HashSet::new(), |acc, s| acc.union(s)
            .map(|c| *c)
            .collect()
        );
    return base.difference(&removal_set)
        .map(|c| *c)
        .collect();
}

fn one<T: Clone>(v: Vec<T>) -> Result<T, String> {
    if v.len() == 0 {
        return error("Expected exactly one item, but found none.");
    } else if v.len() > 1 {
        return error("Expected exactly one item, but found many");
    }
    return Ok(v[0].clone())
}


fn decode(nums: &Vec<Digit>, lookup: &DigitLookup) -> Result<usize, String> {
    let visual: String = nums.iter()
        .map(|digit| {
            let result = one(lookup.iter()
            .filter(|(display, _)| *display == digit.wires)
            .collect())?;
            
            return Ok(result.1.to_string())
        }).collect::<Result<String, String>>()?;

    return visual.parse()
        .map_err(|e| format!("Final Parse error: {}", e));
}
