use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

type CaveMap = HashMap<String, HashSet<String>>;
type ParseTarget = CaveMap;
type Solution = usize;

const EXAMPLES: [(&str, Solution); 3] = [
    ("1", 10),
    ("2", 19),
    ("3", 226)
];

const DAY: u8 = 12;

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

fn parse(contents: String) -> Result<ParseTarget, String> {
    let mut to_return = HashMap::new();
    for line in contents.lines() {
        if let [lhs, rhs] = &line.split("-").map(String::from).collect::<Vec<String>>()[..] {
            connect(&mut to_return, lhs, rhs);
            connect(&mut to_return, rhs, lhs);
        } else {
            return error("Unparseable line.");
        }
    }
    return Ok(to_return);
}

fn connect(map: &mut CaveMap, start: &String, end: &String) {
    if ! map.contains_key(start) {
        let new = HashSet::new();
        map.insert(start.clone(), new);
    }
    map.get_mut(start)
        .unwrap() // Guaranteed to be there line 79
        .insert(end.clone());
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    return Ok(count_paths_to_end(
        &String::from("start"),
        &(vec!(String::from("start")).into_iter().collect::<HashSet<String>>()),
        &parsed
    ));
}

fn count_paths_to_end(curr: &String, seen: &HashSet<String>, map: &CaveMap) -> usize {
    if curr == &"end" {
        return 1;
    }
    return map.get(curr)
        .unwrap() //Only dangerous for 'start'
        .iter()
        .filter(|s| is_big_cave(s) || !seen.contains(*s))
        .map(|s| copy_and_add(seen, s, |new_seen| count_paths_to_end(s, new_seen, map)))
        .sum::<usize>();
}

fn is_big_cave(name: &String) -> bool {
    return name.chars()
        .all(char::is_uppercase);
}

fn copy_and_add<T, F, R>(set: &HashSet<T>, elem: &T, f: F) -> R
    where T: Clone + Hash + Eq,
        F: FnOnce(&HashSet<T>) -> R
{
    if set.contains(elem) {
        return f(set);
    }
    let mut augmented = set.clone();
    augmented.insert(elem.clone());
    return f(&augmented);
}
