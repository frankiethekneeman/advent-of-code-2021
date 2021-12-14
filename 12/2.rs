use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

type CaveMap = HashMap<String, HashSet<String>>;
type ParseTarget = CaveMap;
type Solution = usize;

const EXAMPLES: [(&str, Solution); 3] = [
    ("1", 36),
    ("2", 103),
    ("3", 3509)
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
    let start_node = &String::from("start");
    return Ok(count_paths_to_end(
        start_node,
        &Tracker::new(start_node),
        &parsed
    ));
}

struct Tracker<'a> {
    once: HashSet<String>,
    twice: bool,
    start: &'a String
}

impl Tracker<'_> {
    fn new<'a>(start: &'a String) -> Tracker<'a> {
        return Tracker {
            start: start,
            once: HashSet::new(),
            twice: false
        }
    }
    
    fn can_visit(&self, cave: &String) -> bool {
        if is_big_cave(cave) {
            // We can visit big caves as many times as we want
            return true;
        }
        if cave == self.start {
            // We can never go back to start
            return false;
        }
        if !self.once.contains(cave) {
            // We can go to any cave we've never visited
            return true
        }
        // Now we know this is a small, non-start cave, which we've been too.  As long as
        // We've never doubled back, we can go again.
        return !self.twice;
    }

    fn visit_and<F, R>(&self, cave: &String, f: F) -> R
        where F: FnOnce(&Tracker) -> R
    {
        if is_big_cave(cave) {
            // We don't track visits to big caves.
            return f(self);
        }

        let mut augmented = self.once.clone();

        if augmented.contains(cave) {
            if !self.twice {
                return f(&Tracker{
                    start: self.start,
                    once: augmented,
                    twice: true
                });
            } else {
                panic!("I SAID WE COULDN'T GO THERE");
            }
        }

        augmented.insert(cave.clone());
        return f(&Tracker{
            start: self.start,
            once: augmented,
            twice: self.twice
        });
    }
}

fn count_paths_to_end(curr: &String, seen: &Tracker, map: &CaveMap) -> usize {
    if curr == &"end" {
        return 1;
    }
    return map.get(curr)
        .unwrap() //Only dangerous for 'start'
        .iter()
        .filter(|s| seen.can_visit(s))
        .map(|s| seen.visit_and(s, |new_seen| count_paths_to_end(s, new_seen, map)))
        .sum::<usize>();
}

fn is_big_cave(name: &String) -> bool {
    return name.chars()
        .all(char::is_uppercase);
}
