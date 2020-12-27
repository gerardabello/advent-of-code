pub struct BoardingPass {
    row: u8,
    column: u8,
}

impl BoardingPass {
    pub fn parse(seat_str: &str) -> Self {
        let letters: Vec<char> = seat_str.chars().collect();

        assert_eq!(10, letters.len());

        let row_letters = &letters[..7];
        let column_letters = &letters[7..];

        assert_eq!(7, row_letters.len());
        assert_eq!(3, column_letters.len());

        let row = row_letters
            .iter()
            .map(|letter| match letter {
                'F' => 0,
                'B' => 1,
                n => panic!("Unknown letter {}", n),
            })
            .fold(0, |acc, x| acc * 2 + x);

        let column = column_letters
            .iter()
            .map(|letter| match letter {
                'L' => 0,
                'R' => 1,
                n => panic!("Unknown letter {}", n),
            })
            .fold(0, |acc, x| acc * 2 + x);

        BoardingPass { row, column }
    }

    pub fn id(&self) -> u32 {
        (self.row as u32) * 8 + (self.column as u32)
    }
}

pub fn solve(input: &str) -> String {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| BoardingPass::parse(s))
        .map(|bp| bp.id())
        .max()
        .unwrap()
        .to_string()
}
