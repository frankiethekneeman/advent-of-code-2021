use std::fs;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::cmp::Reverse;

type ParseTarget = Vec<Vec<u32>>;
type Solution = u32;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 315)
];

const DAY: u8 = 15;

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
    let base_tile = contents.lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).ok_or(format!("{} is not a digit.", c)))
            .collect::<Result<Vec<u32>, String>>()
        ).collect::<Result<Vec<Vec<u32>>, String>>()?;

    let base_row = base_tile.into_iter()
        .map(|l| (0..=4).flat_map(|offset|
            l.iter().map(move |r| inc_risk(*r, offset)).collect::<Vec<u32>>()
        ).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    return Ok((0..=4).flat_map(move |offset|
        base_row.iter().map(move |l|
            l.iter().map(move |r| inc_risk(*r, offset)).collect::<Vec<u32>>()
        ).collect::<Vec<Vec<u32>>>()
    ).collect())
}

fn inc_risk(risk: u32, offset: u32) -> u32 {
    let new = risk + offset;
    if new > 9 {
        return new - 9;
    } else {
        return new;
    }
}

struct Position {
    loc: (usize, usize),
    risk: u32,
    target: (usize, usize)
}

impl Position {
    fn new(loc: (usize, usize), risk: u32, target: (usize, usize)) -> Position {
        return Position {
            loc:loc, risk: risk, target: target
        };
    }
    fn min_remaining(&self) -> usize{
        let (tx, ty) = self.target;
        let (x, y) = self.loc;

        return (tx - x) + (ty - y);
    }
    fn up(&self, costs: &Vec<Vec<u32>>) -> Position {
        let (old_y, x) = self.loc;
        let y = old_y - 1;

        return Position::new((y, x), self.risk + costs[y][x], self.target);
    }
    fn down(&self, costs: &Vec<Vec<u32>>) -> Position {
        let (old_y, x) = self.loc;
        let y = old_y + 1;

        return Position::new((y, x), self.risk + costs[y][x], self.target);
    }
    fn left(&self, costs: &Vec<Vec<u32>>) -> Position {
        let (y, old_x) = self.loc;
        let x = old_x - 1;

        return Position::new((y, x), self.risk + costs[y][x], self.target);
    }
    fn right(&self, costs: &Vec<Vec<u32>>) -> Position {
        let (y, old_x) = self.loc;
        let x = old_x + 1;

        return Position::new((y, x), self.risk + costs[y][x], self.target);
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        return (Reverse(self.risk), Reverse(self.min_remaining()))
            .cmp(&(Reverse(other.risk), (Reverse(other.min_remaining()))))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Position {
    fn  eq(&self, other: &Self) -> bool {
        return self.risk == other.risk && self.min_remaining() == other.min_remaining();
    }
}

impl Eq for Position {}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    let target_y = parsed.len() - 1;
    let target_x = parsed[target_y].len() - 1;

    let start = Position::new((0, 0), 0, (target_y, target_x));
    let mut positions = BinaryHeap::new();
    let mut seen = HashSet::new();
    positions.push(start);

    while let Some(curr) = positions.pop() {
        if curr.min_remaining() == 0 {
            return Ok(curr.risk);
        }
        // Risk is monotonically increasing, so if we've seen a position before, it's because 
        // there's a way to get there with lower (or equivalent) risk.
        if !seen.insert(curr.loc) {
            continue;
        }
        if curr.loc.0 > 0 {
            positions.push(curr.up(&parsed));
        }
        if curr.loc.0 < target_y {
            positions.push(curr.down(&parsed));
        }
        if curr.loc.1 > 0 {
            positions.push(curr.left(&parsed));
        }
        if curr.loc.1 < target_x {
            positions.push(curr.right(&parsed));
        }
    }
    
    return error("Could Not find a Path");
}
