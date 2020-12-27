const MARGIN: usize = 2; // add margin to prevent vector overflow

#[derive(PartialEq, Clone, Debug)]
enum State {
    Active,
    Inactive,
}

#[derive(PartialEq, Clone, Debug)]
struct Map {
    grid: Vec<Vec<Vec<Vec<State>>>>,
    origin: (usize, usize, usize, usize),
}

impl Map {
    fn print(&self, z: isize, w: isize) {
        let start = (-(self.origin.2 as isize)) + MARGIN as isize;
        let end = ((self.grid.len() - self.origin.2) as isize) - MARGIN as isize;

        for y in start..end {
            for x in start..end {
                let state = self.state(x as isize, y as isize, z, w);
                let c = match state {
                    State::Active => "#",
                    State::Inactive => ".",
                };
                print!("{}", c);
            }
            println!();
        }
    }

    fn get_vec_position(&self, x: isize, y: isize, z: isize, w:isize) -> (usize, usize, usize, usize) {
        let o_x = self.origin.0 as isize + x;
        let o_y = self.origin.1 as isize + y;
        let o_z = self.origin.2 as isize + z;
        let o_w = self.origin.3 as isize + w;

        let max = self.grid.len() as isize; // all coords have the same size

        if o_w >= max || o_w < 0 {
            unreachable!("invalid w cord");
        }

        if o_z >= max || o_z < 0 {
            unreachable!("invalid z cord");
        }

        if o_y >= max || o_y < 0 {
            unreachable!("invalid y cord");
        }

        if o_x >= max || o_x < 0 {
            unreachable!("invalid x cord");
        }

        (o_x as usize, o_y as usize, o_z as usize, o_w as usize)
    }

    fn set_state(&mut self, x: isize, y: isize, z: isize, w: isize, state: State) {
        let (o_x, o_y, o_z, o_w) = self.get_vec_position(x, y, z, w);

        self.grid[o_w][o_z][o_y][o_x] = state;
    }

    fn state(&self, x: isize, y: isize, z: isize, w:isize) -> State {
        let (o_x, o_y, o_z, o_w) = self.get_vec_position(x, y, z, w);

        self.grid[o_w][o_z][o_y][o_x].clone()
    }

    fn sum_active(&self) -> u32 {
        self.grid
            .iter()
            .flatten()
            .flatten()
            .flatten()
            .filter(|p| **p == State::Active)
            .count() as u32
    }

    fn sum_of_adjacent_actives(&self, x: isize, y: isize, z: isize, w:isize) -> u8 {
        let positions: [State; 80] = [
            self.state(x - 1, y, z,w),
            self.state(x, y - 1, z,w),
            self.state(x - 1, y - 1, z,w),
            self.state(x + 1, y, z,w),
            self.state(x, y + 1, z,w),
            self.state(x + 1, y + 1, z,w),
            self.state(x + 1, y - 1, z,w),
            self.state(x - 1, y + 1, z,w),
            self.state(x, y, z - 1,w),
            self.state(x - 1, y, z - 1,w),
            self.state(x, y - 1, z - 1,w),
            self.state(x - 1, y - 1, z - 1,w),
            self.state(x + 1, y, z - 1,w),
            self.state(x, y + 1, z - 1,w),
            self.state(x + 1, y + 1, z - 1,w),
            self.state(x + 1, y - 1, z - 1,w),
            self.state(x - 1, y + 1, z - 1,w),
            self.state(x, y, z + 1,w),
            self.state(x - 1, y, z + 1,w),
            self.state(x, y - 1, z + 1,w),
            self.state(x - 1, y - 1, z + 1,w),
            self.state(x + 1, y, z + 1,w),
            self.state(x, y + 1, z + 1,w),
            self.state(x + 1, y + 1, z + 1,w),
            self.state(x + 1, y - 1, z + 1,w),
            self.state(x - 1, y + 1, z + 1,w),

            self.state(x, y, z ,w+1),
            self.state(x - 1, y, z,w+1),
            self.state(x, y - 1, z,w+1),
            self.state(x - 1, y - 1, z,w+1),
            self.state(x + 1, y, z,w+1),
            self.state(x, y + 1, z,w+1),
            self.state(x + 1, y + 1, z,w+1),
            self.state(x + 1, y - 1, z,w+1),
            self.state(x - 1, y + 1, z,w+1),
            self.state(x, y, z - 1,w+1),
            self.state(x - 1, y, z - 1,w+1),
            self.state(x, y - 1, z - 1,w+1),
            self.state(x - 1, y - 1, z - 1,w+1),
            self.state(x + 1, y, z - 1,w+1),
            self.state(x, y + 1, z - 1,w+1),
            self.state(x + 1, y + 1, z - 1,w+1),
            self.state(x + 1, y - 1, z - 1,w+1),
            self.state(x - 1, y + 1, z - 1,w+1),
            self.state(x, y, z + 1,w+1),
            self.state(x - 1, y, z + 1,w+1),
            self.state(x, y - 1, z + 1,w+1),
            self.state(x - 1, y - 1, z + 1,w+1),
            self.state(x + 1, y, z + 1,w+1),
            self.state(x, y + 1, z + 1,w+1),
            self.state(x + 1, y + 1, z + 1,w+1),
            self.state(x + 1, y - 1, z + 1,w+1),
            self.state(x - 1, y + 1, z + 1,w+1),

            self.state(x, y, z ,w-1),
            self.state(x - 1, y, z,w-1),
            self.state(x, y - 1, z,w-1),
            self.state(x - 1, y - 1, z,w-1),
            self.state(x + 1, y, z,w-1),
            self.state(x, y + 1, z,w-1),
            self.state(x + 1, y + 1, z,w-1),
            self.state(x + 1, y - 1, z,w-1),
            self.state(x - 1, y + 1, z,w-1),
            self.state(x, y, z - 1,w-1),
            self.state(x - 1, y, z - 1,w-1),
            self.state(x, y - 1, z - 1,w-1),
            self.state(x - 1, y - 1, z - 1,w-1),
            self.state(x + 1, y, z - 1,w-1),
            self.state(x, y + 1, z - 1,w-1),
            self.state(x + 1, y + 1, z - 1,w-1),
            self.state(x + 1, y - 1, z - 1,w-1),
            self.state(x - 1, y + 1, z - 1,w-1),
            self.state(x, y, z + 1,w-1),
            self.state(x - 1, y, z + 1,w-1),
            self.state(x, y - 1, z + 1,w-1),
            self.state(x - 1, y - 1, z + 1,w-1),
            self.state(x + 1, y, z + 1,w-1),
            self.state(x, y + 1, z + 1,w-1),
            self.state(x + 1, y + 1, z + 1,w-1),
            self.state(x + 1, y - 1, z + 1,w-1),
            self.state(x - 1, y + 1, z + 1,w-1),
        ];

        positions.iter().filter(|s| **s == State::Active).count() as u8
    }
}

fn iterate_map(map: &Map) -> Map {
    let mut new_map = map.clone();
    let start = (-(map.origin.2 as isize)) + MARGIN as isize;
    let end = ((map.grid.len() - map.origin.2) as isize) - MARGIN as isize;

    for w in start..end {
    for z in start..end {
        for y in start..end {
            for x in start..end {
                let state = map.state(x, y, z, w);
                let n_active = map.sum_of_adjacent_actives(x, y, z, w);
                if state == State::Active {
                    if n_active != 2 && n_active != 3 {
                        new_map.set_state(x, y, z, w, State::Inactive);
                    }
                } else {
                    if n_active == 3 {
                        new_map.set_state(x, y, z, w, State::Active);
                    }
                }
            }
        }
    }
    }

    new_map
}

fn parse_map(input: &str, size: usize) -> Map {
    let grid: Vec<Vec<Vec<Vec<State>>>> = vec![vec![vec![vec![State::Inactive; size]; size]; size];size];

    let mut map = Map {
        origin: (size / 2, size / 2, size / 2, size/ 2),
        grid,
    };

    for (y, row) in input.split('\n').enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                map.set_state(x as isize, y as isize, 0,0,  State::Active);
            }
        }
    }

    map
}

pub fn solve(input: &str) -> String {
    let initial_map = parse_map(input, 40);

    let mut map = initial_map;

    let mut i = 0;
    loop {
        let new_map = iterate_map(&map);
        i += 1;

        println!();
        println!();
        new_map.print(2, 0);

        if i == 6 {
            return new_map.sum_active().to_string();
        }

        map = new_map;
    }
}
