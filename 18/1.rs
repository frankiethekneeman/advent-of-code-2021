use std::fs;
use std::str::FromStr;
use std::fmt;
use std::ops::Add;

type ParseTarget = Vec<SnailfishNumber>;
type Solution = u64;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 4140)
];

const DAY: u8 = 18;

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

#[derive(Clone)]
enum SnailfishNumber {
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
    Single(u8)
}

impl FromStr for SnailfishNumber {
    type Err = String;
    fn from_str(s: &str) -> Result<SnailfishNumber, String> {
        let mut stack = Vec::new();
        for c in s.chars() {
            if c.is_digit(10) {
                stack.push(SnailfishNumber::Single(
                    c.to_digit(10)
                        .unwrap() as u8
                ))
            } else if c == ']' {
                // Backwards because STACKS
                let rhs = stack.pop().ok_or("Malformed Number".to_string())?;
                let lhs = stack.pop().ok_or("Malformed Number".to_string())?;
                
                stack.push(lhs + rhs);
            }
        }
        if stack.len() == 1 {
            return Ok(stack.pop().unwrap());
        }
        return error("unfinished number");
    }
}


impl fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            SnailfishNumber::Pair(lhs, rhs) => write!(f, "[{}, {}]", lhs, rhs),
            SnailfishNumber::Single(val) => write!(f, "{}", val)
        }
    }
}

fn parse(contents: String) -> Result<ParseTarget, String> {
    return contents.lines()
        .map(FromStr::from_str).collect();

}

impl Add for SnailfishNumber {
    type Output = SnailfishNumber;
    fn add(self, rhs: SnailfishNumber) -> Self::Output {
        return SnailfishNumber::Pair(
            Box::new(self),
            Box::new(rhs)
        );
    }
}

enum ExplodeResult {
    Nothing(), //Nothing exploded
    Done(), //SOmething exploded, but we're done.
    Left(u8), //Explosion, remaining Leftward addition
    Right(u8), //Explosion, remaining Rightward addition
    Explosion(u8, u8) //Explosion
}

impl SnailfishNumber {
    fn explode(self, depth: u8) -> (ExplodeResult, SnailfishNumber) {
        return match self {
            SnailfishNumber::Single(n) => (ExplodeResult::Nothing(), SnailfishNumber::Single(n)),
            SnailfishNumber::Pair(lhs, rhs) => match lhs.explode(depth + 1) {
                (ExplodeResult::Done(), sn) => (ExplodeResult::Done(), sn + *rhs),
                (ExplodeResult::Left(n), sn) => (ExplodeResult::Left(n), sn + *rhs),
                (ExplodeResult::Right(n), sn) => (ExplodeResult::Done(), sn + rhs.add_left(n)),
                (ExplodeResult::Explosion(l, r), sn) => (ExplodeResult::Left(l), sn + rhs.add_left(r)),
                (ExplodeResult::Nothing(), lhs) => match rhs.explode(depth + 1) {
                    (ExplodeResult::Done(), sn) => (ExplodeResult::Done(), lhs + sn),
                    (ExplodeResult::Left(n), sn) => (ExplodeResult::Done(), lhs.add_right(n) + sn),
                    (ExplodeResult::Right(n), sn) => (ExplodeResult::Right(n), lhs + sn),
                    (ExplodeResult::Explosion(l, r), sn) => (ExplodeResult::Right(r), lhs.add_right(l) + sn),
                    (ExplodeResult::Nothing(), rhs) => if depth >= 4 {
                        (
                            ExplodeResult::Explosion(lhs.magnitude() as u8, rhs.magnitude() as u8),
                            SnailfishNumber::Single(0)
                        )
                    } else {
                        (ExplodeResult::Nothing(), lhs + rhs)
                    }
                }
            }
        }
    }
    fn split(self) -> (bool, SnailfishNumber) {
        return match self {
            SnailfishNumber::Single(d) => if d >=10 {
                (true, SnailfishNumber::Single(d/2) + SnailfishNumber::Single(d/2 + (d%2)))
            } else {
                (false, SnailfishNumber::Single(d))
            },
            SnailfishNumber::Pair(lhs, rhs) => match lhs.split() {
                (true, sn) => (true, sn + *rhs),
                (false, lhs) => match rhs.split() {
                    (b, rhs) => (b, lhs + rhs)
                }
            }
        }
    }
    fn reduce(self) -> SnailfishNumber {
        let (result, exploded) = self.explode(0);
        match result {
            ExplodeResult::Nothing() => {
                let (did_work, splitted) = exploded.split();
                if !did_work {
                    return splitted;
                }
                return splitted.reduce();
            },
            _ => { return exploded.reduce(); }
        }
    }
    fn add_right(self, n: u8) -> SnailfishNumber {
        return match self {
            SnailfishNumber::Single(d) => SnailfishNumber::Single(d + n),
            SnailfishNumber::Pair(lhs, rhs) => *lhs + rhs.add_right(n)
        }
    }
    fn add_left(self, n: u8) -> SnailfishNumber {
        return match self {
            SnailfishNumber::Single(d) => SnailfishNumber::Single(d + n),
            SnailfishNumber::Pair(lhs, rhs) => lhs.add_left(n) + *rhs
        }
    }
    fn magnitude(&self) -> u64 {
        return match self {
            SnailfishNumber::Single(d) => *d as u64,
            SnailfishNumber::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude()
        }
    }
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    let mut iter = parsed.iter();
    let mut n = iter.next().ok_or("Must have at least one Number".to_string())?.clone();
    for next in iter {
        n = (n + next.clone()).reduce();
    }
    return Ok(n.magnitude());
}
