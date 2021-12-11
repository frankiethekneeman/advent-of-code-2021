use std::fs;

type ParseTarget = Vec<Vec<char>>;
type Solution = usize;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 288957)
];

const DAY: u8 = 10;

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
    return Ok(contents.lines()
        .map(str::chars)
        .map(Iterator::collect)
        .collect()
    );
}

enum Chunk {
    Square,
    Curly,
    Angle,
    Parens
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    let mut scores: Vec<usize> = parsed.iter()
        .map(auto_complete)
        .flat_map(Result::ok)
        .map(score)
        .collect();

    let slice = &mut scores;
    slice.sort();
    return Ok(slice[slice.len()/2]);

}

fn auto_complete(line: &Vec<char>) -> Result<Vec<char>, char> {
    let mut stack = Vec::new();
    for c in line.iter() {
        match c {
            '[' => stack.push(Chunk::Square),
            '{' => stack.push(Chunk::Curly),
            '<' => stack.push(Chunk::Angle),
            '(' => stack.push(Chunk::Parens),
            closer => if stack.pop()
                .map(closing_char)
                .filter(|expected| expected == closer)
                .is_none() {
                    return Err(*closer);
                }
        }
    }

    return Ok(
        stack.into_iter()
            .map(closing_char)
            .rev()
            .collect()
    );
}

fn closing_char(chunk: Chunk) -> char {
    return match chunk {
        Chunk::Curly => '}',
        Chunk::Square => ']',
        Chunk::Angle => '>',
        Chunk::Parens => ')'
    };
}

fn score(completion: Vec<char>) -> usize {
    return completion.iter()
        .map(|c| match *c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Unexpected Completion.")
        }).fold(0, |acc, s| 5 * acc + s);
}
