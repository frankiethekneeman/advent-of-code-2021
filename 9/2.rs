use std::fs;
use std::convert::TryFrom;

type ParseTarget = Vec<Vec<usize>>;
type Solution = usize;

const EXAMPLES: [(&str, Solution); 3] = [
    ("1", 1134),
    ("2", 7),
    ("3", 11)
];

const DAY: u8 = 9;

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
        .into_iter()
        .map(str::chars)
        .map(|row| row.map(|c| c.to_digit(10)
            .map(usize::try_from)
            .unwrap()
            .map_err(|e| format!("{}", e))
        ))
        .map(Iterator::collect)
        .collect();
}

type Chunks = Vec<(usize, usize)>;
struct Basin {
    chunks: Chunks,
    leading_edge: Chunks
}

impl Basin {
    fn new(chunk: (usize, usize)) -> Basin {
        return Basin {
            chunks: Vec::new(),
            leading_edge: vec!(chunk)
        };
    }
    
    fn empty() -> Basin {
        return Basin {
            chunks: Vec::new(),
            leading_edge: Vec::new()
        }
    }

    fn extend(&mut self, chunks: Chunks) -> Chunks {
        if self.leading_edge.len() == 0 {
            return chunks;
        }
        let (new_edge, rest): (Chunks, Chunks) = chunks.iter()
            .partition(|t| self.matches(t));
        self.chunks.append(&mut self.leading_edge);
        self.leading_edge = new_edge;
        return rest;
    }

    fn matches(&self, (candidate_start, candidate_end): &(usize, usize)) -> bool {
         return self.leading_edge.iter()
             .any(|(edge_start, edge_end)|
                 (edge_start <= candidate_start && candidate_start <= edge_end)
                 || (edge_start <= candidate_end && candidate_end <= edge_end)
                 || (candidate_start <= edge_start && edge_end <= candidate_end)
              )
    }

    fn close(&mut self) {
        self.chunks.append(&mut self.leading_edge);
        self.leading_edge = Vec::new();
    }
    fn combine(&mut self, that: &mut Basin) {
        self.chunks.append(&mut that.chunks);
        self.leading_edge.append(&mut that.leading_edge);
    }

    fn size(&self) -> usize {
        return self.chunks.iter()
            .map(|(s, e)| e - s + 1)
            .sum()
    }
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    let intervals = parsed.iter()
        .map(|v| [9].iter()
            .chain(v)
            .chain([&9])
            .cloned()
            .collect::<Vec<usize>>())
        .map(to_intervals);
    let mut basins: Vec<Basin> = Vec::new();

    for row in intervals {
        let multi_matching_chunks: Chunks = row.iter()
            .filter(|t| basins.iter().filter(|b| b.matches(t)).count() > 1)
            .cloned()
            .collect();
        //Combine any basins that match multiple chunks - they were secretly the same basin.
        for t in multi_matching_chunks {
            let (to_combine, rest): (Vec<Basin>, Vec<Basin>) = basins.into_iter()
                .partition(|b| b.matches(&t));
            let mut combined = Basin::empty();
            for mut b in to_combine {
                combined.combine(&mut b);
            }

            // Doing this iteratively means "W" structures will be properly combined.
            // (See 3.ie)
            basins = rest;
            basins.push(combined);
        }

        //Combine chunks with the (now combined) basins they matched.
        let remaining = basins.iter_mut().fold(row, |r, basin| basin.extend(r));

        // All remaining chunks are NEW basins.
        for chunk in remaining {
            basins.push(Basin::new(chunk));
        }
    }

    for basin in basins.iter_mut() {
        basin.close();
    }

    let sizes = &mut basins.iter().map(Basin::size).collect::<Vec<usize>>();
    sizes.sort();
    return Ok(sizes.iter()
        .rev()
        .take(3)
        .fold(1, |acc, n| acc * n));
}

fn to_intervals(measurements: Vec<usize>) -> Vec<(usize, usize)> {
    return measurements.windows(2)
        .enumerate()
        .flat_map(|(idx, pair)| match pair {
            [9, 9] => None,
            [9, _] => Some(idx),
            [_, 9] => Some(idx - 1),
            _ => None
        }).collect::<Vec<usize>>()
        .chunks(2)
        .map(|s| match s {
            [lhs, rhs] => (*lhs, *rhs),
            _ => panic!("Should have been guaranteed doubles.")
        })
        .collect();
}
