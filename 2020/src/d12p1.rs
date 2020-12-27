use nom::{
    bytes::complete::take,
    character::complete::digit1,
    combinator::{map, map_res, recognize},
    IResult,
};

#[derive(Debug)]
pub enum MovementKind {
    North,
    South,
    East,
    West,
    TurnLeft,
    TurnRight,
    Forward,
}

#[derive(Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub struct Movement {
    pub kind: MovementKind,
    pub amount: u32,
}

#[derive(Debug)]
struct Ship {
    pos_x: i64,
    pos_y: i64,
    direction: Direction,
}

impl Ship {
    fn apply_direction_amount(&mut self, direction: Direction, amount: u32) {
        match direction {
            Direction::East => {
                self.pos_x += amount as i64;
            }
            Direction::West => {
                self.pos_x -= amount as i64;
            }
            Direction::North => {
                self.pos_y -= amount as i64;
            }
            Direction::South => {
                self.pos_y += amount as i64;
            }
        };
    }

    fn turn_left(&mut self) {
        match self.direction {
            Direction::East => {
                self.direction = Direction::North;
            }
            Direction::West => {
                self.direction = Direction::South;
            }
            Direction::North => {
                self.direction = Direction::West;
            }
            Direction::South => {
                self.direction = Direction::East;
            }
        };
    }

    fn turn_rigth(&mut self) {
        self.turn_left();
        self.turn_left();
        self.turn_left();
    }

    fn apply_movement(&mut self, movement: &Movement) {
        let amount = movement.amount;
        match movement.kind {
            MovementKind::East => self.apply_direction_amount(Direction::East, amount),
            MovementKind::West => self.apply_direction_amount(Direction::West, amount),
            MovementKind::North => self.apply_direction_amount(Direction::North, amount),
            MovementKind::South => self.apply_direction_amount(Direction::South, amount),

            MovementKind::Forward => self.apply_direction_amount(self.direction.clone(), amount),

            MovementKind::TurnLeft => {
                assert!(amount % 90 == 0);
                let turns = amount / 90;
                for _ in 0..turns {
                    self.turn_left();
                }
            }

            MovementKind::TurnRight => {
                assert!(amount % 90 == 0);
                let turns = amount / 90;
                for _ in 0..turns {
                    self.turn_rigth();
                }
            }
        };
    }
}

fn parse_movement_kind(input: &str) -> IResult<&str, MovementKind> {
    let parser = take(1 as usize);

    map(parser, |s| match s {
        "N" => MovementKind::North,
        "S" => MovementKind::South,
        "E" => MovementKind::East,
        "W" => MovementKind::West,
        "L" => MovementKind::TurnLeft,
        "R" => MovementKind::TurnRight,
        "F" => MovementKind::Forward,
        _ => unreachable!(),
    })(input)
}

fn parse_unsigned_integer_32(input: &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), str::parse)(input)
}

pub fn parse_movement(input: &str) -> IResult<&str, Movement> {
    let (input, kind) = parse_movement_kind(input)?;
    let (input, amount) = parse_unsigned_integer_32(input)?;

    Ok((input, Movement { kind, amount }))
}

pub fn solve(input: &str) -> String {
    let movements: Vec<Movement> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|l| parse_movement(l).unwrap())
        .map(|(_, instruction)| instruction)
        .collect();

    let mut ship = Ship {
        pos_x: 0,
        pos_y: 0,
        direction: Direction::East,
    };

    for mov in movements {
        ship.apply_movement(&mov);
    }

    let distance = i64::abs(ship.pos_x) + i64::abs(ship.pos_y);

    distance.to_string()
}
