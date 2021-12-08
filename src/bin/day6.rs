use aoc2021::input::{InputFileError, InputFile, load_input};
use core::str::FromStr;

fn main() -> anyhow::Result<()> {
    let mut part1 : SchoolOfFish = load_input(6)?;
    part1.spawn(80);

    println!("part1: {}", part1.fish.len());

    Ok(())
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Fish {
    timer : usize,
}

impl Fish {
    fn new() -> Self {
        Fish {
            timer: 8,
        }
    }

    fn from(timer : usize) -> Self {
        Fish {
            timer,
        }
    }

    fn tick(&mut self) -> Vec<Fish> {
        if self.timer == 0 {
            self.timer = 6;
            vec![Fish::new()]
        } else {
            self.timer -= 1;
            vec![]
        }
    }
}

#[derive(Debug, PartialEq)]
struct SchoolOfFish {
    fish : Vec<Fish>
}

impl std::fmt::Display for SchoolOfFish{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s : Vec<String> = self.fish.iter().map(|x| x.timer.to_string()).collect();
        write!(f, "{}", s.join(","))
    }
}

impl SchoolOfFish {
    fn spawn(&mut self, days : usize) {
        for _day in 0..days {
            let mut new_fish = vec![];

            for fish in self.fish.iter_mut() {
                for new in fish.tick().into_iter() {
                    new_fish.push(new);
                }
            }

            for fish in new_fish.into_iter() {
                self.fish.push(fish);
            }
        }
    }
}

impl FromStr for SchoolOfFish {
    type Err = InputFileError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        Ok(SchoolOfFish {
            fish: s.split(",")
                .map(str::trim)
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>())
                .collect::<Result<Vec<usize>, _>>()?
                .into_iter()
                .map(Fish::from)
                .collect(),
        })
    }
}
impl TryFrom<InputFile> for SchoolOfFish {
    type Error = InputFileError;

    fn try_from(file : InputFile) -> Result<Self, Self::Error> {
        let tokens : Vec<SchoolOfFish> = file.try_into()?;
        Ok(SchoolOfFish {
            fish: tokens.iter()
                .flat_map(|x| x.fish.clone())
                .collect(),
        })
    }
}

#[cfg(test)]
mod day6_tests {
    use super::*;
    use aoc2021::input::load_sample;
    use rstest::rstest;

    #[test]
    fn tick() {
        let mut fish = Fish::new();

        assert_eq!(fish.tick().len(), 0);
        assert_eq!(fish.tick().len(), 0);
        assert_eq!(fish.tick().len(), 0);
        assert_eq!(fish.tick().len(), 0);
        assert_eq!(fish.tick().len(), 0);
        assert_eq!(fish.tick().len(), 0);
        assert_eq!(fish.tick().len(), 0);
        assert_eq!(fish.tick().len(), 0);
        assert_eq!(fish.tick().len(), 1);
    }

    #[test]
    fn sample() -> anyhow::Result<()> {
        let school : SchoolOfFish = load_sample(6)?;

        assert_eq!(school, SchoolOfFish {
            fish: vec![
                Fish::from(3),
                Fish::from(4),
                Fish::from(3),
                Fish::from(1),
                Fish::from(2),
            ],
        });

        Ok(())
    }

    mod sample {
        use super::*;

        #[test]
        fn results() -> anyhow::Result<()> {
            let mut school : SchoolOfFish = load_sample(6)?;
            school.spawn(80);

            assert_eq!(school.fish.len(), 5934);

            Ok(())
        }
    }

    mod part1 {
        use super::*;
        use aoc2021::input::load_input;

        #[test]
        fn results() -> anyhow::Result<()> {
            let mut school : SchoolOfFish = load_input(6)?;
            school.spawn(80);

            assert_eq!(school.fish.len(), 355386);

            Ok(())
        }
    }

    #[rstest]
    #[case(1, "2,3,2,0,1")]
    #[case(2, "1,2,1,6,0,8")]
    #[case(3, "0,1,0,5,6,7,8")]
    #[case(4, "6,0,6,4,5,6,7,8,8")]
    #[case(5, "5,6,5,3,4,5,6,7,7,8")]
    #[case(6, "4,5,4,2,3,4,5,6,6,7")]
    #[case(7, "3,4,3,1,2,3,4,5,5,6")]
    #[case(8, "2,3,2,0,1,2,3,4,4,5")]
    #[case(9, "1,2,1,6,0,1,2,3,3,4,8")]
    #[case(10, "0,1,0,5,6,0,1,2,2,3,7,8")]
    #[case(11, "6,0,6,4,5,6,0,1,1,2,6,7,8,8,8")]
    #[case(12, "5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8")]
    #[case(13, "4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8")]
    #[case(14, "3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8")]
    #[case(15, "2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7")]
    #[case(16, "1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8")]
    #[case(17, "0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8")]
    #[case(18, "6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8")]
    fn spawn(#[case] days : usize, #[case] expected : &str) -> anyhow::Result<()> {
        let mut school : SchoolOfFish = load_sample(6)?;
        let expected_school : SchoolOfFish = FromStr::from_str(expected)?;

        school.spawn(days);

        assert_eq!(school, expected_school);

        Ok(())
    }
}

