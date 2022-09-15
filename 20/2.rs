use std::fs;
use std::convert::TryFrom;

type ParseTarget = ([bool; 512], EnhanceableImage);
type Solution = usize;

const EXAMPLES: [(&str, Solution); 1] = [
    ("1", 3351)
];

const DAY: u8 = 20;

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

struct EnhanceableImage {
    rows: usize,
    columns: usize,
    default: bool,
    pixels: Vec<Vec<bool>>
}

impl EnhanceableImage {
    fn new(pixels: Vec<Vec<bool>>, default: bool) -> Result<EnhanceableImage, String> {
        let min_row_size = pixels.iter().map(|r| r.len()).min().unwrap_or(0);
        let max_row_size = pixels.iter().map(|r| r.len()).max().unwrap_or(0);
        if min_row_size != max_row_size {
            return error("Uneven rows - probably a parse error.");
        }
        return Ok(EnhanceableImage {
            rows: pixels.len(),
            columns: min_row_size,
            default: default,
            pixels: pixels
        });
    }
    fn is_lit(&self, x: isize, y: isize) -> bool {
        let res = match (usize::try_from(x), usize::try_from(y)) {
            (Ok(ux), Ok(uy)) if ux < self.columns && uy < self.rows => self.pixels[uy][ux],
            _ => self.default
        };
        return res;
    }

    fn calculate_enhancement_index(&self, x: isize, y: isize) -> usize {
        return if self.is_lit(x - 1, y - 1) { 256 } else { 0 }
             + if self.is_lit(x    , y - 1) { 128 } else { 0 }
             + if self.is_lit(x + 1, y - 1) {  64 } else { 0 }
             + if self.is_lit(x - 1, y    ) {  32 } else { 0 }
             + if self.is_lit(x    , y    ) {  16 } else { 0 }
             + if self.is_lit(x + 1, y    ) {   8 } else { 0 }
             + if self.is_lit(x - 1, y + 1) {   4 } else { 0 }
             + if self.is_lit(x    , y + 1) {   2 } else { 0 }
             + if self.is_lit(x + 1, y + 1) {   1 } else { 0 };
    }
    
    fn count_pixels(&self) -> usize {
        return self.pixels.iter()
            .map(|r| r.iter().filter(|&&b| b).count())
            .sum()
    }
}

fn parse(contents: String) -> Result<ParseTarget, String> {
    let mut lines = contents.lines();
    let algo = lines.next()
        .ok_or(String::from("No lines in Input."))
        .and_then(parse_algorithm)?;
    assert_eq!(lines.next(), Some(""));
    let pixels = lines
        .map(|l| l.chars().map(parse_pixel).collect())
        .collect::<Result<Vec<Vec<bool>>, String>>()?;
    let img = EnhanceableImage::new(pixels, false)?;
    return Ok((algo,img));
}

fn parse_algorithm(line: &str) -> Result<[bool; 512], String> {
    let mut algo = [false; 512];
    let mut chars = line.chars();
    for i in 0..512 {
        algo[i] = chars.next()
            .ok_or(String::from("Not enough characters in line."))
            .and_then(parse_pixel)?;
    }

    return Ok(algo)
}

fn parse_pixel(c: char) -> Result<bool, String> {
    return match c {
        '#' => Ok(true),
        '.' => Ok(false),
        _ => Err(format!("Unrecognized pixel value: '{}'", c.to_string())),
    }
}

fn solve((algo, base_image): ParseTarget) -> Result<Solution, String> {
    let mut image = base_image;
    for _ in 0..50 {
        image = enhance_image(&image, &algo)?;
    }
    return Ok(image.count_pixels())
}

fn enhance_image(pre: &EnhanceableImage, algo: &[bool; 512]) -> Result<EnhanceableImage, String> {
    let new_rows = isize::try_from(pre.rows).map_err(|e| format!("{}", e))?; 
    let new_columns = isize::try_from(pre.columns).map_err(|e| format!("{}", e))?; 
    let new_pixels = (-1..=new_rows)
        .map(|y| (-1..=new_columns).map(|x| {
                let idx = pre.calculate_enhancement_index(x, y);
                let val = algo[idx];
                return val;
            }).collect())
        .collect();
    return EnhanceableImage::new(new_pixels, algo[pre.calculate_enhancement_index(-2, -2)]);
}
