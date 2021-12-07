#[derive(Default)]
struct Game {
    to_draw : Vec<i64>,
    boards : Vec<Board>,
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

#[derive(Debug, PartialEq, Default)]
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
                if self.data[x][y].number == num {
                    self.data[x][y].drawn = true
                }
            }
        }
    }

}

use aoc2021::input::{InputFile, InputFileError};

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
}
