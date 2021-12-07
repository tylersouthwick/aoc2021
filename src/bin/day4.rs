use aoc2021::input::{InputFileError, InputFile, load_input};

fn main() -> anyhow::Result<()> {
    let mut game: Game = load_input(4)?;

    match game.simulate() {
        SimulationResult::Winner {
            last_number,
            board,
        } => {
            println!("part1: {}", last_number * board.sum_of_all_unmarked_numbers());
        },
        x => {
            println!("part1 failed {:?}", x);
            panic!("part1 failed")
        },
    }


    Ok(())
}

#[derive(Default)]
struct Game {
    to_draw : Vec<i64>,
    boards : Vec<Board>,
}

impl Game {
    fn draw_number(&mut self, num : i64) {
        for board in self.boards.iter_mut() {
            board.draw_number(num);
        }
    }

    fn winner(&self) -> Option<Board> {
        for board in self.boards.iter() {
            if board.is_winner() {
                return Some(*board)
            }
        }
        None
    }

    fn simulate(&mut self) -> SimulationResult {
        for number in self.to_draw.clone().into_iter() {
            self.draw_number(number);
            match self.winner() {
                Some(board) => return SimulationResult::Winner {
                    last_number: number,
                    board: board,
                },
                None => {}
            }
        }
        SimulationResult::Draw
    }
}


#[derive(Debug, PartialEq, Copy, Clone, Default)]
struct Cell {
    number : i64,
    drawn : bool
}

impl Cell {
    fn new(number : i64) -> Self {
        Cell {
            number,
            drawn: false,
        }
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
struct Board {
    data : [[Cell; 5]; 5],
}

fn map_array<A : Copy, B, F>(data : [[A; 5]; 5], f : F) -> [[B; 5]; 5] where F: Fn(A) -> B{
    [
        [f(data[0][0]), f(data[0][1]), f(data[0][2]), f(data[0][3]), f(data[0][4])],
        [f(data[1][0]), f(data[1][1]), f(data[1][2]), f(data[1][3]), f(data[1][4])],
        [f(data[2][0]), f(data[2][1]), f(data[2][2]), f(data[2][3]), f(data[2][4])],
        [f(data[3][0]), f(data[3][1]), f(data[3][2]), f(data[3][3]), f(data[3][4])],
        [f(data[4][0]), f(data[4][1]), f(data[4][2]), f(data[4][3]), f(data[4][4])],
    ]
}

#[derive(Debug)]
enum SimulationResult {
    Draw,
    Winner {
        last_number : i64,
        board : Board,
    }
}

impl Board {
    fn new(data : [[i64; 5]; 5]) -> Self {
        Board {
            data: map_array(data, Cell::new),
        }
    }


    fn is_marked(&self, x : usize, y : usize) -> bool {
        self.data[x][y].drawn
    }

    fn number(&self, x : usize, y : usize) -> i64{
        self.data[x][y].number
    }

    fn draw_number(&mut self, num : i64) {
        for x in 0..5 {
            for y in 0..5 {
                if self.number(x, y) == num {
                    self.data[x][y].drawn = true
                }
            }
        }
    }

    fn sum_of_all_unmarked_numbers(&self) -> i64 {
        let mut sum = 0;
        for x in 0..5 {
            for y in 0..5 {
                if !self.data[x][y].drawn {
                    sum += self.data[x][y].number
                }
            }
        }
        sum
    }

    fn is_winner_row(&self) -> bool {
        for x in 0..5 {
            let mut count = 0;
            for y in 0..5 {
                if self.is_marked(x, y) {
                    count += 1
                }
            }
            if count == 5 {
                return true
            }
        }
        false
    }

    fn is_winner_column(&self) -> bool {
        for y in 0..5 {
            let mut count = 0;
            for x in 0..5 {
                if self.is_marked(x, y) {
                    count += 1
                }
            }
            if count == 5 {
                return true
            }
        }
        false
    }

    fn is_winner(&self) -> bool {
        self.is_winner_column() || self.is_winner_row()
    }

}

#[derive(Debug)]
enum ParsingState {
    Beginning,
    Board(usize),
}

impl TryFrom<InputFile> for Game {
    type Error = InputFileError;

    fn try_from(input : InputFile) -> Result<Self, Self::Error> {
        let mut game = Game::default();

        let mut state = ParsingState::Beginning;

        let mut board : [[i64; 5]; 5] = [[0; 5]; 5];

        for line in input.lines() {
            //println!("parsing state={:?} line=[{}]", state, line);
            match state {
                ParsingState::Beginning => {
                    game.to_draw = line.split(",").map(|i| i.parse::<i64>()).collect::<Result<Vec<i64>, _>>()?;
                    state = ParsingState::Board(0);
                }
                ParsingState::Board(row) => {
                    let cells = line.split_whitespace().map(|i| i.trim().parse::<i64>()).collect::<Result<Vec<i64>, _>>()?;
                    for (place, element) in board[row].iter_mut().zip(cells.iter()) {
                        *place = *element;
                    }
                    if row == 4 {
                        state = ParsingState::Board(0);
                        game.boards.push(Board::new(board));
                    } else {
                        state = ParsingState::Board(row+ 1);
                    }
                }
            }
        }

        Ok(game)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2021::load_sample;

    #[test]
    fn new_board() {
        let data = [
            [22, 13, 17, 11,  0],
            [ 8,  2, 23,  4, 24],
            [21,  9, 14, 16,  7],
            [ 6, 10,  3, 18,  5],
            [ 1, 12, 20, 15, 19],
        ];
        let board = Board::new(data);

        assert_eq!(map_array(board.data, |c| c.number), data);

        for x in 0..5 {
            for y in 0..5 {
                assert_eq!(board.is_marked(x, y), false);
            }
        }

        assert_eq!(board.is_winner(), false)
    }

    #[test]
    fn mark_board() {
        let mut board = Board::new([
            [22, 13, 17, 11,  0],
            [ 8,  2, 23,  4, 24],
            [21,  9, 14, 16,  7],
            [ 6, 10,  3, 18,  5],
            [ 1, 12, 20, 15, 19],
        ]);
        for x in 0..5 {
            for y in 0..5 {
                assert_eq!(board.is_marked(x, y), false);
                board.draw_number(board.number(x, y));
                assert_eq!(board.is_marked(x, y), true);
            }
        }
    }

    #[test]
    fn board_winner_row() {
        let mut board = Board::new([
            [22, 13, 17, 11,  0],
            [ 8,  2, 23,  4, 24],
            [21,  9, 14, 16,  7],
            [ 6, 10,  3, 18,  5],
            [ 1, 12, 20, 15, 19],
        ]);
        board.draw_number(6);
        assert_eq!(board.is_winner(), false);
        board.draw_number(10);
        assert_eq!(board.is_winner(), false);
        board.draw_number(3);
        assert_eq!(board.is_winner(), false);
        board.draw_number(18);
        assert_eq!(board.is_winner(), false);
        board.draw_number(5);
        assert_eq!(board.is_winner(), true);
    }

    #[test]
    fn board_winner_column() {
        let mut board = Board::new([
            [22, 13, 17, 11,  0],
            [ 8,  2, 23,  4, 24],
            [21,  9, 14, 16,  7],
            [ 6, 10,  3, 18,  5],
            [ 1, 12, 20, 15, 19],
        ]);
        board.draw_number(11);
        assert_eq!(board.is_winner(), false);
        board.draw_number(4);
        assert_eq!(board.is_winner(), false);
        board.draw_number(16);
        assert_eq!(board.is_winner(), false);
        board.draw_number(18);
        assert_eq!(board.is_winner(), false);
        board.draw_number(15);
        assert_eq!(board.is_winner(), true);
    }

    #[test]
    fn parse_sample() -> anyhow::Result<()> {
        let game : Game = load_sample(4)?;
        assert_eq!(game.to_draw, vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1]);
        assert_eq!(game.boards, vec![
            Board::new([
                [22, 13, 17, 11,  0],
                [ 8,  2, 23,  4, 24],
                [21,  9, 14, 16,  7],
                [ 6, 10,  3, 18,  5],
                [ 1, 12, 20, 15, 19],
            ]),
            Board::new([
                [ 3, 15,  0,  2, 22],
                [ 9, 18, 13, 17,  5],
                [19,  8,  7, 25, 23],
                [20, 11, 10, 24,  4],
                [14, 21, 16, 12,  6],
            ]),
            Board::new([
                [14, 21, 17, 24,  4],
                [10, 16, 15,  9, 19],
                [18,  8, 23, 26, 20],
                [22, 11, 13,  6,  5],
                [ 2,  0, 12,  3,  7],
            ]),
        ]);

        Ok(())
    }

    #[test]
    fn simulate() -> anyhow::Result<()> {
        let mut game : Game = load_sample(4)?;
        let result = game.simulate();

        match result {
            SimulationResult::Winner {
                board,
                last_number,
            } => {
                assert_eq!(board.sum_of_all_unmarked_numbers(), 188);
                assert_eq!(last_number, 24);
            },
            _ => return Err(anyhow::anyhow!("invalid simulation result")),
        }

        Ok(())
    }

    #[test]
    fn part1() -> anyhow::Result<()> {
        let mut game : Game = load_input(4)?;
        let result = game.simulate();

        match result {
            SimulationResult::Winner {
                board,
                last_number,
            } => {
                assert_eq!(board.sum_of_all_unmarked_numbers() * last_number, 72770);
            },
            _ => return Err(anyhow::anyhow!("invalid simulation result")),
        }

        Ok(())
    }
}
