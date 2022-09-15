/**
 *  
 */
use std::fs;
use std::cmp;
use std::ops::Add;

type ParseTarget = (u8, u8);
type Solution = u64;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 444356092776315)
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

#[derive(Copy, Clone)]
struct Player {
    position: u8,
    score: u8
}

impl Player {
    fn new(position: u8) -> Player {
        Player {
            position: position,
            score: 0
        }
    }
    fn advance(&self, count: u8) -> Player {
        let new_position = (self.position + count) % 10;
        return Player {
            position: new_position,
            score: self.score + new_position + 1
        };
    }
}

struct Game {
    p1: Player,
    p2: Player
}

impl Game {
    fn new (p1: u8, p2: u8) -> Game {
        Game {
            p1: Player::new(p1),
            p2: Player::new(p2)
        }
    }

    fn advance(&self, turn: &Turn, count: u8) -> Game {
        return match turn {
            Turn::P1 => Game {
                p1: self.p1.advance(count),
                p2: self.p2
            },
            Turn::P2 => Game {
                p1: self.p1,
                p2: self.p2.advance(count)
            }
        }
    }
    fn detect_win(&self, t: &Turn) -> bool {
        return match t {
            Turn::P1 => self.p2.score >= 21,
            Turn::P2 => self.p1.score >= 21
        }
    }
}

struct Wins {
    p1: u64,
    p2: u64
}

impl Wins {
    fn for_turn(t: Turn) -> Wins {
        return match t {
            Turn::P1 => Wins { p1: 0, p2: 1 },
            Turn::P2 => Wins { p1: 1, p2: 0 }
        };
    }

    fn get_max(&self) -> u64 {
        return cmp::max(self.p1, self.p2);
    }
    
    fn times(&self, scalar: u64) -> Wins {
        return Wins {
            p1: self.p1 * scalar,
            p2: self.p2 * scalar
        }
    }
}

impl Add for Wins {
    type Output = Wins;
    fn add(self, other: Wins) -> Wins {
        return Wins {
            p1: self.p1 + other.p1,
            p2: self.p2 + other.p2
        }
    }
}

enum Turn {
    P1, P2
}

impl Turn {
    fn next (&self) -> Turn {
        return match self {
            Turn::P1 => Turn::P2,
            Turn::P2 => Turn::P1
        }
    }
}


fn solve((p1start, p2start): ParseTarget) -> Result<Solution, String> {
    let game = Game::new(p1start - 1, p2start - 1);
    return Ok(calculate_victories(game, Turn::P1).get_max());
}

fn calculate_victories(game: Game, turn: Turn) -> Wins {
    if game.detect_win(&turn) {
        return Wins::for_turn(turn);
    }
    return calculate_victories(game.advance(&turn, 3), turn.next())
        + calculate_victories(game.advance(&turn, 4), turn.next()).times(3)
        + calculate_victories(game.advance(&turn, 5), turn.next()).times(6)
        + calculate_victories(game.advance(&turn, 6), turn.next()).times(7)
        + calculate_victories(game.advance(&turn, 7), turn.next()).times(6)
        + calculate_victories(game.advance(&turn, 8), turn.next()).times(3)
        + calculate_victories(game.advance(&turn, 9), turn.next())
}
