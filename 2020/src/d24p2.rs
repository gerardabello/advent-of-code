use crate::d24p1::{dir_to_hex_coords, dirs_to_hex_coords, parse_input, Direction};

#[derive(Clone)]
struct Floor {
    tiles: Vec<Vec<bool>>,
    radius: usize,
}

impl Floor {
    fn new(radius: usize) -> Self {
        let vec_size = radius * 2 + 1;
        Self {
            radius,
            tiles: vec![vec![false; vec_size]; vec_size],
        }
    }

    fn tile_coord(&self, coord: (isize, isize)) -> (usize, usize) {
        if coord.0 < -(self.radius as isize) || coord.0 > self.radius as isize {
            panic!("invalid coordinates {:?}", coord);
        }
        if coord.1 < -(self.radius as isize) || coord.1 > self.radius as isize {
            panic!("invalid coordinates {:?}", coord);
        }

        (
            (coord.0 + self.radius as isize) as usize,
            (coord.1 + self.radius as isize) as usize,
        )
    }

    fn get_tile(&self, coord: (isize, isize)) -> bool {
        let tc = self.tile_coord(coord);

        self.tiles[tc.1][tc.0]
    }

    fn flip_tile(&mut self, coord: (isize, isize)) {
        let tc = self.tile_coord(coord);

        self.tiles[tc.1][tc.0] = !self.tiles[tc.1][tc.0]
    }

    fn n_black_tiles(&self) -> usize {
        self.tiles.iter().flatten().filter(|t| **t).count()
    }

    fn n_neighbour_black_tiles(&self, coord: (isize, isize)) -> usize {
        [
            Direction::East,
            Direction::West,
            Direction::SouthEast,
            Direction::NorthEast,
            Direction::SouthWest,
            Direction::NorthWest,
        ]
        .iter()
        .map(dir_to_hex_coords)
        .map(|(dx, dy)| self.get_tile((coord.0 + dx, coord.1 + dy)))
        .filter(|v| *v)
        .count()
    }

    fn do_flipping(&mut self) {
        let margin = 1;
        let freezed_clone = self.clone();

        for x in -(self.radius as isize) + margin..(self.radius as isize) + 1 - margin {
            for y in -(self.radius as isize) + margin..(self.radius as isize) + 1 - margin {
                let nb = freezed_clone.n_neighbour_black_tiles((x, y));
                let t = freezed_clone.get_tile((x, y));

                if t && (nb == 0 || nb > 2) {
                    self.flip_tile((x, y));
                }

                if !t && (nb == 2) {
                    self.flip_tile((x, y));
                }
            }
        }
    }
}

pub fn solve(input: &str) -> String {
    let (_, directions) = parse_input(input).unwrap();
    let coords: Vec<(isize, isize)> = directions
        .into_iter()
        .map(|dirs| dirs_to_hex_coords(&dirs))
        .collect();

    let mut floor = Floor::new(100);

    for (x, y) in coords {
        floor.flip_tile((x, y));
    }

    for _ in 1..101 {
        floor.do_flipping();
        //println!("Day {}: {}", day, floor.n_black_tiles());
    }

    floor.n_black_tiles().to_string()
}
