use std::fs;
use std::collections::HashMap;

type ParseTarget = (Vec<char>, HashMap<(char, char), char>);
type Solution = u128;

const EXAMPLES: [(&str, u8, Solution); 3] = [
    ("1.ie", 10, 1588),
    ("input", 10, 2068),
    ("1.ie", 40, 2188189693529)
];

const DAY: u8 = 14;

fn main() {
    let results = EXAMPLES.iter()
        .zip(
            EXAMPLES.iter()
                .map(|t| (t.1, format!("{}/{}", DAY, t.0)))
                .map(|(n, f)| operation(n, f))
        )
        .map(|((name, n, expected), result)| 
            (
                format!("{} ({} substitutions)", *name, n),
                result
                    .and_then(|actual| if *expected == actual {
                        return Ok(());
                    } else {
                        return Err(format!("Expected {} but got {}", expected, actual));
                    })
            )
        )
        .collect::<Vec<(String, Result<(), String>)>>();
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
        operation(40, format!("{}/input", DAY)).expect("Unexpected Error in main input.")
    );
}

fn error<T>(msg: &str) -> Result<T, String> {
    return Err(String::from(msg));
}

fn operation(n: u8, filename: String) -> Result<Solution, String> {
    return fs::read_to_string(filename)
        .map_err(|io_error| format!("{}", io_error))
        .and_then(parse)
        .and_then(|p| solve(n, p));
}

fn parse(contents: String) -> Result<ParseTarget, String> {
    let mut lines = contents.lines();
    let template: Vec<char> = lines.next()
        .ok_or("No lines?")?
        .chars()
        .collect();

    if lines.next().is_none() {
        return error("No empty line");
    }
    
    let mut insertion_rules = HashMap::new();

    for l in lines {
        let first = l.chars().nth(0).ok_or("Malformed line".to_string())?;
        let second = l.chars().nth(1).ok_or("Malformed line".to_string())?;
        let insertion = l.chars().nth(6).ok_or("Malformed line".to_string())?;

        insertion_rules.insert(
            (first, second),
            insertion
        );
    }
        
    return Ok((template, insertion_rules));
}
enum Twople<T> {
    Pair(T, T),
    Last(T)
}

struct TwopleWindows<'a, T: Copy, I: Iterator<Item=T>> {
    done: bool,
    prev: Option<T>,
    backer: &'a mut I
}

impl<'a, T: Copy, I: Iterator<Item=T>> TwopleWindows<'a, T, I> {
    fn new(backer: &'a mut I) -> TwopleWindows<'a, T, I> {
        return TwopleWindows {
            done: false,
            prev: None,
            backer: backer
        };
    }
}

impl<'a, T: Copy, I: Iterator<Item=T>> Iterator for TwopleWindows<'a, T, I> {
    type Item = Twople<T>;
    fn next(&mut self) -> Option<Twople<T>> {
        if self.done {
            return None;
        }
        let first = self.prev.or_else(|| self.backer.next())?;
        match self.backer.next() {
            Some(second) => {
                self.prev = Some(second);
                return Some(Twople::Pair(first, second));
            }
            None => {
                self.done = true;
                return Some(Twople::Last(first))
            }
        }
    }
}

struct Expansions {
    memo: HashMap<(char, char, u8), HashMap<char, u128>>,
    substitutions: HashMap<(char, char), char>
}

impl Expansions {
    fn new(substitutions: HashMap<(char, char), char>) -> Expansions {
        return Expansions {
            memo: HashMap::new(),
            substitutions: substitutions
        };
    }
    fn injections(&mut self, l: char, r: char, n: u8) -> HashMap<char, u128> {
        let key = &(l, r, n);
        if !self.memo.contains_key(key) {
            if n == 0 || !self.substitutions.contains_key(&(l, r)) {
                self.memo.insert(
                    *key,
                    HashMap::new()
                );
            } else {
                let i = *self.substitutions.get(&(l, r)).unwrap();
                let lhs = self.injections(l, i, n - 1);
                let rhs = self.injections(i, r, n - 1);
                let mut new_map = combine(lhs, rhs);
                new_map.insert(
                    i,
                    *new_map.get(&i).unwrap_or(&0) + 1
                );
                self.memo.insert(
                    *key,
                    new_map
                );
            }
        }
        
        return self.memo.get(key).unwrap().clone();
    }
}

fn combine(lhs: HashMap<char, u128>, rhs: HashMap<char, u128>) -> HashMap<char, u128>{
    return lhs.keys()
        .chain(rhs.keys())
        .cloned()
        .collect::<std::collections::HashSet<char>>()
        .into_iter()
        .map(|k| (k, *lhs.get(&k).unwrap_or(&0) + *rhs.get(&k).unwrap_or(&0)))
        .collect();
}

fn solve(n: u8, parsed: ParseTarget) -> Result<Solution, String> {
    let (template, rules) = parsed;
    let mut expansions = Expansions::new(rules);
    let mut counts = TwopleWindows::new(&mut template.iter()).fold(HashMap::new(), |counts, t|
        match t {
            Twople::Pair(l, r) => combine(counts, expansions.injections(*l, *r, n)),
            Twople::Last(_) => counts
        }
    );
    for c in template.into_iter() {
        counts.insert(
            c,
            counts.get(&c).unwrap_or(&0) + 1
        );
    }
    let max = counts.values().max().ok_or("No Maximum?")?;
    let min = counts.values().min().ok_or("No Minimum?")?;
    return Ok(*max - *min);
}
