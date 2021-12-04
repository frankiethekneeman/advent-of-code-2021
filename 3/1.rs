use std::fs;
use std::collections::HashMap;

type ParseTarget = Vec<String>;
type Solution = u32;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 198)
];

const DAY: u8 = 3;

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
        .map(String::from)
        .collect());
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    let mut iterator = parsed.into_iter();
    let init_bits = iterator.next().map(as_char_vec);
    return iterator.map(as_char_vec)
        .fold(init_bits,|acc, input|
            acc.map(|bits| bits.iter()
                .zip(input)
                .map(|(running, n)| (*running).clone() + &n)
                .collect()
            )
        ).map(
            |t| t.into_iter().map(get_mode).collect::<String>()
        ).ok_or(String::from("No binary numbers found."))
        .map(|s| (s.clone(), not(s)))
        .and_then(|(gamma, epsilon)| {
            let gamma = u32::from_str_radix(&gamma, 2).map_err(|e| format!("{}", e))?;
            let epsilon = u32::from_str_radix(&epsilon, 2).map_err(|e| format!("{}", e))?;
            return Ok((gamma, epsilon));
        }).map(|(gamma, epsilon)|gamma * epsilon)
        ;
}

fn as_char_vec(string: String) -> Vec<String> {
    return string.chars()
        .map(String::from)
        .collect();
}

fn get_mode(string: String) -> char {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in string.chars() {
        counts.insert(
            c,
            counts.get(&c).unwrap_or(&0) + 1
        );
    }
    return counts.into_iter()
        .fold((' ', 0), |acc, n| if n.1 > acc.1 {n} else {acc})
        .0;
}

fn not(string: String) -> String {
    return string.chars().map(|c| match c {
        '0' => '1',
        '1' => '0',
        _ => panic!("nonbinary string")
    }).collect();
}
