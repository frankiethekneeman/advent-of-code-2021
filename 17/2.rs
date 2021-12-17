use std::fs;
use std::cmp::max;

type ParseTarget = TargetArea;
type Solution = i32;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 112)
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
    x: (i32, i32),
    y: (i32, i32)
}

impl TargetArea {
    fn new(x: (i32, i32), y: (i32, i32)) -> TargetArea {
        return TargetArea {
            x: x,
            y: y
        };
    }
    fn contains(&self, x:i32, y:i32) -> bool {
        return self.x.0 <= x
            && x <= self.x.1
            && self.y.0 <= y
            && y <= self.y.1;
    }
   fn eventually(&self, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    if x > self.x.1 || y < self.y.0 {
        return false;
    }
    if self.contains(x, y) {
        return true;
    }
    return self.eventually(x + dx, y + dy, max(0, dx - 1), dy - 1);
    
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
    let min_x = smallest_triangle_index(parsed.x.0);
    let max_x = parsed.x.1;
    let min_y = parsed.y.0;
    let max_y = -min_y + 1;
    let mut count = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if parsed.eventually(0, 0, x, y) {
                count = count + 1;
            }
        }
    }
    return Ok(count);
}

fn triangle(n: i32) -> i32 {
    return n * (n + 1) / 2;
}

fn smallest_triangle_index(n: i32) -> i32 {
    let base = ((2*n) as f64).sqrt() as i32;
    if triangle(base) < n {
        return base + 1;
    } else {
        return base;
    }
}
