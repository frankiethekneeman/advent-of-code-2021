use std::fs;

type ParseTarget = TargetArea;
type Solution = i32;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 45)
];

const DAY: u8 = 17;

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

struct TargetArea {
    _x: (i32, i32),
    y: (i32, i32)
}

impl TargetArea {
    fn new(x: (i32, i32), y: (i32, i32)) -> TargetArea {
        return TargetArea {
            _x: x,
            y: y
        };
    }
}

fn parse(contents: String) -> Result<ParseTarget, String> {
    let ranges_only = &contents.trim()[15..];
    let mut range_iterator = ranges_only.split(", y=")
        .map(to_range);
    let x = range_iterator.next()
        .ok_or("No X range".to_string())
        ??;
    let y = range_iterator.next()
        .ok_or("No y range".to_string())
        ??;
    return Ok(TargetArea::new(x, y));
}

fn to_range(contents: &str) -> Result<(i32, i32), String> {
    let mut it = contents.split("..")
        .map(|s| i32::from_str_radix(s, 10)
            .map_err(|e| format!("Number Parse Error: {}, {}", e, s.clone()))
        );
    let bottom = it.next()
        .ok_or("No numbers found".to_string())
        ??;
    let top = it.next()
        .ok_or("Only one number found".to_string())
        ??;

    return Ok((bottom, top));
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    let y_velocity = parsed.y.0.abs() - 1;
    return Ok(triangle(y_velocity));
}

fn triangle(n: i32) -> i32 {
    return n * (n + 1) / 2;
}
