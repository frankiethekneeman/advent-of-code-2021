use std::fs;
use std::cmp::max;
use std::cmp::min;
use std::convert::TryInto;

type ParseTarget = Vec<Command>;
type Solution = usize;

const EXAMPLES: [(&str, Solution); 3] = [
    ("1", 39),
    ("2", 39769202357779),
    ("3", 2758514936282235),
];

const DAY: u8 = 22;

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

// fn error<T>(msg: &str) -> Result<T, String> {
//     return Err(String::from(msg));
// }

fn operation(filename: String) -> Result<Solution, String> {
    return fs::read_to_string(filename)
        .map_err(|io_error| format!("{}", io_error))
        .and_then(parse)
        .and_then(solve);
}

#[derive(PartialEq, Eq)]
struct Command {
    on: bool,
    points: Points
}

impl Command {
    fn from_input(line: &str) -> Result<Command, String> {
        let mut bits = line.split(&[' ', ','][..]);
        let on = bits.next()
            .ok_or(String::from("No on/off"))
            .and_then(|on| match on {
                "on" => Ok(true),
                "off" => Ok(false),
                other => Err(format!("Unrecognized on/off: '{}'", other))
            })?;
        let x = bits.next()
            .ok_or(String::from("insufficient segments for x range."))
            .and_then(|s| parse_range("x=", s))?;
        let y = bits.next()
            .ok_or(String::from("insufficient segments for y range."))
            .and_then(|s| parse_range("y=", s))?;
        let z = bits.next()
            .ok_or(String::from("insufficient segments for z range."))
            .and_then(|s| parse_range("z=", s))?;
        return Ok(Command {
            on: on,
            points: Points {
                xs: x,
                ys: y,
                zs: z
            }
        })
    }
}

fn parse_range(prefix: &str, line: &str) -> Result<Range, String> {
    let bounds = line.strip_prefix(prefix)
        .ok_or(format!("Expected prefix '{}' on '{}'", prefix, line))
        .and_then(|rest| rest.split("..")
            .map(str::parse)
            .collect::<Result<Vec<isize>, _>>()
            .map_err(|e| format!("{}", e))
        )?;

    if bounds.len() == 2 {
        return Ok(Range::new(bounds[0], bounds[1]))
    }

    return Err(format!("Got the wrong number of bounds from '{}'.", line));
}

fn parse(contents: String) -> Result<ParseTarget, String> {
    return contents.lines()
        .map(Command::from_input)
        .collect();
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Points {
  xs: Range,
  ys: Range,
  zs: Range
}

impl Points {
    fn size(&self) -> usize {
        return self.xs.size() * self.ys.size() * self.zs.size();
    }

    fn intersects(&self, other: &Points) -> bool {
        return self.xs.intersects(&other.xs) 
            && self.ys.intersects(&other.ys)
            && self.zs.intersects(&other.zs)
    }

    fn without(&self, other: &Points) -> Vec<Points> {
        if !self.intersects(other) {
            return vec![self.clone()];
        }

        let overlap_points = Points{
            xs: self.xs.intersection(&other.xs),
            ys: self.ys.intersection(&other.ys),
            zs: self.zs.intersection(&other.zs)
        };

        return self.xs.partition(&other.xs).iter().flat_map(|xs| 
            self.ys.partition(&other.ys).iter().flat_map(move |ys|
                self.zs.partition(&other.zs).iter().map(move |zs|
                    Points{xs: xs.clone(), ys: ys.clone(), zs: zs.clone()}
                ).collect::<Vec<Points>>()
            ).collect::<Vec<Points>>()
        ).filter(|p: &Points| p.xs.size() > 0 && p.ys.size() > 0 && p.zs.size() > 0)
        .filter(|p| *p != overlap_points)
        .collect();
        
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Range {
    Empty,
    Inclusive { bottom: isize, top: isize }
}

impl Range {
    fn new(bottom: isize, top: isize) -> Range {
        return Range::Inclusive {
            bottom: bottom,
            top: top
        };
    }

    fn size(&self) -> usize {
        return match self {
            Range::Empty => 0,
            Range::Inclusive{bottom, top} => (top - bottom + 1).try_into().unwrap()
        }
    }

    fn intersects(&self, other: &Range) -> bool {
        return self.intersection(other).size() > 0
    }

    fn intersection(&self, other: &Range) -> Range {
        return match (self, other) {
            (Range::Empty, _) => Range::Empty,
            (_, Range::Empty) => Range::Empty,
            (Range::Inclusive{bottom: sbottom, top: stop}, Range::Inclusive{bottom: obottom, top: otop}) => 
                if *stop < *obottom || *sbottom > *otop {
                    Range::Empty // disjoint!
                } else {
                    Range::Inclusive {
                        bottom: max(*sbottom, *obottom),
                        top: min(*stop, *otop)
                    }
                }
        };
    }

    /**
     *  This partitions the _current_ range with respect to the _other_ range. Specifically,
     *  it returns the portion of this range that is _before_ the other range, what portion is
     *  _in_ the other range, and what portion is _after_ the other range.
     */
    fn partition(&self, other: &Range) -> [Range; 3] {
        match (self, other) {
            (Range::Empty, _) => return [Range::Empty, Range::Empty, Range::Empty],
            (_, Range::Empty) => return [self.clone(), Range::Empty, Range::Empty],
            (Range::Inclusive{bottom: sbottom, top: stop}, Range::Inclusive{bottom: obottom, top: otop}) => {
                let before = if *sbottom < *obottom {
                    Range::Inclusive{
                        bottom: *sbottom,
                        top: min(*stop, *obottom - 1)
                    }
                } else {
                    Range::Empty
                };

                let concurrent = self.intersection(&other);

                let after = if *stop > *otop {
                    Range::Inclusive { bottom: max(*sbottom, *otop + 1), top: *stop }
                } else {
                    Range::Empty
                };

                return [before, concurrent, after];
            }
        }
    }
}

#[derive(Debug)]
struct Field {
    on: Vec<Points>
}

impl Field {
    fn new() -> Field {
        Field { on: Vec::new() }
    }

    fn size(&self) -> usize {
        return self.on.iter()
            .map(|p| p.size())
            .sum()
    }
    fn update(self, command: Command) -> Field {
        let mut points = self.on.into_iter()
            .flat_map(|p| p.without(&command.points))
            .collect::<Vec<Points>>();
        if command.on {
            points.push(command.points);
        }
        return Field {on: points};
    }
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    return Ok(parsed.into_iter()
        .fold(Field::new(), |f, c| f.update(c))
        .size());
}
