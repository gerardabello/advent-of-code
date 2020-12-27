
const MARGIN: usize = 2; // add margin to prevent vector overflow

#[derive(PartialEq, Clone, Debug)]
enum State {
    Active,
    Inactive,
}

#[derive(PartialEq, Clone, Debug)]
struct Map {
    grid: Vec<Vec<Vec<State>>>,
    origin: (usize, usize, usize),
}

impl Map {
    fn print(&self, z: isize) {
        let start = (-(self.origin.2 as isize)) + MARGIN as isize;
        let end = ((self.grid.len() - self.origin.2) as isize) - MARGIN as isize;

        for y in start..end {
            for x in start..end {
                let state = self.state(x as isize, y as isize, z);
                let c = match state {
                    State::Active => "#",
                    State::Inactive => ".",
                };
                print!("{}", c);
            }
            println!();
        }
    }

    fn get_vec_position(&self, x: isize, y: isize, z: isize) -> (usize, usize, usize) {
        let o_x = self.origin.0 as isize + x;
        let o_y = self.origin.1 as isize + y;
        let o_z = self.origin.2 as isize + z;

        if o_z >= self.grid.len() as isize || o_z < 0 {
            unreachable!("invalid z cord");
        }

        if o_y >= self.grid[o_z as usize].len() as isize || o_y < 0 {
            unreachable!("invalid y cord");
        }

        if o_x >= self.grid[o_z as usize][o_y as usize].len() as isize || o_x < 0 {
            println!(
                "grid_size: {}, origin: {}, x: {}, o_x: {}",
                self.grid[0].len(),
                self.origin.0,
                x,
                o_x
            );
            unreachable!("invalid x cord");
        }

        (o_x as usize, o_y as usize, o_z as usize)
    }

    fn set_state(&mut self, x: isize, y: isize, z: isize, state: State) {
        let (o_x, o_y, o_z) = self.get_vec_position(x, y, z);

        self.grid[o_z][o_y][o_x] = state;
    }

    fn state(&self, x: isize, y: isize, z: isize) -> State {
        let (o_x, o_y, o_z) = self.get_vec_position(x, y, z);

        self.grid[o_z][o_y][o_x].clone()
    }

    fn sum_active(&self) -> u32 {
        self.grid
            .iter()
            .flatten()
            .flatten()
            .filter(|p| **p == State::Active)
            .count() as u32
    }

    fn sum_of_adjacent_actives(&self, x: isize, y: isize, z: isize) -> u8 {
        let positions: [State; 26] = [
            self.state(x - 1, y, z),
            self.state(x, y - 1, z),
            self.state(x - 1, y - 1, z),
            self.state(x + 1, y, z),
            self.state(x, y + 1, z),
            self.state(x + 1, y + 1, z),
            self.state(x + 1, y - 1, z),
            self.state(x - 1, y + 1, z),
            self.state(x, y, z - 1),
            self.state(x - 1, y, z - 1),
            self.state(x, y - 1, z - 1),
            self.state(x - 1, y - 1, z - 1),
            self.state(x + 1, y, z - 1),
            self.state(x, y + 1, z - 1),
            self.state(x + 1, y + 1, z - 1),
            self.state(x + 1, y - 1, z - 1),
            self.state(x - 1, y + 1, z - 1),
            self.state(x, y, z + 1),
            self.state(x - 1, y, z + 1),
            self.state(x, y - 1, z + 1),
            self.state(x - 1, y - 1, z + 1),
            self.state(x + 1, y, z + 1),
            self.state(x, y + 1, z + 1),
            self.state(x + 1, y + 1, z + 1),
            self.state(x + 1, y - 1, z + 1),
            self.state(x - 1, y + 1, z + 1),
        ];

        positions.iter().filter(|s| **s == State::Active).count() as u8
    }
}

fn iterate_map(map: &Map) -> Map {
    let mut new_map = map.clone();
    let start = (-(map.origin.2 as isize)) + MARGIN as isize;
    let end = ((map.grid.len() - map.origin.2) as isize) - MARGIN as isize;

    for z in start..end {
        for y in start..end {
            for x in start..end {
                let state = map.state(x, y, z);
                let n_active = map.sum_of_adjacent_actives(x, y, z);
                if state == State::Active {
                    if n_active != 2 && n_active != 3 {
                        new_map.set_state(x, y, z, State::Inactive);
                    }
                } else {
                    if n_active == 3 {
                        new_map.set_state(x, y, z, State::Active);
                    }
                }
            }
        }
    }

    new_map
}

fn parse_map(input: &str, size: usize) -> Map {
    let grid: Vec<Vec<Vec<State>>> = vec![vec![vec![State::Inactive; size]; size]; size];

    let mut map = Map {
        origin: (size / 2, size / 2, size / 2),
        grid,
    };

    for (y, row) in input.split('\n').enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                map.set_state(x as isize, y as isize, 0, State::Active);
            }
        }
    }

    map
}

pub fn solve(input: &str) -> String {
    let initial_map = parse_map(input, 100);

    let mut map = initial_map;

    let mut i = 0;
    loop {
        let new_map = iterate_map(&map);
        i += 1;

        println!();
        println!();
        new_map.print(0);

        if i == 6 {
            return new_map.sum_active().to_string();
        }

        map = new_map;
    }
}
