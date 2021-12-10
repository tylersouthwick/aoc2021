use aoc2021::input::{InputFileError, load_input};
use core::str::FromStr;

fn main() -> anyhow::Result<()> {
    let entries : Vec<EncodedEntry> = load_input(8)?;

    println!("part1: {}", entries.part1());

    Ok(())
}

trait Day8 {
    fn part1(&self) -> i64;
}

impl Day8 for Vec<EncodedEntry> {
    fn part1(&self) -> i64 {
        let mut total = 0;
        let mut map = std::collections::HashMap::new();

        for entry in self.iter() {
            for digit in entry.outputs.iter() {
                *map.entry(digit).or_insert(0) += 1;
            }
        }

        for (digit, count) in map.into_iter() {
            //println!("digit={} count={}", digit.value, count);
            total += match digit.value.len() {
                /* 1 */ 2 | /* 4 */ 4 | /* 7 */ 3 | /* 8 */ 7 => count,
                _ => 0,
            };

        }

        total
    }
}

#[derive(Debug, PartialEq)]
struct EncodedEntry {
    outputs : Vec<EncodedDigit>,
}

impl FromStr for EncodedEntry {
    type Err = InputFileError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let tokens : Vec<&str> = s.split("|").collect();
        if tokens.len() != 2 {
            return Err(InputFileError::GeneralError("invalid | token".to_string()))
        }

        //ignore inputs
        let outputs = tokens[1].split_whitespace().map(EncodedDigit::from_str).collect::<Result<Vec<EncodedDigit>, _>>()?;

        Ok(EncodedEntry {
            outputs,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct EncodedDigit {
    value : String,
}

impl EncodedDigit {
    fn new<V : Into<String>>(value : V) -> Self {
        let mut chars : Vec<char> = value.into().chars().collect();
        chars.sort();
        EncodedDigit {
            value: chars.iter().collect(),
        }
    }
}
impl FromStr for EncodedDigit {
    type Err = InputFileError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        Ok(EncodedDigit::new(s))
    }
}

#[cfg(test)]
mod day8_test {

    use rstest::rstest;
    use super::*;
    use aoc2021::input::load_sample;

    #[rstest]
    #[case("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe", EncodedEntry {
        outputs: vec![
            EncodedDigit::new("fdgacbe"),
            EncodedDigit::new("cefdb"),
            EncodedDigit::new("cefbgd"),
            EncodedDigit::new("gcbe"),
        ],
    })]
    fn parse(#[case] input : &str, #[case] entry : EncodedEntry) -> anyhow::Result<()> {
        let parsed_entry : EncodedEntry = FromStr::from_str(input)?;

        assert_eq!(parsed_entry, entry);

        Ok(())
    }

    mod sample {
        use super::*;

        #[test]
        fn part1() -> anyhow::Result<()> {
            let entries : Vec<EncodedEntry> = load_sample(8)?;

            assert_eq!(entries.part1(), 26);

            Ok(())
        }
    }

    mod puzzle {
        use super::*;

        #[test]
        fn part1() -> anyhow::Result<()> {
            let entries : Vec<EncodedEntry> = load_input(8)?;

            assert_eq!(entries.part1(), 274);

            Ok(())
        }
    }
}
