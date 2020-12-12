#[derive(PartialEq, Clone, Debug)]
enum Position {
    Floor,
    Ocupied,
    Free,
}

#[derive(PartialEq, Clone, Debug)]
struct Map {
    grid: Vec<Vec<Position>>,
}

impl Map {
    fn print(&self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if let Some(pos) = self.position(x as isize, y as isize) {
                    let c = match pos {
                        Position::Free => "L",
                        Position::Ocupied => "#",
                        Position::Floor => ".",
                    };
                    print!("{}", c);
                }
            }
            println!();
        }
    }

    fn set_position(&mut self, x: usize, y: usize, pos: Position) {
        self.grid[y][x] = pos;
    }

    fn position(&self, x: isize, y: isize) -> Option<Position> {
        if y >= self.grid.len() as isize || y < 0 {
            return None;
        }

        if x >= self.grid[y as usize].len() as isize || x < 0 {
            return None;
        }

        Some(self.grid[y as usize][x as usize].clone())
    }

    fn sum_occupieds(&self) -> u32 {
        self.grid
            .iter()
            .flatten()
            .filter(|p| **p == Position::Ocupied)
            .count() as u32
    }

    fn next_seat_direction(&self, x: usize, y: usize, dir: (isize, isize)) -> Position {
        let mut px = x as isize;
        let mut py = y as isize;
        loop {
            px += dir.0;
            py += dir.1;

            let opos = self.position(px, py);

            if opos.is_none() {
                return Position::Floor;
            }

            let pos = opos.unwrap();

            if pos == Position::Floor {
                continue;
            }

            return pos;
        }
    }

    fn sum_of_visible_occupieds(&self, x: usize, y: usize) -> u8 {
        let directions: [(isize, isize); 8] = [
            (0, -1),
            (-1, 0),
            (-1, -1),
            (0, 1),
            (1, 0),
            (1, 1),
            (-1, 1),
            (1, -1),
        ];

        directions
            .iter()
            .map(|dir| self.next_seat_direction(x, y, *dir))
            .filter(|p| *p == Position::Ocupied)
            .count() as u8
    }
}

fn iterate_map(map: &Map) -> Map {
    let mut new_map = map.clone();

    for y in 0..map.grid.len() {
        for x in 0..map.grid[y].len() {
            if let Some(pos) = map.position(x as isize, y as isize) {
                if pos == Position::Ocupied && map.sum_of_visible_occupieds(x, y) >= 5 {
                    new_map.set_position(x, y, Position::Free);
                } else if pos == Position::Free && map.sum_of_visible_occupieds(x, y) == 0 {
                    new_map.set_position(x, y, Position::Ocupied);
                }
            }
        }
    }

    new_map
}

pub fn solve(input: &str) -> String {
    let initial_map = Map {
        grid: input
            .split('\n')
            .filter(|row| !row.is_empty())
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        'L' => Position::Free,
                        '.' => Position::Floor,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    };

    let mut map = initial_map;
    loop {
        let new_map = iterate_map(&map);

        new_map.print();

        if new_map == map {
            return new_map.sum_occupieds().to_string();
        }

        map = new_map;
    }
}
