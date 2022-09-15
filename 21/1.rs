use std::fs;

type ParseTarget = (usize, usize);
type Solution = usize;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 739785)
];

const DAY: u8 = 21;

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

fn parse(contents: String) -> Result<ParseTarget, String> {
    let mut lines = contents.lines();
    let p1start = lines.next()
        .and_then(|l| l.strip_prefix("Player 1 starting position: "))
        .ok_or(String::from("Malformed first line"))
        .and_then(|p| p.parse().map_err(|e| format!("{}", e)))?;
    let p2start = lines.next()
        .and_then(|l| l.strip_prefix("Player 2 starting position: "))
        .ok_or(String::from("Malformed second line"))
        .and_then(|p| p.parse().map_err(|e| format!("{}", e)))?;

    return Ok((p1start, p2start))
}

struct Player {
    position: usize,
    score: usize
}

impl Player {
    fn new(position: usize) -> Player {
        Player {
            position: position,
            score: 0
        }
    }
}

struct DeterministicDie {
    requests: usize,
    min: usize,
    ring_size: usize
}

impl DeterministicDie {
    fn new(min: usize, max: usize) -> DeterministicDie {
        return DeterministicDie {
            requests: 0,
            min: min,
            ring_size: max - min + 1
        }
    }
}
impl Iterator for DeterministicDie {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let result = Some( self.min + (self.requests % self.ring_size) );
        self.requests += 1;
        return result;
    }
}
fn solve((p1start, p2start): ParseTarget) -> Result<Solution, String> {
    let mut p1 = Player::new(p1start - 1);
    let mut p2 = Player::new(p2start - 1);
    let mut die = DeterministicDie::new(1, 100);
    loop {
        turn(&mut p1, &mut die);
        if p1.score >= 1000 {
            return Ok(p2.score * die.requests);
        }
        turn(&mut p2, &mut die);
        if p2.score >= 1000 {
            return Ok(p1.score * die.requests);
        }

    }
}

fn turn<I>(player: &mut Player, die: &mut I) 
where
    I: Iterator<Item = usize>
{
    player.position += die.next().unwrap() + die.next().unwrap() + die.next().unwrap();
    player.position %= 10;
    player.score += player.position + 1;
}

