use aoc2021::input::{InputFileError, InputFile, load_input};
use core::str::FromStr;

fn main() -> anyhow::Result<()> {
    let mut part1 : SchoolOfFish = load_input(6)?;
    part1.spawn(80);

    println!("part1: {}", part1.fish_count());

    let mut part2 : SchoolOfFish = aoc2021::input::load_input(6)?;
    part2.spawn(256);

    println!("part2: {}", part2.fish_count());

    Ok(())
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Fish {
    timer : usize,
}

impl Fish {

    fn from(timer : usize) -> Self {
        Fish {
            timer,
        }
    }

}

#[derive(Debug, PartialEq, Default)]
struct SchoolOfFish {
    fish_ages : std::collections::BTreeMap<usize, i64>
}

impl std::ops::Add for SchoolOfFish {

    type Output = SchoolOfFish;

    fn add(self, rhs: Self) -> Self::Output {
        let mut school = SchoolOfFish::default();

        for (age, count) in rhs.fish_ages.iter() {
            *school.fish_ages.entry(*age).or_insert(0) += *count;
        }

        school
    }
}

impl std::fmt::Display for SchoolOfFish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ages = (0_usize..=8_usize).into_iter()
            .map(|x| format!("{}={}", x, self.fish_ages.get(&x).unwrap_or(&0)))
            .collect::<Vec<String>>();
        ages.reverse();
        write!(f, "{}", ages.join(","))
    }
}

impl SchoolOfFish {
    fn new(fish : Vec<Fish>) -> Self {
        let mut school = SchoolOfFish::default();

        for f in fish.iter() {
            school.add(f);
        }

        school
    }

    fn add(&mut self, fish : &Fish) {
        *self.fish_ages.entry(fish.timer).or_insert(0) += 1;
    }

    fn spawn_day(&mut self) {
        let mut new_fish = std::collections::BTreeMap::new();

        for (age, count) in self.fish_ages.iter() {
            if *age == 0 {
                *new_fish.entry(6).or_insert(0) += *count;
                *new_fish.entry(8).or_insert(0) += *count;
            } else {
                *new_fish.entry(*age - 1).or_insert(0) += *count;
            }
        }

        self.fish_ages = new_fish;
    }

    fn fish_count(&self) -> i64 {
        let mut total = 0;

        for count in self.fish_ages.values() {
            total += count
        }

        total
    }

    fn spawn(&mut self, days : usize) {
        for day in 0..days {
            println!("day {} fish={}", day, self);
            self.spawn_day();
        }
    }
}

impl FromStr for SchoolOfFish {
    type Err = InputFileError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        Ok(SchoolOfFish::new(s.split(",")
                .map(str::trim)
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>())
                .collect::<Result<Vec<usize>, _>>()?
                .into_iter()
                .map(Fish::from)
                .collect()))
    }
}
impl TryFrom<InputFile> for SchoolOfFish {
    type Error = InputFileError;

    fn try_from(file : InputFile) -> Result<Self, Self::Error> {
        let schools : Vec<SchoolOfFish> = file.try_into()?;
        Ok(schools.into_iter()
            .fold(SchoolOfFish::default(), |a, b| a + b))
    }
}

#[cfg(test)]
mod day6_tests {
    use super::*;
    use aoc2021::input::load_sample;
    use rstest::rstest;

    #[test]
    fn sample() -> anyhow::Result<()> {
        let school : SchoolOfFish = load_sample(6)?;

        assert_eq!(school, SchoolOfFish::new(vec![
                Fish::from(3),
                Fish::from(4),
                Fish::from(3),
                Fish::from(1),
                Fish::from(2),
        ]));

        Ok(())
    }

    mod sample {
        use super::*;

        #[test]
        fn part1() -> anyhow::Result<()> {
            let mut school : SchoolOfFish = load_sample(6)?;
            school.spawn(80);

            assert_eq!(school.fish_count(), 5934);

            Ok(())
        }

        #[test]
        fn part2() -> anyhow::Result<()> {
            let mut school : SchoolOfFish = load_sample(6)?;
            school.spawn(256);

            assert_eq!(school.fish_count(), 26984457539);

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

            assert_eq!(school.fish_count(), 355386);

            Ok(())
        }
    }

    mod part2 {
        use super::*;
        use aoc2021::input::load_input;

        #[test]
        fn results() -> anyhow::Result<()> {
            let mut school : SchoolOfFish = load_input(6)?;
            school.spawn(256);

            assert_eq!(school.fish_count(), 1613415325809);

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

