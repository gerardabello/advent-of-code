use nom::{multi::many1, IResult};
use std::cmp::Ordering;

use std::collections::BinaryHeap;

use crate::matrix::{get_xy, neighbours};
use crate::parsers::{full, lines, single_digit};

pub fn parse_line(input: &str) -> IResult<&str, Vec<usize>> {
    many1(single_digit)(input)
}

pub fn path_cost(map: &[Vec<usize>], path: &[(usize, usize)]) -> usize {
    path.iter()
        .skip(1)
        .map(|(x, y)| get_xy(map, *x, *y).unwrap())
        .sum()
}

#[allow(dead_code)]
pub fn manhattan_distance_to_end(map: &[Vec<usize>], path: &[(usize, usize)]) -> usize {
    let height = map.len();
    let width = map[0].len();

    let last = path.last().unwrap();

    (width - last.0) + (height - last.1)
}

pub fn heuristic(map: &[Vec<usize>], path: &[(usize, usize)]) -> usize {
    path_cost(map, path) // + manhattan_distance_to_end(map, path)
}

#[derive(Clone, Eq, PartialEq)]
struct Path {
    points: Vec<(usize, usize)>,
    cost: usize,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare distance from (0,0) - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost).then_with(|| {
            let last_self = self.points.last().unwrap_or(&(0, 0));
            let last_other = other.points.last().unwrap_or(&(0, 0));

            let self_distance = last_self.0 + last_self.1;
            let other_distance = last_other.0 + last_other.1;

            self_distance.cmp(&other_distance)
        })
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn find_shortest_path(map: &[Vec<usize>]) -> usize {
    let height = map.len();
    let width = map[0].len();

    let mut visited = vec![(0, 0)];

    let mut paths = BinaryHeap::new();
    paths.push(Path {
        points: vec![(0, 0)],
        cost: 0,
    });
    loop {
        let path_to_grow = paths.pop().unwrap();

        //print_with_highlights(map, |x, y, _| path_to_grow.points.contains(&(x, y)));

        let grow_from = path_to_grow.points.last().unwrap();

        let possible_points = neighbours(map, grow_from.0, grow_from.1).map(|(_, x, y)| (x, y));

        let new_points: Vec<(usize, usize)> = possible_points
            .filter(|(x, y)| !visited.contains(&(*x, *y)))
            .collect();

        let new_paths: Vec<Vec<(usize, usize)>> = new_points
            .into_iter()
            .map(|(x, y)| {
                visited.push((x, y));
                path_to_grow
                    .points
                    .iter()
                    .cloned()
                    .chain([(x, y)])
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();

        for path in new_paths {
            if *(path.last().unwrap()) == (width - 1, height - 1) {
                return path_cost(map, &path);
            }

            let cost = heuristic(map, &path);
            paths.push(Path { points: path, cost });
        }
    }
}

pub fn solve(input: &str) -> usize {
    let (_, map) = full(lines(parse_line))(input).unwrap();

    find_shortest_path(&map)
}
