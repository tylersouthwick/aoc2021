use aoc2021::input::{InputFileError, InputFile, load_input};

fn main() -> anyhow::Result<()> {
    let crabs : Crabs = load_input(7)?;

    println!("part1: {:?}", crabs.optimized_horizontal_alignment());

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Crabs {
    crabs : Vec<Crab>,
}

impl Crabs {
    fn min(&self) -> i64 {
        let mut local = self.crabs.clone();
        local.sort();
        if local.len() > 0 {
            local[0].horizontal_position
        } else {
            0
        }
    }
    fn max(&self) -> i64 {
        let mut local = self.crabs.clone();
        local.sort();
        local.reverse();
        if local.len() > 0 {
            local[0].horizontal_position
        } else {
            0
        }
    }

    fn calculate_fuel_to_move(&self, horizontal_position : i64) -> i64 {
        let mut fuel = 0;

        for crab in self.crabs.iter() {
            fuel += (crab.horizontal_position - horizontal_position).abs();
        }

        fuel
    }

    fn optimized_horizontal_alignment(&self) -> Option<(i64, i64)> {
        let mut map = std::collections::HashMap::new();
        for location in self.min()..self.max() {
            let result = self.calculate_fuel_to_move(location);
            map.insert(location, result);
        }

        let mut min = None;

        for (location, fuel) in map.into_iter() {
            match min {
                Some((_, m)) if m > fuel => min = Some((location, fuel)),
                None => min = Some((location, fuel)),
                _ => {},
            }
        }
        min
    }
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Clone, Copy)]
struct Crab {
    horizontal_position : i64,
}

impl Crab {
    fn new(horizontal_position : i64) -> Self {
        Crab {
            horizontal_position,
        }
    }
}

impl TryFrom<InputFile> for Crabs {
    type Error = InputFileError;

    fn try_from(file : InputFile) -> Result<Self, Self::Error> {
        Ok(Crabs {
            crabs: file.with_delimeter(",")
                .data
                .iter()
                .map(|x| x.parse::<i64>())
                .collect::<Result<Vec<i64>, _>>()?
                .into_iter()
                .map(Crab::new)
                .collect(),
        })
    }
}

#[cfg(test)]
mod day7_test {
    use super::*;
    use aoc2021::input::load_sample;
    use rstest::rstest;

    #[test]
    fn parse() -> anyhow::Result<()> {
        let crabs : Crabs = load_sample(7)?;

        assert_eq!(crabs, Crabs {
            crabs: vec![16,1,2,0,4,2,7,1,2,14].into_iter().map(Crab::new).collect(),
        });

        Ok(())
    }

    #[rstest]
    #[case(2, 37)]
    #[case(1, 41)]
    #[case(3, 39)]
    #[case(10, 71)]
    fn calculate_fuel_to_move(#[case] target : i64, #[case] fuel : i64) -> anyhow::Result<()> {
        let crabs : Crabs = load_sample(7)?;

        assert_eq!(crabs.calculate_fuel_to_move(target), fuel);

        Ok(())
    }

    #[test]
    fn optimized_horizontal_alignment() -> anyhow::Result<()> {
        let crabs : Crabs = load_sample(7)?;

        assert_eq!(crabs.optimized_horizontal_alignment(), Some((2, 37)));

        Ok(())
    }

    #[test]
    fn part1() -> anyhow::Result<()> {
        let crabs : Crabs = load_input(7)?;

        assert_eq!(crabs.optimized_horizontal_alignment(), Some((325, 326132)));
        Ok(())
    }
}
