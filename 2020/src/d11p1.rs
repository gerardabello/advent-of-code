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
        print!("\n");
        print!("\n");
        print!("\n");
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
        print!("\n");
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

    fn sum_of_adjacent_occupieds(&self, x: usize, y: usize) -> u8 {
        let positions: [Option<Position>; 8] = [
            self.position(x as isize - 1, y as isize),
            self.position(x as isize, y as isize - 1),
            self.position(x as isize - 1, y as isize - 1),
            self.position(x as isize + 1, y as isize),
            self.position(x as isize, y as isize + 1),
            self.position(x as isize + 1, y as isize + 1),
            self.position(x as isize + 1, y as isize - 1),
            self.position(x as isize - 1, y as isize + 1),
        ];

        positions
            .iter()
            .filter(|o| o.is_some())
            .cloned()
            .map(|o| o.unwrap())
            .filter(|p| *p == Position::Ocupied)
            .count() as u8
    }
}

fn iterate_map(map: &Map) -> Map {
    let mut new_map = map.clone();

    for y in 0..map.grid.len() {
        for x in 0..map.grid[y].len() {
            if let Some(pos) = map.position(x as isize, y as isize) {
                if pos == Position::Ocupied && map.sum_of_adjacent_occupieds(x, y) >= 4 {
                    new_map.set_position(x, y, Position::Free);
                } else if pos == Position::Free && map.sum_of_adjacent_occupieds(x, y) == 0 {
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
