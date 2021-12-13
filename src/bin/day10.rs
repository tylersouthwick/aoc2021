use aoc2021::input::{InputFileError, load_input};
use core::str::FromStr;

fn main() -> anyhow::Result<()> {
    let lines : Vec<Line> = load_input(10)?;

    println!("part1: {}", lines.part1());
    println!("part2: {}", lines.part2());

    Ok(())
}

struct Line {
    value : String,
}

fn find_expected(c : char) -> char {
    match c {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => c,
    }
}

impl Line {
    fn check_syntax_error(&self) -> Option<SyntaxError> {
        let mut symbols = vec![];

        for c in self.value.chars() {
            //println!("symbols: {:?} -> {}", symbols, c);
            match c {
                '(' | '[' | '{' | '<' => symbols.push(c),
                ')' => match symbols.pop() {
                    Some(pop) if pop != '(' => {
                        return Some(SyntaxError {
                            found: ')',
                            expected: find_expected(pop),
                        })
                    },
                    _ => {},
                },
                '>' => match symbols.pop() {
                    Some(pop) if pop != '<' => {
                        return Some(SyntaxError {
                            found: '>',
                            expected: find_expected(pop),
                        })
                    },
                    _ => {},
                },
                '}' => match symbols.pop() {
                    Some(pop) if pop != '{' => {
                        return Some(SyntaxError {
                            found: '}',
                            expected: find_expected(pop),
                        })
                    },
                    _ => {},
                },
                ']' => match symbols.pop() {
                    Some(pop) if pop != '[' => {
                        return Some(SyntaxError {
                            found: ']',
                            expected: find_expected(pop),
                        })
                    },
                    _ => {},
                },
                _ => {},
            }
        }

        None
    }
}
impl FromStr for Line {
    type Err = InputFileError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        Ok(Line {
            value: s.to_string(),
        })
    }
}

pub trait Day10 {
    fn part1(&self) -> i64;
    fn part2(&self) -> i64;
}

impl Day10 for Vec<Line> {

    fn part1(&self) -> i64 {
        let mut symbols = std::collections::HashMap::new();

        for line in self.iter() {
            match line.check_syntax_error() {
                None => {},
                Some(error) => {
                    *symbols.entry(error.found).or_insert(0) += 1;
                }
            }

        }

        let mut total = 0;
        for (symbol, count) in symbols.iter() {
            total += match symbol {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            } * count
        }

        total
    }
    fn part2(&self) -> i64 {
        0
    }
}

#[derive(Debug, PartialEq)]
struct SyntaxError {
    expected : char,
    found: char,
}

impl FromStr for SyntaxError {
    type Err = InputFileError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let chars : Vec<char> = s.chars().collect();
        if chars.len() != 2{
            return Err(InputFileError::GeneralError(format!("invalid number of chars {}", chars.len())));
        }
        Ok(SyntaxError {
            expected: chars[0],
            found: chars[1],
        })
    }
}

#[cfg(test)]
mod day10_tests {

    use super::*;

    mod line {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case("{([(<{}[<>[]}>{[]{[(<()>", "]}")]
        #[case("[[<[([]))<([[{}[[()]]]", "])")]
        #[case("[{[{({}]{}}([{[{{{}}([]", ")]")]//, SyntaxError { expected: ')', found: ']' })],
        #[case("[<(<(<(<{}))><([]([]()", ">)")]//, SyntaxError { expected: '>', found: ')' })],
        #[case("<{([([[(<>()){}]>(<<{{", "]>")]//, SyntaxError{ expected: ']', found: '>' })],
        fn check_syntax_error(#[case] input : Line, #[case] syntax_error : SyntaxError) {

            let error = input.check_syntax_error();

            assert_eq!(error, Some(syntax_error));
        }

    }

    mod sample {

        use super::*;
        use aoc2021::load_sample;

        #[test]
        fn part1() -> anyhow::Result<()> {
            let lines : Vec<Line> = load_sample(10)?;

            assert_eq!(lines.part1(), 26397);
            Ok(())
        }
    }

    mod puzzle {

        use super::*;
        use aoc2021::load_input;

        #[test]
        fn part1() -> anyhow::Result<()> {
            let lines : Vec<Line> = load_input(10)?;

            assert_eq!(lines.part1(), 413733);
            Ok(())
        }
    }
}
