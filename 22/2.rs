use std::fs;
use std::cmp::max;
use std::cmp::min;
use std::convert::TryInto;
use std::error::Error;

type ParseTarget = Vec<Command>;
type Solution = usize;
type AoC<T> = Result<T, Box<dyn Error>>;

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
                        Ok(())
                    } else {
                        error(format!("Expected {} but got {}", expected, actual))
                    })
            )
        )
        .collect::<Vec<(&str, AoC<()>)>>();
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

fn errorize<S: Into<Box<dyn Error>>>(msg: S) -> Box<dyn Error> {
    return msg.into();
}

fn error<T, S: Into<Box<dyn Error>>>(err: S) -> AoC<T> {
    return Err(err.into());
}

fn operation(filename: String) -> AoC<Solution> {
    return fs::read_to_string(filename)
        .map_err(errorize)
        .and_then(parse)
        .and_then(solve);
}

#[derive(PartialEq, Eq)]
struct Command {
    on: bool,
    points: Points
}

impl Command {
    fn from_input(line: &str) -> AoC<Command> {
        let mut bits = line.split(&[' ', ','][..]);
        let on = bits.next()
            .ok_or(errorize("No on/off"))
            .and_then(|on| match on {
                "on" => Ok(true),
                "off" => Ok(false),
                other => error(format!("Unrecognized on/off: '{}'", other))
            })?;
        let range_strs: Vec<&str> = bits.take(3).collect();
        if range_strs.len() != 3 {
            return error(format!("Unexpected number of ranges: {}", range_strs.len()));
        }

        let ranges = range_strs.iter().zip(["x=", "y=", "z="])
            .map(|(string, prefix)| parse_range(prefix, string))
            .collect::<AoC<Vec<Range>>>()?;
        return match &ranges[..] {
            [x, y, z] => Ok(Command {
                on: on,
                points: Points {
                    xs: x.clone(),
                    ys: y.clone(),
                    zs: z.clone()
                }
            }),
            _ => error("I don't know how this happened"),
        }
    }
}

fn parse_range(prefix: &str, line: &str) -> AoC<Range> {
    let bounds = line.strip_prefix(prefix)
        .ok_or(errorize(format!("Expected prefix '{}' on '{}'", prefix, line)))
        .and_then(|rest| rest.split("..")
            .map(str::parse)
            .collect::<Result<Vec<isize>, _>>()
            .map_err(errorize)
        )?;

    if bounds.len() == 2 {
        return Ok(Range::new(bounds[0], bounds[1]))
    }

    return error(format!("Got the wrong number of bounds from '{}'.", line));
}

fn parse(contents: String) -> AoC<ParseTarget> {
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

        let xparts = &self.xs.partition(&other.xs);
        let yparts = &self.ys.partition(&other.ys);
        let zparts = &self.zs.partition(&other.zs);

        return xparts.iter().flat_map(|xs| 
            yparts.iter().flat_map(move |ys|
                zparts.iter().map(move |zs|
                    Points{xs: xs.clone(), ys: ys.clone(), zs: zs.clone()}
                )
            )
        ).filter(|p| p.size() > 0 )
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
            (Range::Empty, _) | (_, Range::Empty) => Range::Empty,
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
        return match (self, other) {
            (Range::Empty, _) => [Range::Empty, Range::Empty, Range::Empty],
            (_, Range::Empty) => [self.clone(), Range::Empty, Range::Empty],
            (Range::Inclusive{bottom: sbottom, top: stop}, Range::Inclusive{bottom: obottom, top: otop}) => {
                let before = if *sbottom < *obottom {
                    Range::Inclusive{ bottom: *sbottom, top: min(*stop, *obottom - 1) }
                } else {
                    Range::Empty
                };

                let concurrent = self.intersection(&other);

                let after = if *stop > *otop {
                    Range::Inclusive { bottom: max(*sbottom, *otop + 1), top: *stop }
                } else {
                    Range::Empty
                };

                [before, concurrent, after]
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

fn solve(parsed: ParseTarget) -> AoC<Solution> {
    return Ok(parsed.into_iter()
        .fold(Field::new(), Field::update)
        .size());
}
