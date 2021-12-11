use std::fs;

type ParseTarget = Vec<Vec<usize>>;
type Solution = usize;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 1656)
];

const DAY: u8 = 11;

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
    return contents.lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(10)
                .map(|d| d as usize)
                .ok_or(format!("Illegal character: {}", c))
            ).collect()
        ).collect();
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    let mut octopuses = parsed.clone();
    let mut flashes = 0;
    for _ in 0..100 {
        charge(&mut octopuses);
        flashes += flash(&mut octopuses);
        reset(&mut octopuses);
    }

    return Ok(flashes);
}

fn charge(octopodes: &mut Vec<Vec<usize>>) {
    for x in 0..octopodes.len() {
        for y in 0..octopodes[x].len() {
            octopodes[x][y] = octopodes[x][y] + 1;
        }
    }
}

fn reset(cephalopods: &mut Vec<Vec<usize>>) {
    for x in 0..cephalopods.len() {
        for y in 0..cephalopods[x].len() {
            if cephalopods[x][y] == 10 {
                cephalopods[x][y] = 0;
            }
        }
    }
}

fn flash(octopi: &mut Vec<Vec<usize>>) -> usize {
    let mut flashes: Vec<(usize, usize)> = octopi.iter().enumerate()
        .flat_map(|(x, r)| r.iter().enumerate()
            .flat_map(move |(y, v)| if v == &10 {
                    Some((x, y))
                } else {
                    None
                })
        ).collect();
    let mut flasher = 0;
    while flasher < flashes.len() {
        let (x, y) = flashes[flasher];
        for (nx, ny) in neighbors(x, y).iter() {
            let nval = octopi.get(*nx)
                .and_then(|row| row.get(*ny));
            match nval {
                Some(10) | None => (), //Already flashed, or out of bounds.
                Some(v) => {
                    if v == &9 { //meaning v + 1 = 10
                        flashes.push((*nx, *ny));
                    }
                    octopi[*nx][*ny] = v + 1;
                }
            }
        }
        flasher += 1;
    }
    return flasher;
}

fn neighbors(x: usize, y:usize) -> Vec<(usize, usize)> {
    return (-1..=1).flat_map(
        |dx| (-1..=1)
            .map(move |dy| (dx, dy))
    ).filter(|t| t != &(0, 0))
    .flat_map(|(dx, dy)| tup (
        if dx == -1 { x.checked_sub(1) } else { Some(x + (dx as usize)) },
        if dy == -1 { y.checked_sub(1) } else { Some(y + (dy as usize)) }
    )).collect();
}

fn tup<T>(x: Option<T>, y: Option<T>) -> Option<(T, T)> {
    return Some((x?, y?));
}
