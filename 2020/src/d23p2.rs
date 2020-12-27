use nom::{bytes::complete::take, combinator::map_res, multi::many1, IResult};

const MAX_NUM: usize = 1_000_000;

type Cup = usize;

pub fn parse_input(input: &str) -> IResult<&str, Vec<Cup>> {
    many1(map_res(take(1 as usize), |s: &str| s.parse::<usize>()))(input)
}

struct CupRing(Vec<(usize, usize)>);

impl CupRing {
    fn new(cups: &[usize]) -> Self {
        // As there is no "0" cup, we need to add a 0 element that will be never used
        let mut ret = vec![(0, 0); cups.len() + 1];

        for w in cups.windows(3) {
            ret[w[1]] = (w[0], w[2]);
        }

        ret[cups[0]] = (cups[cups.len() - 1], cups[1]);
        ret[cups[cups.len() - 1]] = (cups[cups.len() - 2], cups[0]);

        Self(ret)
    }

    fn get_next(&self, cup: Cup) -> Cup {
        self.0[cup].1
    }

    fn set_next(&mut self, one: Cup, next: Cup) {
        self.0[one].1 = next;
        self.0[next].0 = one;
    }
}

struct Game {
    ring: CupRing,
    current: Cup,
}

impl Game {
    fn play_turn(&mut self) {
        let mut picked = Vec::new();
        picked.push(self.ring.0[self.current].1);
        picked.push(self.ring.0[picked[0]].1);
        picked.push(self.ring.0[picked[1]].1);

        let mut possible = self.current - 1;

        loop {
            if possible == 0 {
                possible = MAX_NUM;
            }

            if picked.contains(&possible) {
                possible -= 1;
            } else {
                break;
            }
        }

        let destination = possible;

        let destination_next = self.ring.get_next(destination);
        let picked_next = self.ring.get_next(picked[2]);

        self.ring.set_next(self.current, picked_next);

        self.ring.set_next(destination, picked[0]);
        self.ring.set_next(picked[2], destination_next);

        self.current = self.ring.get_next(self.current);
    }
}

pub fn solve(input: &str) -> String {
    let (_, mut cups) = parse_input(input).unwrap();

    for i in cups.len()..MAX_NUM as usize {
        cups.push(i + 1)
    }

    assert!(cups.len() == MAX_NUM as usize);

    let mut game = Game {
        ring: CupRing::new(&cups),
        current: cups[0],
    };

    for _ in 1..10_000_000 {
        game.play_turn();
    }

    (game.ring.get_next(1) * game.ring.get_next(game.ring.get_next(1))).to_string()
}
