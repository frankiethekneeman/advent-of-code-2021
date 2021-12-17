use std::fs;

type ParseTarget = Packet;
type Solution = u32;

const EXAMPLES: [(&str, Solution); 7] = [
    ("1", 6),
    ("2", 9),
    ("3", 14),
    ("4", 16),
    ("5", 12),
    ("6", 23),
    ("7", 31)
];

const DAY: u8 = 16;

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

enum Packet {
    Operator(u8, u8, Vec<Packet>),
    Value(u8, u8, u64)
}

fn parse(contents: String) -> Result<ParseTarget, String> {
    let binary = contents.trim().chars()
        .map(|c| match c {
            '0' => Ok("0000"),
            '1' => Ok("0001"),
            '2' => Ok("0010"),
            '3' => Ok("0011"),
            '4' => Ok("0100"),
            '5' => Ok("0101"),
            '6' => Ok("0110"),
            '7' => Ok("0111"),
            '8' => Ok("1000"),
            '9' => Ok("1001"),
            'A' => Ok("1010"),
            'B' => Ok("1011"),
            'C' => Ok("1100"),
            'D' => Ok("1101"),
            'E' => Ok("1110"),
            'F' => Ok("1111"),
            _ => error("Unrecognized Hex Digit")

        }).collect::<Result<String, String>>()?;
    let (packet, _) = parse_packet(binary)?;
    return Ok(packet);
}

fn parse_packet(from: String) -> Result<(Packet, String), String> {
    let version = u8::from_str_radix(&from[0..3], 2)
        .map_err(|e| format!("{}", e))?;
    let type_id = u8::from_str_radix(&from[3..6], 2)
        .map_err(|e| format!("{}", e))?;

    if type_id == 4 {
        let (value, rest) = parse_literal(from[6..].to_string())?;
        return Ok(
            (Packet::Value(version, type_id, value), rest)
        );
    } else {
        if from.chars().nth(6).ok_or("Missing Length Type ID")? == '0' {
            let (sub_packets, rest) = parse_packets_by_length(from[7..].to_string())?;
            return Ok(
                (Packet::Operator(version, type_id, sub_packets), rest)
            );
        } else {
            let (sub_packets, rest) = parse_packets_by_count(from[7..].to_string())?;
            return Ok(
                (Packet::Operator(version, type_id, sub_packets), rest)
            );
        }
    }
}

fn parse_literal(from: String) -> Result<(u64, String), String> {
    let mut bits = String::new();
    let mut indicator = 0;
    while from.chars().nth(indicator).ok_or("Ran off end of literal")? == '1' {
        bits.push_str(&from[indicator+1..indicator+5]);
        indicator = indicator + 5;
    }
    bits.push_str(&from[indicator+1..indicator+5]);
    let val = u64::from_str_radix(&bits, 2)
        .map_err(|e| format!("failed to parse value: {}", e))?;

    return Ok((val, from[indicator+5..].to_string()));
}

fn parse_packets_by_count(from: String) -> Result<(Vec<Packet>, String), String> {
    let count = u16::from_str_radix(&from[0..11], 2)
        .map_err(|e| format!("failed to parse packet count: {}", e))?;
    let mut rest = from[11..].to_string();
    let sub_packets = (0..count).map(|_| {
        let (packet, remainder) = parse_packet(rest.clone())?;
        rest = remainder;
        return Ok(packet);
    }).collect::<Result<Vec<Packet>, String>>()?;
    return Ok((sub_packets, rest));
}

fn parse_packets_by_length(from: String) -> Result<(Vec<Packet>, String), String> {
    let count = usize::from_str_radix(&from[0..15], 2)
        .map_err(|e| format!("failed to parse packet length: {}", e))?;
    let mut rest = from[15..15+count].to_string();
    let mut sub_packets = Vec::new();
    while rest.len() != 0 {
        let (packet, remainder) = parse_packet(rest.clone())?;
        rest = remainder;
        sub_packets.push(packet);
    }
    return Ok((sub_packets, from[15+count..].to_string()));
}

fn solve(parsed: ParseTarget) -> Result<Solution, String> {
    return Ok(sum_versions(&parsed));
}

fn sum_versions(p: &Packet) -> u32 {
    return match p {
        Packet::Operator(v, _, sub_packets) => (*v as u32) + sub_packets.iter().map(sum_versions).sum::<u32>(),
        Packet::Value(v, _, _) => *v as u32
    };
}
