use aoc2021::input::{InputFile, InputFileError, load_input};
use core::str::FromStr;

fn main() -> anyhow::Result<()> {
    let heightmap : Heightmap = load_input(8)?;

    println!("part1: {}", heightmap.part1());
    //println!("part2: {}", entries.part2()?);

    Ok(())
}

trait Day9 {
    fn part1(&self) -> i64;
}

impl Day9 for Heightmap {
    fn part1(&self) -> i64 {
        self.risk_level()
    }
}

#[derive(Debug, PartialEq)]
struct Heightmap {
    measurements: Vec<Vec<i64>>,
}

struct Row {
    measurements : Vec<i64>,
}

impl FromStr for Row {
    type Err = InputFileError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        Ok(Row {
            measurements: s.chars()
                .map(|x| x.to_string().parse::<i64>())
                .collect::<Result<Vec<i64>, _>>()?,
        })
    }
}
impl TryFrom<InputFile> for Heightmap {
    type Error = InputFileError;
    fn try_from(input : InputFile) -> Result<Self, Self::Error> {
        let lines : Vec<Row> = input.try_into()?;
        Ok(Heightmap::new(lines.iter().map(|x| x.measurements.clone()).collect()))
    }
}
impl Heightmap {
    fn new(measurements : Vec<Vec<i64>>) -> Self {
        Heightmap {
            measurements,
        }
    }

    fn risk_level(&self) -> i64 {
        let mut count = 0;
        for low_point in self.find_low_points().iter() {
            count += low_point + 1;
        }
        count
    }
    fn find_low_points(&self) -> Vec<i64> {
        let mut low_points = vec![];

        for x in 0..self.width() {
            for y in 0..self.height() {

                let v = self.get(x, y);

                let mut edges = vec![];
                if x > 0 {
                    edges.push(self.get(x - 1, y));
                }
                if x < self.width() - 1{
                    edges.push(self.get(x + 1, y));
                }
                if y > 0 {
                    edges.push(self.get(x, y - 1));
                }
                if y < self.height() - 1 {
                    edges.push(self.get(x, y + 1));
                }

                let mut less_than = true;
                for e in edges.into_iter() {
                    less_than &= v < e;
                }
                if less_than {
                    low_points.push(v);
                }
            }
        }
        low_points
    }

    fn get(&self, x : usize, y : usize) -> i64 {
        self.measurements[y][x]
    }

    fn width(&self) -> usize {
        self.measurements[0].len()
    }

    fn height(&self) -> usize {
        self.measurements.len()
    }
}

#[cfg(test)]
mod day9_tests {

    use super::*;
    use aoc2021::input::load_sample;

    #[test]
    fn parse() -> anyhow::Result<()> {
        let parsed : Heightmap = load_sample(9)?;
        let expected = Heightmap::new(vec![
            vec![2,1,9,9,9,4,3,2,1,0],
            vec![3,9,8,7,8,9,4,9,2,1],
            vec![9,8,5,6,7,8,9,8,9,2],
            vec![8,7,6,7,8,9,6,7,8,9],
            vec![9,8,9,9,9,6,5,6,7,8],
        ]);

        assert_eq!(parsed, expected);
        Ok(())
    }

    mod sample {
        use super::*;

        #[test]
        fn sample1() -> anyhow::Result<()> {
            let heightmap: Heightmap = load_sample(9)?;

            let mut low_points = heightmap.find_low_points().clone();
            low_points.sort();
            assert_eq!(low_points, vec![0, 1, 5, 5]);
            assert_eq!(heightmap.part1(), 15);
            Ok(())
        }
    }

    mod puzzle {
        use super::*;

        #[test]
        fn sample1() -> anyhow::Result<()> {
            let heightmap: Heightmap = load_input(9)?;

            assert_eq!(heightmap.part1(), 526);
            Ok(())
        }
    }
}
