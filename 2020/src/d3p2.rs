struct Forest {
    trees: Vec<Vec<bool>>,
    rows: usize,
    columns: usize,
}

impl Forest {
    fn new(map: &str) -> Self {
        let mut columns = 0;
        let mut rows = 0;
        let mut trees: Vec<Vec<bool>> = Vec::new();
        for line in map.split('\n') {
            let mut row: Vec<bool> = Vec::new();
            for p in line.chars() {
                let val = match p {
                    '.' => false,
                    '#' => true,
                    c => panic!("Unexpected char {}", c),
                };
                row.push(val);
            }
            if !row.is_empty() {
                columns = row.len();
                trees.push(row);
                rows = trees.len();
            }
        }

        Forest {
            trees,
            columns,
            rows,
        }
    }

    fn has_tree(&self, pos: [u32; 2]) -> bool {
        let x = pos[0] % (self.columns as u32);

        self.trees[pos[1] as usize][x as usize]
    }

    fn height(&self) -> u32 {
        self.rows as u32
    }
}

fn count_trees_slope(forest: &Forest, slope: &[u32; 2]) -> u32 {
    let mut pos = [0, 0];

    let mut count = 0;

    loop {
        if forest.has_tree(pos) {
            count += 1;
        }

        pos[0] += slope[0];
        pos[1] += slope[1];

        if pos[1] >= forest.height() {
            return count;
        }
    }
}

pub fn solve(input: &str) -> String {
    let forest = Forest::new(&input);

    let slopes: Vec<[u32; 2]> = vec![[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];

    slopes
        .iter()
        .map(|slope| count_trees_slope(&forest, slope) as u64)
        .fold(1, |a, b| a * b)
        .to_string()
}
