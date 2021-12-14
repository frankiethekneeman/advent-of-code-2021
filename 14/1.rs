use std::fs;
use std::collections::HashMap;

type ParseTarget = (Vec<char>, HashMap<(char, char), char>);
type Solution = usize;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 1588)
];

const DAY: u8 = 14;

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

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    let (template, rules) = parsed;
    let result = inject(&mut template.into_iter(), &rules, 10);
    let mut counts: HashMap<char, usize> = HashMap::new(); 
    for c in result.iter() {
        counts.insert(
            *c,
            *counts.get(c).unwrap_or(&0) + 1
        );
    }
    println!("Done?");
    return error("Solve Not Yet Implemented");
}

fn inject<T>(elements: &mut T, rules: &HashMap<(char, char), char>, n: u8) -> Vec<char>
where T: Iterator<Item = char>
{
    if n == 0 {
        return elements.collect();
    }
    let mut iter = TwopleWindows::new(elements)
        .flat_map(|t| match t {
            Twople::Pair(l, r) => match rules.get(&(l, r)) {
                Some(i) => vec![l, *i],
                None => vec![l]
            },
            Twople::Last(last) => vec![last]
        });

    return inject(&mut iter, rules, n - 1);
}
