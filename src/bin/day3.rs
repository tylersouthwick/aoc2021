use aoc2021::input::{InputFileError, InputFile, load_input};
use std::str::FromStr;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let data: DiagnosticReport = load_input(3)?;

    println!("part1: {}", data.result()?);

    println!("part2: {}", data.life_support_rating()?);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum BinaryDigit {
    Zero,
    One,
}

impl From<BinaryDigit> for usize {
    fn from(binary_digit : BinaryDigit) -> Self {
        match binary_digit {
            BinaryDigit::One => 1,
            BinaryDigit::Zero => 0,
        }
    }
}

#[derive(Debug)]
struct BinaryNumber {
    bits : Vec<BinaryDigit>
}

impl BinaryNumber {
    fn new() -> Self {
        BinaryNumber {
            bits: vec![],
        }
    }

    fn push(&mut self, digit : BinaryDigit) {
        self.bits.push(digit);
    }
}

impl From<BinaryNumber> for usize {
    fn from(binary_number : BinaryNumber) -> Self {
        let mut result = 0;
        for i in 0..binary_number.bits.len() {
            let digit : usize = binary_number.bits[binary_number.bits.len() - i - 1].into();
            let offset = digit * 2_usize.pow(i.try_into().unwrap());
            result += offset;
        }
        result
    }
}

impl From<DiagnosticReading> for usize {
    fn from(diagnostic_reading: DiagnosticReading) -> Self {
        let mut result = 0;
        for i in 0..diagnostic_reading.bits.len() {
            let digit : usize = diagnostic_reading.bits[diagnostic_reading.bits.len() - i - 1].into();
            let offset = digit * 2_usize.pow(i.try_into().unwrap());
            result += offset;
        }
        result
    }
}

#[derive(Debug, Default, PartialEq)]
struct DiagnosticReport {
    readings : Vec<DiagnosticReading>,
}

use std::hash::Hash;

struct Counter {
    map : std::collections::HashMap<BinaryDigit, usize>,
}

impl Default for Counter {
    fn default() -> Self {
        Counter {
            map: std::collections::HashMap::new(),
        }
    }
}

impl Counter {
    fn push(&mut self, a : BinaryDigit) {
        *self.map.entry(a).or_insert(0) += 1;
    }

    fn do_max(&self) -> Option<(&BinaryDigit, &usize)> {
        let mut max = None;
        for (key, value) in self.map.iter() {
            match max {
                Some((_, max_value)) => {
                    if max_value < value {
                        max = Some((key, value))
                    }
                },
                None => {
                    max = Some((key, value))
                }
            }
        }
        max
    }

    fn max(&self) -> Option<BinaryDigit> {
        self.do_max().map(|x| x.0.clone())
    }

    fn do_min(&self) -> Option<(&BinaryDigit, &usize)> {
        let mut min = None;
        for (key, value) in self.map.iter() {
            match min {
                Some((_, min_value)) => {
                    if min_value > value {
                        min = Some((key, value))
                    }
                },
                None => {
                    min = Some((key, value))
                }
            }
        }
        min
    }

    fn min(&self) -> Option<BinaryDigit> {
        self.do_min().map(|x| x.0.clone())
    }

    fn is_equal(&self) -> bool {
        let result : Vec<&usize> = self.map.values().unique().collect();
        result.len() == 1
    }
}

struct RatingGenerator {
    readings : Vec<DiagnosticReading>,
    heristic : Box<dyn Fn(&Counter) -> Option<BinaryDigit>>,
    default_when_equal : BinaryDigit,
}

impl RatingGenerator {
    fn new(report : &DiagnosticReport, heristic : Box<dyn Fn(&Counter) -> Option<BinaryDigit>>, default_when_equal : BinaryDigit) -> Self {
        RatingGenerator {
            readings: report.readings.clone(),
            heristic,
            default_when_equal,
        }
    }

    fn find_bit(&mut self, pos : usize) -> anyhow::Result<BinaryDigit> {
        let mut counter = Counter::default();
        for reading in self.readings.iter() {
            counter.push(reading.bits[pos]);
        }
        if counter.is_equal() {
            Ok(self.default_when_equal)
        } else {
            match (self.heristic)(&counter) {
                Some(value) => Ok(value),
                None => Err(anyhow::anyhow!("could not find a value bit")),
            }
        }
    }

    fn calculate(&mut self) -> anyhow::Result<usize> {
        let mut binary_number = BinaryNumber::new();
        for i in 0..self.readings[0].bits.len() {
            let digit = self.find_bit(i)?;
            //println!("digit: {:?}", digit);
            binary_number.push(digit);
            //println!("current readings: {}", self.readings.len());
            self.readings = self.readings.iter()
                .filter(|reading| reading.bits[i] == digit)
                .map(Clone::clone)
                .collect();
            //println!("after filter readings: {} -> {:?}", self.readings.len(), binary_number);
            if self.readings.len() == 1 {
                return Ok(self.readings[0].clone().into())
            }
        }
        Err(anyhow::anyhow!("unable to calculate"))
    }
}

impl DiagnosticReport {
    fn result(&self) -> anyhow::Result<usize> {
        Ok(self.gamma_rate()? * self.epsilon_rate()?)
    }

    fn life_support_rating(&self) -> anyhow::Result<usize> {
        Ok(self.oxygen_generator_rating()? * self.co2_scrubber_rating()?)
    }

    fn oxygen_generator_rating(&self) -> anyhow::Result<usize> {
        //println!("start oxygen");
        let mut generator = RatingGenerator::new(self, Box::new(Counter::max), BinaryDigit::One);
        let r = generator.calculate()?;
        //println!("end oxygen");
        Ok(r)
    }

    fn co2_scrubber_rating(&self) -> anyhow::Result<usize> {
        //println!("start co2");
        let mut generator = RatingGenerator::new(self, Box::new(Counter::min), BinaryDigit::Zero);
        let r = generator.calculate()?;
        //println!("end co2");
        Ok(r)
    }

    fn gamma_rate(&self) -> anyhow::Result<usize> {
        let mut bits = BinaryNumber::new();
        for i in 0..self.max_bit_width() {
            bits.push(self.most_common(i)?)
        }
        Ok(bits.into())
    }

    fn max_bit_width(&self) -> usize {
        self.readings[0].bits.len()
    }

    fn most_common(&self, i : usize) -> anyhow::Result<BinaryDigit> {
        let mut counter = Counter::default();
        for reading in self.readings.iter() {
            counter.push(reading.bits[i]);
        }

        match counter.max() {
            Some(v) => Ok(v),
            None => Err(anyhow::anyhow!("no max found in counter")),
        }
    }

    fn least_common(&self, i : usize) -> anyhow::Result<BinaryDigit> {
        let mut counter = Counter::default();
        for reading in self.readings.iter() {
            counter.push(reading.bits[i]);
        }

        match counter.min() {
            Some(v) => Ok(v),
            None => Err(anyhow::anyhow!("no min found in counter")),
        }
    }

    fn epsilon_rate(&self) -> anyhow::Result<usize> {
        let mut bits = BinaryNumber::new();
        for i in 0..self.max_bit_width() {
            bits.push(self.least_common(i)?)
        }
        Ok(bits.into())
    }
}

impl From<Vec<DiagnosticReading>> for DiagnosticReport {
    fn from(readings : Vec<DiagnosticReading>) -> Self {
        DiagnosticReport {
            readings
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
struct DiagnosticReading {
    bits : Vec<BinaryDigit>
}

impl FromStr for DiagnosticReading {
    type Err = InputFileError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let mut bits = vec![];
        for c in s.chars() {
            let digit = match c {
                '1' => Ok(BinaryDigit::One),
                '0' => Ok(BinaryDigit::Zero),
                _ => Err(InputFileError::GeneralError(format!("invalid binary digit: {}", c)))
            }?;
            bits.push(digit);
        }
        Ok(DiagnosticReading {
            bits,
        })
    }
}

impl TryFrom<InputFile> for DiagnosticReport {
    type Error = InputFileError;

    fn try_from(input_file : InputFile) -> Result<Self, Self::Error> {
        let diagnostic_readings : Vec<DiagnosticReading> = input_file.try_into()?;
        Ok(diagnostic_readings.into())
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use super::*;
    use super::BinaryDigit::*;
    use aoc2021::InputFile;

    #[rstest]
    #[case("00100", vec![Zero, Zero, One, Zero, Zero])]
    #[case("11110", vec![One, One, One, One, Zero])]
    fn parse_binary_digits(#[case] s : &str, #[case] bits : Vec<BinaryDigit>) -> anyhow::Result<()> {
        let parsed : DiagnosticReading = FromStr::from_str(s)?;
        assert_eq!(parsed, DiagnosticReading { bits });
        Ok(())
    }

    #[test]
    fn calculate() -> anyhow::Result<()> {
        let input = InputFile {
            data: vec![
                "00100".to_string(),
                "11110".to_string(),
                "10110".to_string(),
                "10111".to_string(),
                "10101".to_string(),
                "01111".to_string(),
                "00111".to_string(),
                "11100".to_string(),
                "10000".to_string(),
                "11001".to_string(),
                "00010".to_string(),
                "01010".to_string(),
            ].join("\n"),
        };
        let diagnostic_report : DiagnosticReport = input.try_into()?;
        assert_eq!(9, diagnostic_report.epsilon_rate()?);
        assert_eq!(22, diagnostic_report.gamma_rate()?);
        assert_eq!(198, diagnostic_report.result()?);

        assert_eq!(23, diagnostic_report.oxygen_generator_rating()?);
        assert_eq!(10, diagnostic_report.co2_scrubber_rating()?);
        assert_eq!(230, diagnostic_report.life_support_rating()?);

        Ok(())
    }
}
