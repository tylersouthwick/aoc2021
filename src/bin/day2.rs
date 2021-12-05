use aoc2021::input::{InputFileError, load_input};
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let commands : Vec<MovementCommand> = load_input(2)?;

    let mut part1 = Position::default();
    part1.r#move(&commands);
    println!("part1: {}", part1.result());

    Ok(())
}

#[derive(Default, PartialEq, Debug)]
struct Position {
    horizontal : usize,
    depth : usize
}

impl Position {
    fn r#move(&mut self, commands : &Vec<MovementCommand>) {
        for command in commands.into_iter() {
            command.r#move(self)
        }
    }

    fn result(&self) -> usize {
        self.horizontal * self.depth
    }
}

#[derive(Debug, PartialEq)]
enum MovementCommand {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl MovementCommand {
    fn r#move(&self, position : &mut Position) {
        match self {
            MovementCommand::Forward(count) => {
                position.horizontal += count
            },
            MovementCommand::Down(count) => {
                position.depth += count
            },
            MovementCommand::Up(count) => {
                position.depth -= count
            },
        }
    }
}

impl FromStr for MovementCommand {
    type Err = InputFileError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let tokens : Vec<&str> = s.split(" ").collect();

        if tokens.len() == 2 {
            let count : usize = tokens[1].parse()?;
            match tokens[0] {
                "forward" => Ok(MovementCommand::Forward(count)),
                "up" => Ok(MovementCommand::Up(count)),
                "down" => Ok(MovementCommand::Down(count)),
                _ => Err(InputFileError::GeneralError(format!("unknown command {}", tokens[0])))
            }
        } else {
            Err(InputFileError::GeneralError(format!("unable to parse {}", s)))
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use super::*;
    use super::MovementCommand::*;

    #[rstest]
    #[case("forward 5", MovementCommand::Forward(5))]
    #[case("down 3", MovementCommand::Down(3))]
    #[case("up 7", MovementCommand::Up(7))]
    fn parse_movement_command(#[case] s : &str, #[case] movement_command : MovementCommand) -> anyhow::Result<()> {
        let parsed : MovementCommand = FromStr::from_str(s)?;
        assert_eq!(parsed, movement_command);
        Ok(())
    }

    #[test]
    fn move_commands() {
        let commands = vec![
            Forward(5),
            Down(5),
            Forward(8),
            Up(3),
            Down(8),
            Forward(2),
        ];
        let mut pos = Position::default();
        pos.r#move(&commands);

        assert_eq!(pos, Position {
            depth: 10,
            horizontal: 15,
        });
        assert_eq!(pos.result(), 150);
    }
}
