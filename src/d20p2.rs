use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

use crate::d20p1::{
    rotate_coords, flip_horizontal, flip_vertical,
    extract_borders, extract_bottom_border, extract_left_border, extract_right_border,
    extract_top_border, find_edges, flip_image_size_array, parse_input, ImageData, ImagePiece,
    Rotation, IMAGE_PIECE_SIZE,
};

type Border = [bool; IMAGE_PIECE_SIZE];

fn merge_vecs<T: Clone>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T> {
    a.append(&mut b);

    a
}

#[derive(Clone)]
pub struct Image {
    width: usize,
    pieces: Vec<Option<ImagePiece>>,
}

impl Image {
    fn new(width: usize) -> Self {
        Image {
            width,
            pieces: vec![None; width * width],
        }
    }

    fn form_complete_image(&self, datas: &[ImageData]) -> Vec<Vec<bool>> {
        let no_border_width = IMAGE_PIECE_SIZE - 2;
        let mut ret = vec![vec![false; self.width * no_border_width]; self.width * no_border_width];

        for ix in 0..self.width {
            for iy in 0..self.width {
                let piece = self.get_piece_at_pos(ix, iy).unwrap();
                let data = datas.iter().find(|d| d.id == piece.id).unwrap();
                for x in 0..no_border_width {
                    for y in 0..no_border_width {
                        let d = piece.get_data_at_pos(data, 1 + x, 1 + y).unwrap();

                        ret[iy * no_border_width + y][ix * no_border_width + x] = d;
                    }
                }
            }
        }
        ret
    }

    fn get_neiguhouring_borders(&self, x: usize, y: usize, datas: &[ImageData]) -> Vec<Border> {
        let mut ret = Vec::new();

        let top_neighbour = self.get_top_neighbour(x, y);

        if let Some(n) = top_neighbour {
            ret.push(extract_bottom_border(
                n,
                datas.iter().find(|d| d.id == n.id).unwrap(),
            ));
        }

        let left_neighbour = self.get_left_neighbour(x, y);

        if let Some(n) = left_neighbour {
            ret.push(extract_right_border(
                n,
                datas.iter().find(|d| d.id == n.id).unwrap(),
            ));
        }

        let bottom_neighbout = self.get_bottom_neighbour(x, y);

        if let Some(n) = bottom_neighbout {
            ret.push(extract_top_border(
                n,
                datas.iter().find(|d| d.id == n.id).unwrap(),
            ));
        }

        let right_neighbour = self.get_right_neighbour(x, y);

        if let Some(n) = right_neighbour {
            ret.push(extract_left_border(
                n,
                datas.iter().find(|d| d.id == n.id).unwrap(),
            ));
        }

        ret
    }

    fn can_add_piece_at(
        &self,
        x: usize,
        y: usize,
        piece: &ImagePiece,
        datas: &[ImageData],
    ) -> bool {
        let top_neighbour = self.get_top_neighbour(x, y);

        if let Some(n) = top_neighbour {
            if !self.compare_pieces_vertically(n, piece, datas) {
                return false;
            }
        }

        let left_neighbour = self.get_left_neighbour(x, y);

        if let Some(n) = left_neighbour {
            if !self.compare_pieces_horizontally(n, piece, datas) {
                return false;
            }
        }

        let bottom_neighbout = self.get_bottom_neighbour(x, y);

        if let Some(n) = bottom_neighbout {
            if !self.compare_pieces_vertically(piece, n, datas) {
                return false;
            }
        }

        let right_neighbour = self.get_right_neighbour(x, y);

        if let Some(n) = right_neighbour {
            if !self.compare_pieces_horizontally(piece, n, datas) {
                return false;
            }
        }

        true
    }

    fn compare_pieces_horizontally(
        &self,
        left: &ImagePiece,
        right: &ImagePiece,
        datas: &[ImageData],
    ) -> bool {
        for y in 0..IMAGE_PIECE_SIZE {
            if left.get_data_at_pos(
                datas.iter().find(|d| d.id == left.id).unwrap(),
                IMAGE_PIECE_SIZE - 1,
                y,
            ) != right.get_data_at_pos(datas.iter().find(|d| d.id == right.id).unwrap(), 0, y)
            {
                return false;
            }
        }

        true
    }

    fn compare_pieces_vertically(
        &self,
        top: &ImagePiece,
        bottom: &ImagePiece,
        datas: &[ImageData],
    ) -> bool {
        for x in 0..IMAGE_PIECE_SIZE {
            if top.get_data_at_pos(
                datas.iter().find(|d| d.id == top.id).unwrap(),
                x,
                IMAGE_PIECE_SIZE - 1,
            ) != bottom.get_data_at_pos(datas.iter().find(|d| d.id == bottom.id).unwrap(), x, 0)
            {
                return false;
            }
        }

        true
    }

    fn set_piece_at_pos(&mut self, x: usize, y: usize, piece: ImagePiece) {
        if x >= self.width || y >= self.width {
            panic!("wrong indexes at set_piece_at_pos");
        }

        let index = y * self.width + x;
        self.pieces[index] = Some(piece);
    }

    fn get_piece_at_pos(&self, x: usize, y: usize) -> Option<&ImagePiece> {
        if x >= self.width || y >= self.width {
            return None;
        }

        let index = y * self.width + x;
        match self.pieces.get(index).unwrap() {
            Some(p) => Some(p),
            None => None,
        }
    }

    fn get_top_neighbour(&self, x: usize, y: usize) -> Option<&ImagePiece> {
        if y == 0 {
            return None;
        }

        self.get_piece_at_pos(x, y - 1)
    }

    fn get_left_neighbour(&self, x: usize, y: usize) -> Option<&ImagePiece> {
        if x == 0 {
            return None;
        }

        self.get_piece_at_pos(x - 1, y)
    }

    fn get_bottom_neighbour(&self, x: usize, y: usize) -> Option<&ImagePiece> {
        if y == self.width - 1 {
            return None;
        }

        self.get_piece_at_pos(x, y + 1)
    }

    fn get_right_neighbour(&self, x: usize, y: usize) -> Option<&ImagePiece> {
        if x == self.width - 1 {
            return None;
        }

        self.get_piece_at_pos(x + 1, y)
    }
}

pub fn next_pos(px: usize, py: usize, width: usize) -> Option<(usize, usize)> {
    let i = py * width + px;
    let ni = i + 1;

    if ni >= width * width {
        return None;
    }

    Some((ni % width, ni / width))
}

fn pieces_matching_borders(borders: &[Border], hm: &HashMap<Border, Vec<u32>>) -> HashSet<u32> {
    let mut hs: HashSet<u32> = HashSet::new();

    for border in borders {
        let ids = &hm[border];
        for id in ids {
            hs.insert(*id);
        }
    }

    hs
}

pub fn solveit(
    image: &Image,
    all_pieces: &[ImagePiece],
    datas: &[ImageData],
    hm: &HashMap<Border, Vec<u32>>,
    used_images: &[u32],
    px: usize,
    py: usize,
    order: usize,
) -> Option<Image> {
    let neighbour_borders = image.get_neiguhouring_borders(px, py, datas);
    let mut pieces_to_find: Vec<u32> = pieces_matching_borders(&neighbour_borders, hm)
        .into_iter()
        .collect();

    if px == 0 && py == 0 {
        pieces_to_find = find_edges(all_pieces, datas, true)
            .iter()
            .map(|p| p.id)
            .collect();
    }

    for id in pieces_to_find {
        let piece = all_pieces.iter().find(|p| p.id == id).unwrap();
        if used_images.contains(&piece.id) {
            continue;
        }

        for (flip_v, flip_h) in &[(false, false), (true, false), (false, true)] {
            for rot in &[
                Rotation::Deg0,
                Rotation::Deg90,
                Rotation::Deg180,
                Rotation::Deg270,
            ] {
                let mut pc = (*piece).clone();

                pc.rotation = rot.clone();
                pc.flip_h = *flip_h;
                pc.flip_v = *flip_v;
                if image.can_add_piece_at(px, py, &pc, datas) {
                    let mut ic = image.clone();
                    ic.set_piece_at_pos(px, py, pc);
                    let mut new_used: Vec<u32> = used_images.to_vec();
                    new_used.push(piece.id);
                    match next_pos(px, py, image.width) {
                        Some((nx, ny)) => {
                            match solveit(&ic, all_pieces, datas, hm, &new_used, nx, ny, order + 1)
                            {
                                Some(im) => return Some(im),
                                None => continue,
                            }
                        }
                        None => {
                            return Some(ic);
                        }
                    };
                }
            }
        }
    }

    None
}

fn has_monster_at(image: &[Vec<bool>], px: usize, py: usize, rot: &Rotation, flip_v: bool, flip_h: bool) -> bool {
    [
        (0, 1),
        (1, 2),
        (4, 2),
        (5, 1),
        (6, 1),
        (7, 2),
        (10, 2),
        (11, 1),
        (12, 1),
        (13, 2),
        (16, 2),
        (17, 1),
        (18, 1),
        (18, 0),
        (19, 1),
    ]
    .iter()
    .all(|(x, y)| {
        let (fx,fy) = flip_horizontal(image.len(),flip_vertical(image.len(),rotate_coords(image.len(),(px + *x,py + *y),rot), flip_v), flip_h);

        return match image.get(fy).and_then(|i| i.get(fx)) {
            Some(b) => *b,
            None => false,
        };
    })
}

pub fn solve(input: &str) -> String {
    let (_, datas) = parse_input(input).unwrap();

    let pieces = datas
        .iter()
        .map(|d| ImagePiece {
            id: d.id,
            rotation: Rotation::Deg0,
            flip_h: false,
            flip_v: false,
        })
        .collect::<Vec<_>>();

    // We asume the image is square
    let image_width: usize = f64::sqrt(pieces.len() as f64) as usize;

    let mut hm: HashMap<[bool; IMAGE_PIECE_SIZE], Vec<u32>> = HashMap::new();

    for piece in &pieces {
        let borders: [[bool; IMAGE_PIECE_SIZE]; 4] =
            extract_borders(&piece, datas.iter().find(|d| d.id == piece.id).unwrap());

        for border in borders.iter() {
            match hm.clone().get(border) {
                Some(v) => hm.insert(*border, merge_vecs(v.clone(), vec![piece.id])),
                None => hm.insert(*border, vec![piece.id]),
            };

            match hm.clone().get(&flip_image_size_array(border)) {
                Some(v) => hm.insert(
                    flip_image_size_array(border),
                    merge_vecs(v.clone(), vec![piece.id]),
                ),
                None => hm.insert(flip_image_size_array(border), vec![piece.id]),
            };
        }
    }
    let solution = solveit(&Image::new(image_width), &pieces, &datas, &hm, &[], 0, 0, 0).unwrap();

    let image = solution.form_complete_image(&datas);

    let mut n_monsters = 0;

    'outer: for (flip_v, flip_h) in &[(false, false), (true, false), (false, true)] {
        for rot in &[
            Rotation::Deg0,
            Rotation::Deg90,
            Rotation::Deg180,
            Rotation::Deg270,
        ] {
            n_monsters = 0;

            for x in 0..image.len() {
                for y in 0..image[x].len() {
                    if has_monster_at(&image, x, y, rot, *flip_v, *flip_h) {
                        n_monsters += 1;
                    }
                }
            }

            if n_monsters != 0 {
                break 'outer;
            }
        }
    }


    let n_rough = image.into_iter().flatten().filter(|b| *b).count() - 15 * n_monsters;

    n_rough.to_string()
}
