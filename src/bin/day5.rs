use aoc2021::input::{InputFileError, InputFile, load_input};
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let field : VentField = load_input(5)?;

    println!("part1: {}", field.number_of_points_where_at_least_two_lines_overlap());

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

impl std::fmt::Display for LineSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{} -> {},{}", self.from.x, self.from.y, self.to.x, self.to.y)
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

impl LineSegment {
    fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }
    fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }
}
#[derive(Debug, PartialEq)]
struct VentField {
    lines : Vec<LineSegment>
}

#[derive(Debug, PartialEq, Default)]
struct Intersections {
    rows: Vec<Vec<i64>>,
}

use std::cmp::{min, max};

impl Intersections {
    #[cfg(test)]
    fn new(rows: Vec<Vec<i64>>) -> Self {
        Intersections {
            rows,
        }
    }

    fn increment(&mut self, x : usize, y : usize) {
        if self.rows.len() <= y {
            for _row in 0..=(y - self.rows.len()) {
                self.rows.push(vec![]);
            }
        }

        for row in self.rows.iter_mut() {
            if row.len() <= x {
                for _col in 0..=(x - row.len()) {
                    row.push(0);
                }
            }
        }
        self.rows[y][x] += 1;
    }

    fn add(&mut self, segment : &LineSegment) {
        if segment.is_horizontal() {
            for x in min(segment.from.x, segment.to.x)..=max(segment.from.x, segment.to.x) {
                self.increment(x, segment.from.y)
            }
        } else if segment.is_vertical() {
            for y in min(segment.from.y, segment.to.y)..=max(segment.to.y, segment.from.y) {
                self.increment(segment.from.x, y)
            }
        } else {
            //ignore
        }
    }

    fn width(&self) -> usize {
        if self.rows.len() > 0 {
            self.rows[0].len()
        } else {
            0
        }
    }
    fn height(&self) -> usize {
        self.rows.len()
    }
}

impl VentField {
    fn number_of_points_where_at_least_two_lines_overlap(&self) -> i64 {
        let diagram = self.diagram();
        let mut count = 0;

        for row in diagram.rows.iter() {
            for col in row.iter() {
                if col >= &2 {
                    count += 1;
                }
            }
        }

        count
    }

    fn diagram(&self) -> Intersections {
        let mut intersections = Intersections::default();
        for segment in self.lines.iter() {
            intersections.add(segment);
        }
        println!("width={} height={}", intersections.width(), intersections.height());
        intersections
    }
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

    mod line_segment {
        use super::*;

        #[test]
        fn is_horizontal() {
            let line_segment = LineSegment {
                from: Point {
                    x: 5,
                    y: 10,
                },
                to: Point {
                    x: 9,
                    y: 10,
                },
            };

            assert_eq!(line_segment.is_horizontal(), true);
        }

        #[test]
        fn is_vertical() {
            let line_segment = LineSegment {
                from: Point {
                    x: 5,
                    y: 1,
                },
                to: Point {
                    x: 5,
                    y: 10,
                },
            };

            assert_eq!(line_segment.is_vertical(), true);
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

    #[test]
    fn diagram() -> anyhow::Result<()> {
        let expected = Intersections::new(vec![
            vec![0,0,0,0,0,0,0,1,0,0],
            vec![0,0,1,0,0,0,0,1,0,0],
            vec![0,0,1,0,0,0,0,1,0,0],
            vec![0,0,0,0,0,0,0,1,0,0],
            vec![0,1,1,2,1,1,1,2,1,1],
            vec![0,0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0,0],
            vec![2,2,2,1,1,1,0,0,0,0],
        ]);

        let field : VentField = load_sample(5)?;
        assert_eq!(field.diagram(), expected);

        Ok(())
    }

    #[test]
    fn number_of_points_where_at_least_two_lines_overlap() -> anyhow::Result<()> {
        let field : VentField = load_sample(5)?;
        assert_eq!(field.number_of_points_where_at_least_two_lines_overlap(), 5);
        Ok(())
    }

    #[test]
    fn part1() -> anyhow::Result<()> {
        let field : VentField = load_input(5)?;

        assert_eq!(field.number_of_points_where_at_least_two_lines_overlap(), 6461);

        Ok(())
    }
}
