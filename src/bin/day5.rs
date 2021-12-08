use aoc2021::input::{InputFileError, InputFile, load_input};
use std::str::FromStr;
use std::cmp::{min, max};

fn main() -> anyhow::Result<()> {
    let field : VentField = load_input(5)?;

    println!("part1: {}", field.part1());

    Ok(())
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
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
        write!(f, "{} -> {}", self.from, self.to)
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

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Ord, Eq, Default, Hash)]
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

    fn line(&self, include_diagnal : bool) -> Vec<Point> {
        let mut line = vec![];
        if self.is_horizontal() {
            for x in self.min().x..=self.max().x {
                line.push(Point {
                    x,
                    y: self.from.y
                });
            }
        } else if self.is_vertical() {
            for y in self.min().y..=self.max().y {
                line.push(Point {
                    y,
                    x: self.from.x,
                });
            }
        } else if include_diagnal {
            let m = ((self.from.y as i64) - (self.to.y as i64)) / ((self.from.x as i64) - (self.to.x as i64));
            let b = (self.from.y as i64) - m * (self.from.x as i64);

            for x in min(self.from.x, self.to.x)..=max(self.from.x, self.to.x) {
                line.push(Point {
                    x,
                    y: ((m * (x as i64) + b) as usize),
                })
            }
        }

        line
    }

    fn min(&self) -> Point {
        if self.is_horizontal() {
            if self.from.x < self.to.x {
                self.from
            } else {
                self.to
            }
        } else if self.is_vertical() {
            if self.from.y < self.to.y {
                self.from
            } else {
                self.to
            }
        } else {
            self.from
        }
    }

    fn max(&self) -> Point {
        if self.is_horizontal() {
            if self.from.x > self.to.x {
                self.from
            } else {
                self.to
            }
        } else if self.is_vertical() {
            if self.from.y > self.to.y {
                self.from
            } else {
                self.to
            }
        } else {
            self.from
        }
    }
}

#[derive(Debug, PartialEq)]
struct VentField {
    lines : Vec<LineSegment>
}

#[derive(Debug, Default, PartialEq, Clone)]
struct SparseMatrix {
    points: std::collections::HashMap<Point, i64>,
}


impl SparseMatrix {
    fn new(rows: Vec<Vec<i64>>) -> Self {
        let mut matrix = SparseMatrix::default();
        for row in 0..rows.len() {
            for col in 0..rows[row].len() {
                if rows[row][col] > 0 {
                    matrix.set(col, row, rows[row][col]);
                }
            }
        }
        matrix
    }

    fn set(&mut self, x : usize, y : usize, val : i64) {
        *self.points.entry(Point{
            x,
            y,
        }).or_insert(0) += val
    }
    fn get(&mut self, x : usize, y : usize) -> i64 {
        match self.points.get(&Point{
            x,
            y,
        }) {
            Some(value) => *value,
            None => 0,
        }
    }

    fn increment(&mut self, x : usize, y : usize) {
        *self.points.entry(Point {
            x,
            y,
        }).or_insert(0) += 1
    }

    fn values(&self) -> std::collections::hash_map::Values<Point, i64> {
        self.points.values()
    }

}

#[derive(Debug, PartialEq, Default)]
struct Intersections {
    matrix: SparseMatrix,
    include_diagnal : bool,
}

impl Intersections {
    #[cfg(test)]
    fn new(rows: Vec<Vec<i64>>) -> Self {
        Intersections {
            matrix: SparseMatrix::new(rows),
            include_diagnal: false,
        }
    }
    #[cfg(test)]
    fn new_with_diagnal(rows: Vec<Vec<i64>>) -> Self {
        Intersections {
            matrix: SparseMatrix::new(rows),
            include_diagnal: true,
        }
    }

    fn add(&mut self, segment : &LineSegment) {
        for point in segment.line(self.include_diagnal).iter() {
            self.matrix.increment(point.x, point.y)
        }
    }

}

impl VentField {
    fn part1(&self) -> i64 {
        self.number_of_points_where_at_least_two_lines_overlap(false)
    }

    fn part2(&self) -> i64 {
        self.number_of_points_where_at_least_two_lines_overlap(true)
    }

    fn number_of_points_where_at_least_two_lines_overlap(&self, include_diagnal : bool) -> i64 {
        let intersections = self.intersections(include_diagnal);
        let mut count = 0;

        for val in intersections.matrix.values() {
            if val >= &2 {
                count += 1;
            }
        }

        count
    }

    fn part2_intersections(&self) -> Intersections {
        self.intersections(true)
    }

    fn part1_intersections(&self) -> Intersections {
        self.intersections(false)
    }

    fn intersections(&self, include_diagnal: bool) -> Intersections {
        let mut intersections = Intersections {
            include_diagnal,
            ..Intersections::default()
        };
        for segment in self.lines.iter() {
            intersections.add(segment);
        }
        intersections
    }
}

#[cfg(test)]
mod day5_test {
    use super::*;
    use aoc2021::input::*;

    mod point {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case("1,1", Point { x: 1, y: 1 })]
        #[case("3,4", Point { x: 3, y: 4 })]
        fn parse_point(#[case] p : &str, #[case] expected : Point) -> anyhow::Result<()> {
            let point : Point= FromStr::from_str(p)?;
            assert_eq!(point, expected);

            Ok(())
        }
    }

    mod line_segment {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case("1,1 -> 3,3", vec!["1,1", "2,2", "3,3"])]
        #[case("9,7 -> 7,9", vec!["9,7", "8,8", "7,9"])]
        fn generate_line(#[case] line_segment : &str, #[case] points : Vec<&str>) -> anyhow::Result<()> {
            let parsed_segment : LineSegment = FromStr::from_str(line_segment)?;
            let mut parsed_points : Vec<Point> = points.into_iter().map(FromStr::from_str).collect::<Result<Vec<Point>, _>>()?;

            assert_eq!(parsed_segment.line(true).sort(), parsed_points.sort());

            Ok(())
        }

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
            data: vec![
                "0,9 -> 5,9".to_string(),
                "8,0 -> 0,8".to_string(),
            ],
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

    mod sparse_matrix {
        use super::super::SparseMatrix;

        #[test]
        fn simple() {
            let mut matrix = SparseMatrix::default();
            matrix.set(1, 1, 3);
            assert_eq!(matrix.get(1, 1), 3);
        }

        #[test]
        fn simple_new_from_arrays() {
            let matrix = SparseMatrix::new(vec![
                vec![0, 3],
                vec![7, 0],
            ]);

            let mut expected = SparseMatrix::default();
            expected.set(1, 0, 3);
            expected.set(0, 1, 7);

            assert_eq!(matrix, expected);
        }
    }

    mod part1 {
        use super::*;

        #[test]
        fn intersections() -> anyhow::Result<()> {
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
            assert_eq!(field.part1_intersections(), expected);

            Ok(())
        }

        #[test]
        fn sample() -> anyhow::Result<()> {
            let field : VentField = load_sample(5)?;
            assert_eq!(field.part1(), 5);
            Ok(())
        }

        #[test]
        fn part1() -> anyhow::Result<()> {
            let field : VentField = load_input(5)?;

            assert_eq!(field.part1(), 6461);

            Ok(())
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn intersections() -> anyhow::Result<()> {
            let expected = Intersections::new_with_diagnal(vec![
                vec![1,0,1,0,0,0,0,1,1,0],
                vec![0,1,1,1,0,0,0,2,0,0],
                vec![0,0,2,0,1,0,1,1,1,0],
                vec![0,0,0,1,0,2,0,2,0,0],
                vec![0,1,1,2,3,1,3,2,1,1],
                vec![0,0,0,1,0,2,0,0,0,0],
                vec![0,0,1,0,0,0,1,0,0,0],
                vec![0,1,0,0,0,0,0,1,0,0],
                vec![1,0,0,0,0,0,0,0,1,0],
                vec![2,2,2,1,1,1,0,0,0,0],
            ]);

            let field : VentField = load_sample(5)?;
            assert_eq!(field.part2_intersections(), expected);

            Ok(())
        }

        #[test]
        fn sample() -> anyhow::Result<()> {
            let field : VentField = load_sample(5)?;
            assert_eq!(field.part2(), 12);
            Ok(())
        }

        #[test]
        fn part2() -> anyhow::Result<()> {
            let field : VentField = load_input(5)?;

            assert_eq!(field.part2(), 18065);

            Ok(())
        }
    }
}
