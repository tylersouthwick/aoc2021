use aoc2021::input::{InputFileError, load_input};
use core::str::FromStr;

fn main() -> anyhow::Result<()> {
    let entries : Vec<EncodedEntry> = load_input(8)?;

    println!("part1: {}", entries.part1());
    println!("part2: {}", entries.part2()?);

    Ok(())
}

trait Day8 {
    fn part1(&self) -> i64;
    fn part2(&self) -> anyhow::Result<i64>;
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
            total += match digit.len() {
                /* 1 */ 2 | /* 4 */ 4 | /* 7 */ 3 | /* 8 */ 7 => count,
                _ => 0,
            };

        }

        total
    }

    fn part2(&self) -> anyhow::Result<i64> {
        let mut total = 0;

        for entry in self.iter() {
            total += entry.decoded_output()?;
        }

        Ok(total)
    }
}

#[derive(Debug, PartialEq)]
struct EncodedEntry {
    outputs : Vec<EncodedDigit>,
    inputs: Vec<EncodedDigit>,
}

impl std::ops::Sub<EncodedDigit> for EncodedDigit {
    type Output = Self;

    fn sub(self, rhs : Self) -> Self::Output {
        let mut list = vec![];
        for item in self.value().chars() {
            if !rhs.value().contains(item) {
                list.push(item);
            }
        }
        EncodedDigit::new(list.into_iter().collect::<String>())
    }
}

impl std::ops::Add<EncodedDigit> for EncodedDigit {
    type Output = Self;

    fn add(self, rhs : Self) -> Self::Output {
        let mut list = vec![];
        for item in rhs.value().chars() {
            list.push(item);
        }
        for item in self.value().chars() {
            list.push(item);
        }
        EncodedDigit::new(list.into_iter().collect::<String>())
    }
}

impl EncodedDigit {
    fn intersects(self, rhs : Self) -> bool {
        let mut i = true;
        for item in rhs.value().chars() {
            i &= self.value().contains(item);
        }
        i
    }

}

impl EncodedEntry {
    fn decode_value(&self, s : &str) -> anyhow::Result<i64> {
        let digit : EncodedDigit = FromStr::from_str(s)?;

        let key = self.build_key();

        Ok(*key.get(&digit).unwrap_or_else(|| &0))
    }

    fn build_key(&self) -> std::collections::HashMap<EncodedDigit, i64> {
        let mut map = std::collections::HashMap::new();
        for digit in self.inputs.iter() {
            match digit.len() {
                /* 1 */ 2 => map.insert(1, *digit),
                /* 4 */ 4 => map.insert(4, *digit),
                /* 7 */ 3 => map.insert(7, *digit),
                /* 8 */ 7 => map.insert(8, *digit),
                _ => None,
            };
        }

        let four = map.get_mut(&4).unwrap().clone();
        let one = map.get_mut(&1).unwrap().clone();
        let seven = map.get_mut(&7).unwrap().clone();
        let eight = map.get_mut(&8).unwrap().clone();

        let a = seven - one;
        let g = self.inputs.iter()
            .filter(|x| x.len() == 6)
            .map(|x| *x - (four + seven))
            .filter(|x| x.len() == 1)
            .collect::<Vec<EncodedDigit>>()[0];

        let nine = a + g + four + seven;

        let e = eight - nine;

        let six = *self.inputs.iter()
            .filter(|x| x.len() == 6)
            .filter(|x| **x != nine)
            .filter(|x| one.intersects(eight - **x))
            .collect::<Vec<_>>()[0];

        let c = eight - six;
        let f = one - c;

        let five = six - e;

        let two = *self.inputs.iter()
            .filter(|x| x.len() == 5)
            .filter(|x| c + e == **x - five)
            .collect::<Vec<_>>()[0];

        let three= two + f - e;

        let d = two - a - c - e -g;
        let zero = eight - d;

        let mut key = std::collections::HashMap::new();

        key.insert(zero, 0);
        key.insert(one, 1);
        key.insert(two, 2);
        key.insert(three, 3);
        key.insert(four, 4);
        key.insert(five, 5);
        key.insert(six, 6);
        key.insert(seven, 7);
        key.insert(eight, 8);
        key.insert(nine, 9);

        for (key, digit) in key.iter() {
            println!("key={} digit={}", key, digit);
        }

        key
    }

    fn decoded_output(&self) -> anyhow::Result<i64> {
        let mut output = vec![];
        let key = self.build_key();
        for o in self.outputs.iter() {
            match key.get(&o) {
                Some(decoded_value) =>output.push(decoded_value),
                None => return Err(anyhow::anyhow!("unable to decode {}", o)),
            };
        }

        let mut result = 0;
        for i in 0..output.len() {
            result += output[output.len() - i - 1] * 10_i64.pow(i.try_into().unwrap());
        }
        Ok(result)
    }
}

impl FromStr for EncodedEntry {
    type Err = InputFileError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let tokens : Vec<&str> = s.split("|").collect();
        if tokens.len() != 2 {
            return Err(InputFileError::GeneralError("invalid | token".to_string()))
        }

        let outputs = tokens[1].split_whitespace().map(EncodedDigit::from_str).collect::<Result<Vec<EncodedDigit>, _>>()?;
        let inputs = tokens[0].split_whitespace().map(EncodedDigit::from_str).collect::<Result<Vec<EncodedDigit>, _>>()?;

        Ok(EncodedEntry {
            outputs,
            inputs,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct EncodedDigit {
    a : bool,
    b : bool,
    c : bool,
    d : bool,
    e : bool,
    f : bool,
    g : bool,
}

impl std::fmt::Display for EncodedDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl EncodedDigit {
    fn new<V : Into<String>>(value : V) -> Self {
        let mut digit = EncodedDigit::default();

        for c in value.into().chars() {
            match c {
                'a' => digit.a = true,
                'b' => digit.b = true,
                'c' => digit.c = true,
                'd' => digit.d = true,
                'e' => digit.e = true,
                'f' => digit.f = true,
                'g' => digit.g = true,
                _ => {},
            };
        }

        digit
    }

    fn value(&self) -> String {
        let mut s = String::new();

        if self.a {
            s.push('a');
        }
        if self.b {
            s.push('b');
        }
        if self.c {
            s.push('c');
        }
        if self.d {
            s.push('d');
        }
        if self.e {
            s.push('e');
        }
        if self.f {
            s.push('f');
        }
        if self.g {
            s.push('g');
        }

        s
    }

    fn len(&self) -> usize {
        let mut count = 0;
        if self.a {
            count += 1;
        }
        if self.b {
            count += 1;
        }
        if self.c {
            count += 1;
        }
        if self.d {
            count += 1;
        }
        if self.e {
            count += 1;
        }
        if self.f {
            count += 1;
        }
        if self.g {
            count += 1;
        }
        count
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
        inputs: vec![
            EncodedDigit::new("be"),
            EncodedDigit::new("cfbegad"),
            EncodedDigit::new("cbdgef"),
            EncodedDigit::new("fgaecd"),
            EncodedDigit::new("cgeb"),
            EncodedDigit::new("fdcge"),
            EncodedDigit::new("agebfd"),
            EncodedDigit::new("fecdb"),
            EncodedDigit::new("fabcd"),
            EncodedDigit::new("edb"),
        ]
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

        #[test]
        fn part2() -> anyhow::Result<()> {
            let entries : Vec<EncodedEntry> = load_sample(8)?;

            assert_eq!(entries.part2()?, 61229);

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

        #[test]
        fn part2() -> anyhow::Result<()> {
            let entries : Vec<EncodedEntry> = load_input(8)?;

            assert_eq!(entries.part2()?, 1012089);

            Ok(())
        }
    }

    #[rstest]
    #[case("cdfbe", 5)]
    #[case("gcdfa", 2)]
    #[case("fbcad", 3)]
    #[case("dab", 7)]
    #[case("cefabd", 9)]
    #[case("cdfgeb", 6)]
    #[case("eafb", 4)]
    #[case("cagedb", 0)]
    #[case("ab", 1)]
    fn decode_digits(#[case] digit : &str, #[case] value : i64) -> anyhow::Result<()> {
        let input : EncodedEntry = FromStr::from_str("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")?;

        println!("digit: {}", digit);
        assert_eq!(input.decode_value(digit)?, value);

        Ok(())
    }

    #[test]
    fn decoded_output() -> anyhow::Result<()> {
        let input : EncodedEntry = FromStr::from_str("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")?;

        assert_eq!(input.decoded_output()?, 5353);

        Ok(())
    }

    mod digits {
        use super::*;

        #[rstest]
        #[case("a", "b", "ab")]
        #[case("aa", "b", "ab")]
        #[case("abcdefg", "abc", "abcdefg")]
        fn add(#[case] lhs : &str, #[case] rhs : &str, #[case] expected : &str) -> anyhow::Result<()> {
            let l : EncodedDigit = FromStr::from_str(lhs)?;
            let r : EncodedDigit = FromStr::from_str(rhs)?;
            let e : EncodedDigit = FromStr::from_str(expected)?;

            assert_eq!(l + r, e);

            Ok(())
        }

        #[rstest]
        #[case("a", "b", "a")]
        #[case("a", "a", "")]
        #[case("ab", "b", "a")]
        #[case("abcdefg", "abc", "defg")]
        fn sub(#[case] lhs : &str, #[case] rhs : &str, #[case] expected : &str) -> anyhow::Result<()> {
            let l : EncodedDigit = FromStr::from_str(lhs)?;
            let r : EncodedDigit = FromStr::from_str(rhs)?;
            let e : EncodedDigit = FromStr::from_str(expected)?;

            println!("{} - {} = {} [{}]", l, r, l - r, e);
            assert_eq!(l - r, e);

            Ok(())
        }
    }
}
