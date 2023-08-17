use std::fmt;
fn main() {
    Game::new().two_player_mode();
}

use StateColor::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum StateColor {
    Black,
    White,
}

impl fmt::Display for StateColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Black => write!(f, "黒"),
            White => write!(f, "白"),
        }
    }
}

impl StateColor {
    fn another(&self) -> StateColor {
        match self {
            Black => White,
            White => Black,
        }
    }
}

type State = Option<StateColor>;

trait StoneDisplay {
    fn to_string(&self) -> String;
}

impl StoneDisplay for State {
    fn to_string(&self) -> String {
        match self {
            // StateColorの方で定義するのもありなのかも
            Some(Black) => "●".to_string(),
            Some(White) => "○".to_string(),
            None => " ".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

const DIRECTION_ALL_PATTARN: [Direction; 8] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
    Direction::UpLeft,
    Direction::UpRight,
    Direction::DownLeft,
    Direction::DownRight,
];

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    // その方向に1マス進んだPointを返す
    pub fn point_calc_in_direction(point: Point, direction: &Direction) -> Option<Point> {
        // ぶっちゃけchecked_addとか使う必要ない
        let tmp = match direction {
            Direction::Up => (point.x.checked_add(1), Some(point.y)),
            Direction::Down => (point.x.checked_sub(1), Some(point.y)),
            Direction::Left => (Some(point.x), point.y.checked_sub(1)),
            Direction::Right => (Some(point.x), point.y.checked_add(1)),
            Direction::UpLeft => (point.x.checked_add(1), point.y.checked_sub(1)),
            Direction::UpRight => (point.x.checked_add(1), point.y.checked_add(1)),
            Direction::DownLeft => (point.x.checked_sub(1), point.y.checked_sub(1)),
            Direction::DownRight => (point.x.checked_sub(1), point.y.checked_add(1)),
        };
        match tmp {
            (Some(x @ 0..=7), Some(y @ 0..=7)) => Some(Point { x, y }),
            _ => None,
        }
    }

    pub fn from_input(input: &str) -> Point {
        let mut chars = input.chars();
        let y = match chars.next().unwrap() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("invalid input"),
        };
        let x = match chars.next().unwrap().to_digit(10).unwrap() {
            1 => 0,
            2 => 1,
            3 => 2,
            4 => 3,
            5 => 4,
            6 => 5,
            7 => 6,
            8 => 7,
            _ => panic!("invalid input"),
        };
        Point { x, y }
    }
}

#[derive(Debug)]
struct Board([[State; 8]; 8]);

impl Board {
    pub fn new() -> Self {
        let mut bord = [[None; 8]; 8];
        bord[3][3] = Some(White);
        bord[4][4] = Some(White);
        bord[3][4] = Some(Black);
        bord[4][3] = Some(Black);

        Board(bord)
    }

    pub fn check_end(&self) -> bool {
        !(self.check_valid_put_all(Black) || self.check_valid_put_all(White))
    }

    // スキップになるかを判別するため
    pub fn check_valid_put_all(&self, state_color: StateColor) -> bool {
        for x in 0..8 {
            for y in 0..8 {
                if self.0[x][y].is_none() {
                    let point = Point { x, y };
                    if self.can_place(point, state_color) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn can_put_points(&self, state_color: StateColor) -> Vec<Point> {
        let mut result = vec![];
        for x in 0..8 {
            for y in 0..8 {
                if self.0[x][y].is_none() {
                    let point = Point { x, y };
                    if self.can_place(point, state_color) {
                        result.push(point);
                    }
                }
            }
        }
        result
    }

    pub fn put(&mut self, point: Point, state_color: StateColor) -> bool {
        let mut result = false;

        let mut tmps = vec![];
        for d in DIRECTION_ALL_PATTARN {
            // どこまで置けるか
            let tmp = self.get_last_flippable_tile_in_direction(point, state_color, &d);


            // ひっくり返す
            if let Some(p) = tmp {
                tmps.push((p, d.clone()));
                result = true;
            }
        }

        for (end_point, direction) in tmps {
            self.flip_tiles_in_direction(point, end_point, state_color, &direction);
        }

        result
    }

    // 置けるかどうか
    fn can_place(&self, point: Point, state_color: StateColor) -> bool {
        let mut result = false;

        for d in DIRECTION_ALL_PATTARN {
            let tmp = self.get_last_flippable_tile_in_direction(point, state_color, &d);
            if let Some(_) = tmp {
                result = true;
            }
        }
        result
    }

    // todo:pointとstateをまとめる構造体があってもいいかも
    // ひっくり返す場所を教えてくれる
    fn get_last_flippable_tile_in_direction(
        &self,
        point: Point,
        state_color: StateColor,
        direction: &Direction,
    ) -> Option<Point> {
        let mut tmp_point = point.clone();
        // 違う色があったか
        let mut found_another_color = false;

        if let Some(_) = self.0[point.x][point.y] {
            return None;
        }

        loop {
            match Point::point_calc_in_direction(tmp_point, &direction) {
                Some(p) => tmp_point = p,
                None => return None,
            }

            match self.0[tmp_point.x][tmp_point.y] {
                None => {
                    return None;
                }
                Some(color) => {
                    if color.another() == state_color {
                        found_another_color = true;
                    } else if found_another_color {
                        break;
                    } else {
                        return None;
                    }
                }
            }
        }

        Some(tmp_point)
    }

    // ひっくり返す
    fn flip_tiles_in_direction(
        &mut self,
        put_point: Point,
        end_point: Point,
        state_color: StateColor,
        direction: &Direction,
    ) {
        let mut flip_point = put_point.clone();
        loop {
            self.0[flip_point.x][flip_point.y] = Some(state_color);
            match Point::point_calc_in_direction(flip_point, &direction) {
                Some(p) => flip_point = p,
                None => panic!("invalid flip"),
            }
            if flip_point == end_point {
                break;
            }
        }
    }
}

impl fmt::Display for Board {
    // sample
    //  |a|b|c|d|e|f|g|h|
    // 1| | | | | | | | |
    // 2| | | | | | | | |
    // 3| | | | | | | | |
    // 4| | | |○|●| | | |
    // 5| | | |●|○| | | |
    // 6| | | | | | | | |
    // 7| | | | | | | | |
    // 8| | | | | | | | |
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(" |a|b|c|d|e|f|g|h|\n");
        for i in 0..8 {
            result.push_str(&format!("{}|", i + 1));
            for j in 0..8 {
                result.push_str(&format!("{}|", self.0[i][j].to_string()));
            }
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}

struct Game {
    board: Board,
    turn: StateColor,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            turn: Black,
        }
    }

    pub fn two_player_mode(&mut self) {
        loop {
            if self.board.check_end() {
                println!("end");
                break;
            }
            if !self.board.check_valid_put_all(self.turn) {
                println!("pass");
                self.turn = self.turn.another();
                continue;
            }

            println!("{}", self.board);
            println!("{}の番です", self.turn);
            
            let point = self.input_point();
            if self.board.put(point, self.turn) {
                self.turn = self.turn.another();
            } else {
                println!("invalid input");
            }
        }
    }

    fn input_point(&self) -> Point {
        loop {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let point = Point::from_input(&input);
            if self.board.can_place(point, self.turn) {
                return point;
            } else {
                println!("invalid input");
            }
        }
    }
}

#[cfg(test)]
struct TestGame {
    board: Board,
    turn: StateColor,
}

#[cfg(test)]
impl TestGame {
    pub fn new() -> Self {
        TestGame {
            board: Board::new(),
            turn: Black,
        }
    }

    fn put_in_points(points: Vec<Point>) {
        let mut game = TestGame::new();

        for point in points {
            game.board.put(point, game.turn);
            game.turn = game.turn.another();
        }
    }

    fn random() {
        use rand::prelude::SliceRandom;
        let mut game = TestGame::new();
        let mut rng = rand::thread_rng();
        loop {
            if game.board.check_end() {
                break;
            }
            if !game.board.check_valid_put_all(game.turn) {
                game.turn = game.turn.another();
                continue;
            }
            let points = game.board.can_put_points(game.turn);
            let point = points.choose(&mut rng).unwrap().clone();
            if game.board.put(point, game.turn) {
                game.turn = game.turn.another();
            } else {
                println!("invalid input");
            }
        }
    }
}

#[test]
fn random_tests() {
    for _ in 0..1000 {
        TestGame::random();
    }
}

#[test]
fn put_in_points() {
    let points = vec![
        Point { x: 3, y: 2 },
        Point { x: 2, y: 4 },
        Point { x: 1, y: 5 },
        Point { x: 3, y: 1 },
        Point { x: 2, y: 3 },
        Point { x: 2, y: 2 },
        Point { x: 2, y: 1 },
        Point { x: 1, y: 4 },
        Point { x: 1, y: 3 },
        Point { x: 1, y: 1 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: 6 },
        Point { x: 4, y: 0 },
        Point { x: 4, y: 1 },
        Point { x: 2, y: 5 },
        Point { x: 0, y: 3 },
        Point { x: 0, y: 0 },
        Point { x: 5, y: 3 },
        Point { x: 0, y: 5 },
        Point { x: 3, y: 5 },
        Point { x: 0, y: 4 },
        Point { x: 1, y: 6 },
        Point { x: 3, y: 6 },
        Point { x: 4, y: 2 },
        Point { x: 0, y: 2 },
        Point { x: 3, y: 7 },
        Point { x: 5, y: 1 },
        Point { x: 2, y: 6 },
        Point { x: 6, y: 3 },
        Point { x: 3, y: 0 },
        Point { x: 1, y: 7 },
        Point { x: 5, y: 4 },
        Point { x: 4, y: 7 },
        Point { x: 5, y: 7 },
        Point { x: 4, y: 5 },
        Point { x: 0, y: 1 },
        Point { x: 2, y: 0 },
        Point { x: 2, y: 7 },
        Point { x: 1, y: 2 },
        Point { x: 7, y: 2 },
        Point { x: 7, y: 3 },
        Point { x: 6, y: 2 },
        Point { x: 5, y: 5 },
        Point { x: 6, y: 4 },
        Point { x: 6, y: 7 },
        Point { x: 5, y: 2 },
        Point { x: 7, y: 1 },
        Point { x: 6, y: 0 },
        Point { x: 5, y: 0 },
        Point { x: 6, y: 1 },
        Point { x: 7, y: 5 },
        Point { x: 7, y: 4 },
        Point { x: 0, y: 7 },
        Point { x: 4, y: 6 },
        Point { x: 6, y: 6 },
        Point { x: 7, y: 6 },
        Point { x: 5, y: 6 },
        Point { x: 7, y: 0 },
        Point { x: 6, y: 5 },
        Point { x: 7, y: 7 },
    ];
    TestGame::put_in_points(points)
}
