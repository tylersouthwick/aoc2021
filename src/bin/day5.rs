use aoc2021::input::{InputFileError, InputFile, load_input};
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let mut field : VentField = load_input(5)?;

    Ok(())
}

impl FromStr for Point {
    type Err = InputFileError;
    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let tokens : Vec<&str> = s.split(",").collect();

        if tokens.len() == 2 {
            Ok(Point {
                x: tokens[0].parse()?,
                y: tokens[1].parse()?,
            })
        } else {
            Err(InputFileError::GeneralError(format!("unable to parse {}", s)))
        }
    }
}

impl FromStr for LineSegment {
    type Err = InputFileError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let tokens : Vec<&str> = s.split(" -> ").collect();
        println!("tokens: {:?}", tokens);

        if tokens.len() == 2 {
            Ok(LineSegment {
                from: FromStr::from_str(tokens[0])?,
                to: FromStr::from_str(tokens[1])?,
            })
        } else {
            Err(InputFileError::GeneralError(format!("unable to parse {}", s)))
        }
    }
}

impl TryFrom<InputFile> for VentField {
    type Error = InputFileError;

    fn try_from(input : InputFile) -> Result<Self, Self::Error> {
        Ok(VentField {
            lines: input.try_into()?,
        })
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x : usize,
    y : usize,
}
#[derive(Debug, PartialEq)]
struct LineSegment {
    from : Point,
    to : Point,
}
#[derive(Debug, PartialEq)]
struct VentField {
    lines : Vec<LineSegment>
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2021::input::*;

    #[test]
    fn parse_point() -> anyhow::Result<()> {
        let point : Point= FromStr::from_str("3,9")?;
        assert_eq!(point, Point {
            x: 3,
            y: 9,
        });

        Ok(())
    }

    #[test]
    fn parse_line_segment() -> anyhow::Result<()> {
        let line_segment : LineSegment = FromStr::from_str("0,9 -> 5,9")?;
        assert_eq!(line_segment, LineSegment {
            from: Point {
                x: 0,
                y: 9,
            },
            to: Point {
                x: 5,
                y: 9,
            },
        });

        Ok(())
    }

    #[test]
    fn parse() -> anyhow::Result<()> {
        let field : VentField = InputFile {
            data: "0,9 -> 5,9\n8,0 -> 0,8".to_string(),
        }.try_into()?;
        assert_eq!(field, VentField {
            lines: vec![
                LineSegment {
                    from: Point {
                        x: 0,
                        y:9, 
                    },
                    to: Point {
                        x: 5,
                        y:9, 
                    },
                },
                LineSegment {
                    from: Point {
                        x: 8,
                        y:0, 
                    },
                    to: Point {
                        x: 0,
                        y:8, 
                    },
                },
            ],
        });
        Ok(())
    }
}
